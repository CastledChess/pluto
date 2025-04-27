use std::cmp::max;

use crate::bound::Bound;
use crate::eval::Eval;
use crate::logger::Logger;
use crate::moves::DEFAULT_MOVE;
use crate::nnue::OFF;
use crate::nnue::ON;
use crate::time_control::time_mode::TimeMode;
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{CastlingMode, CastlingSide, Chess, EnPassantMode, Move, Piece, Position, Square};

use super::move_picker::MovePicker;
use super::SearchState;

pub struct Search {
    pub state: SearchState,
}

impl Search {
    pub fn new() -> Self {
        Self {
            state: SearchState::new(),
        }
    }
    /// Makes a move on the board while updating NNUE (Neural Network) accumulator states.
    /// This method should be used instead of regular make_move when NNUE evaluation is active.
    ///
    /// # Arguments
    /// * `pos` - Mutable reference to the chess position
    /// * `m` - Reference to the move to be played
    pub fn make_move(&mut self, pos: &mut Chess, m: &Move) {
        self.state.nnue.push();
        let turn = pos.turn();
        let board = pos.board();

        match m {
            Move::EnPassant { from, to } => {
                let ep_target = Square::from_coords(to.file(), from.rank()); // captured pawn

                self.state
                    .nnue
                    .manual_update::<OFF>((!pos.turn()).pawn(), ep_target);
            }

            Move::Castle { king, rook } => {
                let side = CastlingSide::from_queen_side(rook < king);
                let rook_target = Square::from_coords(side.rook_to_file(), rook.rank());

                self.state.nnue.move_update(turn.rook(), *rook, rook_target);
                self.state.nnue.move_update(
                    Piece {
                        color: turn,
                        role: m.role(),
                    },
                    m.from().unwrap(),
                    m.to(),
                );
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

                    self.state
                        .nnue
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

                    self.state.nnue.manual_update::<OFF>(piece, *from);
                    self.state.nnue.manual_update::<ON>(promoted_piece, *to);
                } else {
                    self.state.nnue.move_update(piece, *from, *to);
                }
            }

            _ => {}
        }

        pos.play_unchecked(m);
        self.state
            .history
            .push(pos.zobrist_hash(EnPassantMode::Legal));
    }

    /// Reverts the last move made with make_move_nnue.
    /// Restores NNUE state and removes last position from history.
    pub fn undo_move(&mut self) {
        self.state.nnue.pop();
        self.state.history.pop();
    }

    /// Starts the search process using iterative deepening.
    /// Prints search information and best move when complete.
    pub fn go(&mut self, print: bool) {
        self.state.tc.setup(&self.state.params, &self.state.game);
        self.state.hist.new_search();
        self.state.info.nodes = 0;
        self.state.tt.new_search();

        let mut best_move = DEFAULT_MOVE.clone();

        /* Iterative deepening */
        for current_depth in 0..self.state.params.depth {
            if TimeMode::is_finite(&self.state.tc.time_mode)
                && (self.state.tc.elapsed() * 2) as u128 > self.state.tc.play_time
            {
                break;
            }

            self.state.info.depth = current_depth + 1;
            let pos = self.state.game.clone();
            let iteration_score = self.negamax(&pos, self.state.info.depth, -100000, 100000, 0);

            if self.state.tc.is_time_up() {
                break;
            }

            best_move = self.state.pv.get_best_move().unwrap();

            let elapsed = self.state.tc.elapsed();
            let pv = self.state.pv.collect();

            if print {
                Logger::log(&format!(
                    "info depth {} nodes {} nps {} score cp {} time {} pv {}",
                    self.state.info.depth,
                    self.state.info.nodes,
                    self.state.info.nodes as u128 * 1000 / (elapsed + 1) as u128,
                    iteration_score,
                    elapsed,
                    pv.join(" ")
                ));
            }
        }

        if print {
            Logger::log(&format!(
                "bestmove {}",
                best_move.to_uci(CastlingMode::Standard)
            ));
        }
    }

    /// Performs negamax search with alpha-beta pruning and various optimizations.
    ///
    /// # Arguments
    /// * `pos` - Reference to current chess position
    /// * `depth` - Remaining search depth
    /// * `alpha` - Alpha value for alpha-beta pruning
    /// * `beta` - Beta value for alpha-beta pruning
    /// * `ply` - Current ply (half-move) in search
    ///
    /// # Returns
    /// * Score of the position from the perspective of the side to move
    fn negamax(&mut self, pos: &Chess, depth: u8, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        self.state.pv.update_length(ply);

        if self.state.tc.is_time_up() {
            return 0;
        }

        if depth == 0 {
            return self.quiesce(pos, alpha, beta, self.state.cfg.qsearch_depth);
        }

        self.state.info.nodes += 1;

        let is_root = ply == 0;
        let position_key = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal);
        let entry = self.state.tt.probe(position_key);

        /* Transposition Table Cut-offs */
        if entry.key == position_key
            && entry.generation == self.state.tt.generation
            && !is_root
            && entry.depth >= depth
            && (entry.bound == Bound::Exact
                || (entry.bound == Bound::Alpha && entry.score <= alpha)
                || (entry.bound == Bound::Beta && entry.score >= beta))
        {
            return entry.score;
        }

        let static_eval = Eval::nnue_eval(&self.state.nnue, pos);

        if ply > 0
            && self
                .state
                .history
                .iter()
                .rev()
                .skip(1)
                .filter(|&&h| h == position_key)
                .count()
                >= 1
        {
            return 0;
        }

        let is_pv = beta - alpha != 1;
        let is_check = pos.is_check();

        if !is_check {
            /* Null Move Pruning */
            if depth > 3 && !is_pv && ply > 0 && Eval::has_pieces(pos) {
                let r = (4 + depth / 4).min(depth);
                let pos = pos.clone().swap_turn().unwrap();
                let score = -self.negamax(&pos, depth - r, -beta, -beta + 1, ply + 1);

                if score >= beta {
                    return score;
                }
            }

            /* Reverse Futility Pruning */
            if !is_pv && depth <= self.state.cfg.rfp_depth {
                let score = static_eval - self.state.cfg.rfp_depth_multiplier * depth as i32;
                if score >= beta {
                    return static_eval;
                }
            }
        }

        let moves = pos.legal_moves();
        let mp = MovePicker::new(&moves, &self.state, &entry, ply);

        /* Checkmate/Draw Detection */
        if moves.is_empty() {
            return match pos.is_checkmate() {
                true => -100000 + ply as i32,
                false => 0,
            };
        }

        let start_alpha = alpha;
        let mut best_score = -100000;
        let mut best_move = &moves[0];

        for (i, move_index) in mp.enumerate() {
            let m = &moves[move_index];
            let mut pos = pos.clone();
            self.make_move(&mut pos, m);

            let mut score: i32;
            let mut r = 1;

            if depth >= 2 && i >= 1 && !pos.is_check() {
                r = match m {
                    m if m.is_capture() || m.is_promotion() => {
                        max(1, (0.7 + (depth as f64).ln() * (i as f64).ln() / 3.0) as u8)
                    }
                    _ => max(1, (0.7 + (depth as f64).ln() * (i as f64).ln() / 2.4) as u8),
                };
            }
            /* Principal Variation Search */
            match i {
                0 => score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1),
                _ => {
                    score = -self.negamax(&pos, depth - r, -(alpha + 1), -alpha, ply + 1);

                    if score > alpha && beta - alpha > 1 {
                        score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1);
                    }
                }
            }

            self.undo_move();

            if score > best_score {
                best_score = score;
                best_move = m;

                if best_score > alpha {
                    self.state.pv.store(ply, best_move.clone());
                    alpha = best_score;
                }
            }

            if score >= beta {
                self.state.km.store(ply, m.clone());

                if !m.is_capture() {
                    let from = m.from().unwrap();
                    let to = m.to();
                    let role = m.role();

                    self.state.hist.update(role, to, depth as i32);
                    self.state.counter.set(from, to, m.clone());
                }

                break;
            }
        }

        let bound = match best_score {
            score if score <= start_alpha => Bound::Alpha,
            score if score >= beta => Bound::Beta,
            _ => Bound::Exact,
        };

        self.state
            .tt
            .store(position_key, depth, best_score, bound, best_move.clone());

        best_score
    }

    /// Performs quiescence search to evaluate tactical sequences.
    ///
    /// # Arguments
    /// * `pos` - Reference to current chess position
    /// * `alpha` - Alpha value for alpha-beta pruning
    /// * `beta` - Beta value for alpha-beta pruning
    /// * `limit` - Maximum remaining depth for quiescence search
    ///
    /// # Returns
    /// * Static evaluation or tactical sequence evaluation
    fn quiesce(&mut self, pos: &Chess, mut alpha: i32, beta: i32, limit: u8) -> i32 {
        self.state.info.nodes += 1;

        let stand_pat = Eval::nnue_eval(&self.state.nnue, pos);

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
            self.make_move(&mut pos, &m);
            let score = -self.quiesce(&pos, -beta, -alpha, limit - 1);
            self.undo_move();

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}
