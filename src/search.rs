use crate::bound::Bound;
use crate::eval::{Eval, SimpleEval};
use crate::timecontrol::TimeControl;
use crate::transposition::TranspositionTableEntry;
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{CastlingMode, Chess, Color, EnPassantMode, Move, Position, Role, Square};
use std::time::SystemTime;

pub trait SearchEngine {
    fn go(&mut self);
    fn negamax(&mut self, depth: u32, alpha: i32, beta: i32, ply: u32) -> i32;
}

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
    transposition_table: Vec<TranspositionTableEntry>,
    start_time: SystemTime,
}

impl SearchEngine for Search {
    fn go(&mut self) {
        self.play_time = match self.game.turn() {
            Color::White => self.wtime,
            Color::Black => self.btime,
        };

        println!("info string play_time: {}", self.play_time / 30);

        self.transposition_table = vec![TranspositionTableEntry::default(); 0x7FFFFF];
        self.start_time = SystemTime::now();
        let mut best_move = Move::Normal {
            role: Role::Pawn,
            from: Square::A1,
            to: Square::A1,
            promotion: None,
            capture: None,
        };

        for current_depth in 1..self.depth + 1 {
            let iteration_score = self.negamax(current_depth, -100000, 100000, 0);
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

    fn negamax(&mut self, depth: u32, mut alpha: i32, beta: i32, ply: u32) -> i32 {
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
            return self
                .eval
                .simple_eval(self.game.board().clone(), self.game.turn());
        }

        let is_root = ply == 0;
        let position_key = self.game.zobrist_hash::<Zobrist64>(EnPassantMode::Legal);
        let entry = &self.transposition_table[(position_key.0 % 0x7FFFFF) as usize];

        if entry.key == position_key
            && !is_root
            && entry.depth >= depth
            && (entry.bound == Bound::Exact
                || (entry.bound == Bound::Alpha && entry.score <= alpha)
                || (entry.bound == Bound::Beta && entry.score >= beta))
        {
            return entry.score;
        }

        let moves = &self.game.legal_moves();

        if moves.len() == 0 && self.game.is_checkmate() {
            return -10000 + ply as i32;
        }

        let start_alpha = alpha;
        let mut best_score = -100000;
        let mut best_move = &moves[0];

        for m in moves {
            let game = self.game.clone();
            self.game.play_unchecked(&m);
            let score = -self.negamax(depth - 1, -beta, -alpha, ply + 1);
            self.game = game;

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

        self.transposition_table[(position_key.0 % 0x7FFFFF) as usize] = TranspositionTableEntry {
            key: position_key,
            score: best_score,
            depth,
            bound,
            _move: best_move.clone(),
        };

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
            iteration_move: Move::Normal {
                role: Role::Pawn,
                from: Square::A1,
                to: Square::A1,
                promotion: None,
                capture: None,
            },
            start_time: SystemTime::now(),
            transposition_table: vec![TranspositionTableEntry::default(); 0x7FFFFF],
        }
    }
}
