use crate::bound::Bound;
use crate::eval::Eval;
use crate::moves::DEFAULT_MOVE;
use crate::transposition::{TranspositionTable, TranspositionTableEntry};
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{CastlingMode, Chess, EnPassantMode, Move, MoveList, Position};
use crate::search::search_info::SearchInfo;
use crate::search::search_params::SearchParams;
use crate::time_control::time_controller::TimeController;

pub struct Search {
    pub game: Chess,
    pub params: SearchParams,
    pub info: SearchInfo,
    pub time_controller: TimeController,
    eval: Eval,
    iteration_move: Move,
    transposition_table: TranspositionTable,
}


impl Search {
    pub fn go(&mut self) {
        self.time_controller.setup(&self.params, &self.game);

        self.info.nodes = 0;
        self.transposition_table.new_search();

        let mut best_move = DEFAULT_MOVE.clone();

        for current_depth in 0..self.params.depth {
            let pos = self.game.clone();
            let iteration_score = self.negamax(&pos, current_depth + 1, -100000, 100000, 0);

            if self.time_controller.is_time_up() { break; }

            best_move = self.iteration_move.clone();
            let elapsed = self.time_controller.elapsed();

            println!(
                "info depth {} nodes {} nps {} score cp {} time {} pv {}",
                current_depth + 1,
                self.info.nodes,
                self.info.nodes as u128 * 1000 / (elapsed + 1),
                iteration_score,
                elapsed,
                best_move.to_uci(CastlingMode::Standard)
            );
        }

        println!("bestmove {}", best_move.to_uci(CastlingMode::Standard));
    }

    fn negamax(&mut self, pos: &Chess, depth: u8, mut alpha: i32, beta: i32, ply: u32) -> i32 {
        if self.time_controller.is_time_up() {
            self.info.nodes += 1;
            return 0;
        }

        if depth == 0 {
            self.info.nodes += 1;
            return self.eval.simple_eval(pos);
        }

        let is_root = ply == 0;
        let position_key = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal);
        let entry = self.transposition_table.probe(position_key);

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
        let static_eval = self.eval.simple_eval(pos);

        if !is_pv && depth <= 7 && !pos.is_check() {
            let score = static_eval - 50 * depth as i32;
            if score >= beta {
                self.info.nodes += 1;
                return static_eval;
            }
        }

        let moves = pos.legal_moves();

        if moves.len() == 0 {
            self.info.nodes += 1;
            return match pos.is_checkmate() {
                true => -100000 + ply as i32,
                false => 0,
            };
        }

        let start_alpha = alpha;
        let mut best_score = -100000;
        let ordered_moves = self.order_moves(entry, moves);
        let mut best_move = &ordered_moves[0];

        for i in 0..ordered_moves.len() {
            let m = &ordered_moves[i];
            let mut pos = pos.clone();
            pos.play_unchecked(&m);

            let mut score: i32;

            match i {
                0 => score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1),
                _ => {
                    score = -self.negamax(&pos, depth - 1, -alpha - 1, -alpha, ply + 1);

                    if score > alpha && beta - alpha > 1 {
                        score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1);
                    }
                }
            }

            if score > best_score {
                best_score = score;
                best_move = m;

                if is_root { self.iteration_move = best_move.clone(); }
                if best_score > alpha { alpha = best_score }
                if alpha >= beta { break; }
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

    fn move_importance(&self, entry: &TranspositionTableEntry, m: &Move) -> i32 {
        match m {
            m if m == &entry._move => 200,
            m if m.capture().is_some() => {
                let piece_value = m.role() as i32;
                let capture_value = m.capture().unwrap() as i32;

                50 * capture_value - piece_value
            }
            m if m.promotion().is_some() => m.promotion().unwrap() as i32,
            _ => 0,
        }
    }

    fn order_moves(&self, entry: TranspositionTableEntry, mut moves: MoveList) -> MoveList {
        moves.sort_by(|a, b| {
            let a_score = self.move_importance(&entry, &a);
            let b_score = self.move_importance(&entry, &b);

            b_score.cmp(&a_score)
        });

        moves
    }
}

impl Default for Search {
    fn default() -> Self {
        Search {
            game: Chess::default(),
            eval: Eval::default(),
            iteration_move: DEFAULT_MOVE.clone(),
            transposition_table: TranspositionTable::new(0x7FFFFF),
            params: SearchParams::default(),
            info: SearchInfo::default(),
            time_controller: TimeController::default(),
        }
    }
}
