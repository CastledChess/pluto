use crate::bound::Bound;
use crate::eval::Eval;
use crate::logger::Logger;
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

    pub fn make_move(&mut self, pos: &mut Chess, m: &Move, eval: i32) {
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
            .hstack
            .push(pos.zobrist_hash(EnPassantMode::Legal), Some(eval));
    }

    pub fn undo_move(&mut self) {
        self.state.nnue.pop();
        self.state.hstack.pop();
    }

    pub fn go(&mut self, print: bool) {
        self.state
            .tc
            .setup(&self.state.params, &self.state.game, &self.state.cfg);
        self.state.hist.new_search();
        self.state.info.nodes = 0;
        self.state.tt.new_search();

        let mut best_move = None;

        /* Iterative deepening */
        for current_depth in 0..self.state.params.depth {
            if TimeMode::is_finite(&self.state.tc.time_mode)
                && (self.state.tc.elapsed() * self.state.cfg.tc_elapsed_factor.value) as u128
                    > self.state.tc.play_time
            {
                break;
            }

            self.state.info.depth = current_depth + 1;
            let pos = self.state.game.clone();
            let iteration_score = self.negamax(&pos, self.state.info.depth, -100000, 100000, 0);

            if self.state.tc.is_time_up() {
                break;
            }

            best_move = Some(self.state.pv.get_best_move().unwrap());

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
                best_move.unwrap().to_uci(CastlingMode::Standard)
            ));
        }
    }

    fn negamax(
        &mut self,
        pos: &Chess,
        mut depth: u8,
        mut alpha: i32,
        beta: i32,
        ply: usize,
    ) -> i32 {
        self.state.pv.update_length(ply);

        if self.state.tc.is_time_up() {
            return 0;
        }

        if depth == 0 {
            return self.quiesce(pos, alpha, beta, self.state.cfg.qsearch_depth.value);
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

        /* Improving */
        let improving = match ply {
            ply if ply < 2 => false,
            _ => {
                static_eval >= {
                    let e = self.state.hstack.get_eval(ply - 2);
                    if let Some(e) = e {
                        e
                    } else {
                        static_eval
                    }
                }
            }
        };

        /* Threefold Detection */
        if ply > 0 && self.state.hstack.count_zobrist(position_key) >= 1 {
            return 0;
        }

        let is_pv = beta - alpha != 1;
        let is_check = pos.is_check();

        if !is_check && !is_pv {
            /* Null Move Pruning */
            if depth > self.state.cfg.nmp_depth.value && ply > 0 && Eval::has_pieces(pos) {
                let r = match improving {
                    true => (self.state.cfg.nmp_margin.value
                        + depth / self.state.cfg.nmp_divisor_improving.value)
                        .min(depth),
                    false => (self.state.cfg.nmp_margin.value
                        + depth / self.state.cfg.nmp_divisor.value)
                        .min(depth),
                };

                let pos = pos.clone().swap_turn().unwrap();
                let score = -self.negamax(&pos, depth - r, -beta, -beta + 1, ply + 1);

                if score >= beta {
                    return score;
                }
            }

            /* Reverse Futility Pruning */
            if depth <= self.state.cfg.rfp_depth.value {
                let rfp_margin = match improving {
                    true => {
                        self.state.cfg.rfp_base_margin.value * depth as i32
                            - self.state.cfg.rfp_reduction_improving.value
                    }
                    false => self.state.cfg.rfp_base_margin.value * depth as i32,
                };

                if static_eval >= beta + rfp_margin {
                    return static_eval;
                }
            }
        }

        let moves = pos.legal_moves();
        let mp = MovePicker::new(&moves, &self.state, &entry, ply);

        /* Internal Iterative Reductions */
        if let Some(tt_move) = entry._move {
            if !moves.contains(&tt_move) && depth > 1 {
                depth -= 1;
            }
        }

        /* Checkmate/Draw Detection */
        if moves.is_empty() {
            return match pos.is_checkmate() {
                true => -100000 + ply as i32,
                false => 0,
            };
        }

        let start_alpha = alpha;
        let mut best_score = -100000;
        let mut skip_quiets = false;
        let mut best_move = &moves[0];

        for (i, m) in mp.enumerate() {
            if skip_quiets && (!m.is_promotion() && !m.is_capture()) {
                continue;
            }

            /* Late Move Pruning */
            if !m.is_capture()
                && !is_pv
                && !m.is_promotion()
                && !is_check
                && i >= self.state.cfg.lmp_move_margin.value
                    + (self.state.cfg.lmp_depth_factor.value * depth) as usize
            {
                continue;
            }

            let mut pos = pos.clone();
            self.make_move(&mut pos, m, static_eval);

            let mut score: i32;
            let mut r = 1;

            /* Late Move Reductions */
            if depth >= self.state.cfg.lmr_depth.value
                && i >= self.state.cfg.lmr_move_margin.value
                && !pos.is_check()
            {
                r = match m {
                    m if m.is_capture() || m.is_promotion() => {
                        (self.state.cfg.lmr_base_margin.value
                            + (depth as f64).ln() * (i as f64).ln()
                                / self.state.cfg.lmr_base_divisor.value)
                            as u8
                    }

                    _ => {
                        (self.state.cfg.lmr_quiet_margin.value
                            + (depth as f64).ln() * (i as f64).ln()
                                / self.state.cfg.lmr_quiet_divisor.value)
                            as u8
                    }
                };

                if !improving {
                    r *= 2;
                }

                r = r.clamp(1, depth);
            }

            /* Extended Futility Pruning */
            if depth - r <= self.state.cfg.fp_depth_margin.value
                && static_eval
                    + (self.state.cfg.fp_base_margin.value
                        + self.state.cfg.fp_margin_depth_factor.value * (depth - r) as i32)
                    < alpha
            {
                skip_quiets = true;
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
                    self.state.hist.update(m.role(), m.to(), depth as i32);
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
            self.make_move(&mut pos, &m, stand_pat);
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
