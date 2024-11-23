use crate::bound::Bound;
use crate::eval::Eval;
use crate::moves::DEFAULT_MOVE;
use crate::timecontrol::TimeControl;
use crate::transposition::TranspositionTable;
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{CastlingMode, Chess, Color, EnPassantMode, Move, Position, Role};
use std::time::SystemTime;

pub struct Search {
    pub game: Chess,
    pub time_control: TimeControl,
    pub depth: u32,
    pub movetime: u32,
    pub wtime: u32,
    pub btime: u32,
    play_time: u32,
    eval: Eval,
    iteration_move: Move,
    transposition_table: TranspositionTable,
    start_time: SystemTime,
}

impl Search {
    pub fn go(&mut self) {
        self.play_time = match self.game.turn() {
            Color::White => self.wtime,
            Color::Black => self.btime,
        };

        self.transposition_table.new_search();
        self.start_time = SystemTime::now();
        let mut best_move = DEFAULT_MOVE.clone();

        for current_depth in 1..self.depth + 1 {
            let pos = self.game.clone();
            let iteration_score = self.negamax(&pos, current_depth, -100000, 100000, 0);
            let duration = SystemTime::now().duration_since(self.start_time);
            let elapsed = duration.unwrap().as_millis();

            if self.time_control == TimeControl::MoveTime && elapsed > self.movetime as u128 {
                break;
            }
            if self.time_control == TimeControl::WOrBTime && elapsed > self.play_time as u128 / 30 {
                break;
            }

            best_move = self.iteration_move.clone();

            println!(
                "info depth {} score cp {} time {} pv {}",
                current_depth,
                iteration_score,
                elapsed,
                best_move.to_uci(CastlingMode::Standard)
            );
        }

        println!("bestmove {}", best_move.to_uci(CastlingMode::Standard));
    }

    fn negamax(&mut self, pos: &Chess, depth: u32, mut alpha: i32, beta: i32, ply: u32) -> i32 {
        let duration = SystemTime::now().duration_since(self.start_time);
        let elapsed = duration.unwrap().as_millis();

        match self.time_control {
            TimeControl::WOrBTime => {
                if elapsed > self.play_time as u128 / 30 {
                    return 0;
                }
            }

            TimeControl::MoveTime => {
                if elapsed > self.movetime as u128 {
                    return 0;
                }
            }
            _ => {}
        };

        if depth == 0 {
            return self.eval.simple_eval(pos.board().clone(), pos.turn());
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
            return entry.score;
        }

        let moves = &mut pos.legal_moves();
        let start_alpha = alpha;
        let mut best_score = -100000;

        if moves.len() == 0 {
            if pos.is_checkmate() {
                return -10000 + ply as i32;
            }

            return best_score;
        }

        let mut best_move = &moves[0];
        let mut move_scores = vec![];

        for i in 0..moves.len() {
            if moves[i] == entry._move {
                move_scores.push(200);
            } else if let Some(capture) = moves[i].capture() {
                let piece_value = match moves[i].role() {
                    Role::Pawn => 1,
                    Role::Knight => 3,
                    Role::Bishop => 3,
                    Role::Rook => 5,
                    Role::Queen => 9,
                    _ => 0,
                };
                let capture_value = match capture {
                    Role::Pawn => 1,
                    Role::Knight => 3,
                    Role::Bishop => 3,
                    Role::Rook => 5,
                    Role::Queen => 9,
                    _ => 0,
                };

                move_scores.push(100 * capture_value - piece_value);
            } else if let Some(m) = moves[i].promotion() {
                let promotion_value = match m {
                    Role::Knight => 3,
                    Role::Bishop => 3,
                    Role::Rook => 5,
                    Role::Queen => 9,
                    _ => 0,
                };

                move_scores.push(promotion_value);
            } else {
                move_scores.push(0);
            }
        }

        let mut move_indices: Vec<usize> = (0..moves.len()).collect();
        move_indices.sort_by_key(|&i| -move_scores[i]);

        for &i in &move_indices {
            let m = &moves[i];

            let mut pos = pos.clone();
            pos.play_unchecked(&m);
            let score = -self.negamax(&pos, depth - 1, -beta, -alpha, ply + 1);

            if score > best_score {
                best_score = score;
                best_move = m;

                if is_root {
                    self.iteration_move = best_move.clone();
                }
                if best_score > alpha {
                    alpha = best_score
                }
                if alpha >= beta {
                    break;
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

        best_score
    }
}

impl Default for Search {
    fn default() -> Self {
        Search {
            game: Chess::default(),
            time_control: TimeControl::Infinite,
            depth: 1000,
            movetime: 0,
            wtime: 0,
            btime: 0,
            play_time: 0,
            eval: Eval::default(),
            iteration_move: DEFAULT_MOVE.clone(),
            start_time: SystemTime::now(),
            transposition_table: TranspositionTable::new(0x7FFFFF),
        }
    }
}
