use crate::eval::{Eval, SimpleEval};
use shakmaty::{CastlingMode, Chess, Color, Move, Position};
use std::time::SystemTime;

pub trait SearchEngine {
    fn go(&mut self);
    fn negamax(&mut self, depth: u32, alpha: i32, beta: i32, ply: u32) -> i32;
}

pub enum TimeControl {
    Infinite,
    MoveTime,
    WOrBTime,
    None,
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
    iteration_move: Option<Move>,
}

impl SearchEngine for Search {
    fn go(&mut self) {
        self.play_time = match self.game.turn() {
            Color::White => self.wtime,
            Color::Black => self.btime,
        };

        let start = SystemTime::now();
        let mut best_move;

        println!("info string starting search {}", self.depth);

        for current_depth in 1..self.depth + 1 {
            let iteration_score = -self.negamax(current_depth, -100000, 100000, 0);
            let duration = SystemTime::now().duration_since(start);
            let elapsed = duration.unwrap().as_millis();

            match self.time_control {
                TimeControl::WOrBTime => {
                    if elapsed > self.play_time as u128 {
                        break;
                    }
                }
                TimeControl::MoveTime => {
                    if elapsed > self.movetime as u128 {
                        break;
                    }
                }
                _ => {}
            };

            let m = <Option<Move> as Clone>::clone(&self.iteration_move).unwrap();
            best_move = &m;

            println!(
                "info depth {} score cp {} time {} pv {}",
                current_depth,
                iteration_score,
                elapsed,
                best_move.to_uci(CastlingMode::Standard)
            );
        }
        let m = <Option<Move> as Clone>::clone(&self.iteration_move).unwrap();
        println!("bestmove {}", m.to_uci(CastlingMode::Standard));
    }

    fn negamax(&mut self, depth: u32, mut alpha: i32, beta: i32, ply: u32) -> i32 {
        if depth == 0 {
            return self
                .eval
                .simple_eval(self.game.board().clone(), self.game.turn());
        }

        let is_root = ply == 0;
        let moves = self.game.legal_moves();

        for m in moves {
            let game = self.game.clone();
            self.game.play_unchecked(&m);
            let score = -self.negamax(depth - 1, -beta, -alpha, ply + 1);
            self.game = game;

            if score >= beta {
                if is_root {
                    self.iteration_move = Some(m);
                }
                return beta;
            }

            if score > alpha {
                if is_root {
                    self.iteration_move = Some(m);
                }
                alpha = score;
            }
        }

        return alpha;
    }
}

pub(crate) fn default() -> Search {
    let eval = Eval::default();
    let search = Search {
        game: Chess::default(),
        time_control: TimeControl::Infinite,
        depth: 1000,
        movetime: 0,
        wtime: 0,
        btime: 0,
        play_time: 0,
        eval,
        iteration_move: None,
    };

    return search;
}
