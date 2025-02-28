use crate::bound::Bound;
use crate::config::Config;
use crate::eval::Eval;
use crate::moves::DEFAULT_MOVE;
use crate::nnue::NNUEState;
use crate::nnue::OFF;
use crate::nnue::ON;
use crate::principal_variation::PvTable;
use crate::search::search_info::SearchInfo;
use crate::search::search_params::SearchParams;
use crate::time_control::time_controller::TimeController;
use crate::transposition::{TranspositionTable, TranspositionTableEntry};
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{
    CastlingMode, CastlingSide, Chess, EnPassantMode, Move, MoveList, Piece, Position, Square,
};


pub struct Search {
    pub game: Chess,
    pub params: SearchParams,
    pub info: SearchInfo,
    pub time_controller: TimeController,
    pub nnue_state: NNUEState,
    eval: Eval,
    iteration_move: Move,
    transposition_table: TranspositionTable,
    history: Vec<Zobrist64>,
    config: Config,
    pv_table: PvTable,
    killer_moves: Vec<Vec<Option<Move>>>,
}


impl Search {
    pub fn make_move_nnue(&mut self, pos: &mut Chess, m: &Move) {
        self.nnue_state.push();
        let turn = pos.turn();
        let board = pos.board();

        match m {
            Move::EnPassant { from, to } => {
                let ep_target = Square::from_coords(to.file(), from.rank()); // captured pawn

                self.nnue_state
                    .manual_update::<OFF>((!pos.turn()).pawn(), ep_target);
            }

            Move::Castle { king, rook } => {
                let side = CastlingSide::from_queen_side(rook < king);
                let rook_target = Square::from_coords(side.rook_to_file(), rook.rank());

                self.nnue_state.move_update(turn.rook(), *rook, rook_target);
            }

            Move::Normal {
                role,
                from,
                capture: _capture,
                to,
                promotion,
            } => {
                if m.is_capture() {
                    let target_piece = board.piece_at(*to).unwrap();
                    let target_square = *to;

                    self.nnue_state
                        .manual_update::<OFF>(target_piece, target_square);
                }

                let piece = Piece {
                    color: turn,
                    role: *role,
                };

                if m.is_promotion() {
                    let promoted_piece = Piece {
                        color: turn,
                        role: promotion.unwrap(),
                    };

                    self.nnue_state.manual_update::<OFF>(piece, *from);
                    self.nnue_state.manual_update::<ON>(promoted_piece, *to);
                } else {
                    self.nnue_state.move_update(piece, *from, *to);
                }
            }

            _ => {}
        }

        pos.play_unchecked(m);
        self.history.push(pos.zobrist_hash(EnPassantMode::Legal));
    }

    #[allow(dead_code)]
    pub fn make_move(&mut self, pos: &mut Chess, m: &Move) {
        pos.play_unchecked(m);
        self.history.push(pos.zobrist_hash(EnPassantMode::Legal));
        self.nnue_state.refresh(pos.board());
    }

    pub fn undo_move_nnue(&mut self) {
        self.nnue_state.pop();
        self.history.pop();
    }

    #[allow(dead_code)]
    pub fn undo_move(&mut self) {
        self.history.pop();
    }

    pub fn go(&mut self) {
        self.time_controller.setup(&self.params, &self.game);
        self.info.nodes = 0;
        self.transposition_table.new_search();

        let mut best_move = DEFAULT_MOVE.clone();

        /* Iterative deepening */
        for current_depth in 0..self.params.depth {
            self.info.depth = current_depth + 1;
            let pos = self.game.clone();
            let iteration_score = self.negamax(&pos, self.info.depth, -100000, 100000, 0);

            if self.time_controller.is_time_up() {
                break;
            }

            best_move = self.pv_table.get_best_move();

            let elapsed = self.time_controller.elapsed();
            let pv = self.pv_table.collect();

            println!(
                "info depth {} nodes {} nps {} score cp {} time {} pv {}",
                self.info.depth,
                self.info.nodes,
                self.info.nodes as u128 * 1000 / (elapsed + 1),
                iteration_score,
                elapsed,
                pv.join(" ")
            );
        }

        println!("bestmove {}", best_move.to_uci(CastlingMode::Standard));
    }

    fn negamax(&mut self, pos: &Chess, depth: u8, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        self.pv_table.update_length(ply);

        if self.time_controller.is_time_up() {
            self.info.nodes += 1;
            return 0;
        }

        if depth == 0 {
            return self.quiesce(pos, alpha, beta, self.config.qsearch_depth);
        }

        let is_root = ply == 0;
        let position_key = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal);
        let entry = self.transposition_table.probe(position_key);

        /* Transposition Table Cut-offs */
        if entry.key == position_key
            && entry.generation == self.transposition_table.generation
            && !is_root
            && entry.depth >= depth
            && (entry.bound == Bound::Exact
                || (entry.bound == Bound::Alpha && entry.score <= alpha)
                || (entry.bound == Bound::Beta && entry.score >= beta))
        {
            self.info.nodes += 1;
            return entry.score;
        }

        let is_pv = beta - alpha != 1;
        let static_eval = self.eval.nnue_eval(&self.nnue_state, pos);

        /* Reverse Futility Pruning */
        if !is_pv && depth <= self.config.rfp_depth && !pos.is_check() {
            let score = static_eval - self.config.rfp_depth_multiplier * depth as i32;
            if score >= beta {
                self.info.nodes += 1;
                return static_eval;
            }
        }

        /* Threefold Repetition Detection */
        if self.history.iter().filter(|&x| x == &position_key).count() >= 2 {
            self.info.nodes += 1;
            return 0;
        }

        let moves = pos.legal_moves();

        /* Checkmate/Draw Detection */
        if moves.len() == 0 {
            self.info.nodes += 1;
            return match pos.is_checkmate() {
                true => -100000 + ply as i32,
                false => 0,
            };
        }

        let start_alpha = alpha;
        let mut best_score = -100000;
        let ordered_moves = self.order_moves(entry, ply, moves);
        let mut best_move = &ordered_moves[0];

        for i in 0..ordered_moves.len() {
            let m = &ordered_moves[i];
            let mut pos = pos.clone();
            self.make_move_nnue(&mut pos, m);

            let mut score: i32;

            /* Principal Variation Search */
            match i {
                0 => score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1),
                _ => {
                    score = -self.negamax(&pos, depth - 1, -alpha - 1, -alpha, ply + 1);

                    if score > alpha && beta - alpha > 1 {
                        score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1);
                    }
                }
            }

            self.undo_move_nnue();

            if score > best_score {
                best_score = score;
                best_move = m;

                if is_root {
                    self.iteration_move = best_move.clone();
                }
                if best_score > alpha {
                    // Stocking PV search Line
                    self.pv_table.store(ply, best_move.clone());
                    alpha = best_score;

                    if alpha >= beta {
                        if ply < self.config.max_depth_killer_moves {
                            self.add_killer_move(ply, m.clone());
                        }
                        break;
                    }
                }
            }
        }

        let bound = match best_score {
            score if score <= start_alpha => Bound::Alpha,
            score if score >= beta => Bound::Beta,
            _ => Bound::Exact,
        };

        self.transposition_table
            .store(position_key, depth, best_score, bound, best_move.clone());

        self.info.nodes += 1;
        best_score
    }

    fn quiesce(&mut self, pos: &Chess, mut alpha: i32, beta: i32, limit: u8) -> i32 {
        self.info.nodes += 1;
        let stand_pat = self.eval.nnue_eval(&self.nnue_state, pos);

        if limit == 0 {
            return stand_pat;
        }
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let moves = pos.capture_moves();

        for m in moves {
            let mut pos = pos.clone();
            self.make_move_nnue(&mut pos, &m);
            let score = -self.quiesce(&pos, -beta, -alpha, limit - 1);
            self.undo_move_nnue();

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn move_importance(&self, entry: &TranspositionTableEntry,  ply: usize, m: &Move) -> i32 {
        match m {
            m if m == &entry._move => self.config.mo_tt_entry_value,
            m if m.is_capture() => {
                let piece_value = m.role() as i32;
                let capture_value = m.capture().unwrap() as i32;

                self.config.mo_capture_value * capture_value - piece_value
            }
            m if self.killer_moves[ply].contains(&Some(m.clone())) => self.config.mo_killer_move_value,
            m if m.is_promotion() => m.promotion().unwrap() as i32,
            _ => 0,
        }
    }

    fn order_moves(&self, entry: TranspositionTableEntry, ply: usize,  mut moves: MoveList) -> MoveList {
        moves.sort_by(|a, b| {
            let a_score = self.move_importance(&entry, ply, &a);
            let b_score = self.move_importance(&entry, ply, &b);

            b_score.cmp(&a_score)
        });

        moves
    }
    fn add_killer_move(&mut self, ply: usize, m: Move) {
        if ply < self.config.max_depth_killer_moves {
            let killers = &mut self.killer_moves[ply];
            if !killers.contains(&Some(m.clone())) {
                killers.pop();
                killers.insert(0, Some(m));
            }
        }
    }
}

impl Default for Search {
    fn default() -> Self {
        let config = Config::load().unwrap();
        let params = SearchParams::default();
        let killer_moves = vec![vec![None; config.nb_killer_moves]; config.max_depth_killer_moves];
        Search {
            game: Chess::default(),
            eval: Eval::default(),
            iteration_move: DEFAULT_MOVE.clone(),
            transposition_table: TranspositionTable::new(config.tt_size),
            params,
            info: SearchInfo::default(),
            time_controller: TimeController::default(),
            history: Vec::new(),
            pv_table: PvTable::default(),
            nnue_state: *NNUEState::from_board(Chess::default().board()),
            config,
            killer_moves,
        }
    }
}
