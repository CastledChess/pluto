use crate::eval::{Eval, SimpleEval};
use shakmaty::{CastlingMode, Chess, Position};
use std::time::SystemTime;

pub trait SearchEngine {
    fn go(&mut self);
    fn negamax(&mut self, depth: i32, alpha: i32, beta: i32) -> i32;
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
    eval: Eval,
}

impl SearchEngine for Search {
    fn go(&mut self) {
        let start = SystemTime::now();
        let moves = self.game.legal_moves();
        let mut best_move = &moves[0];
        let mut best_score = -100000;

        for m in &moves {
            let game = self.game.clone();
            self.game.play_unchecked(&m);
            let score = -self.negamax(5, -100000, 100000);
            self.game = game;

            if score > best_score {
                best_move = &m;
                best_score = score;
            }
        }

        let duration = SystemTime::now().duration_since(start);

        match duration {
            Ok(clock) => {
                println!("info score cp {} time {}", best_score, clock.as_millis());
            }
            Err(_) => {
                panic!();
            }
        }
        println!("bestmove {}", &best_move.to_uci(CastlingMode::Standard));
    }

    fn negamax(&mut self, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            return self
                .eval
                .simple_eval(self.game.board().clone(), self.game.turn());
        }

        let moves = self.game.legal_moves();

        for m in moves {
            let game = self.game.clone();
            self.game.play_unchecked(&m);
            let score = -self.negamax(depth - 1, -beta, -alpha);
            self.game = game;

            if score >= beta {
                return beta;
            }

            if score > alpha {
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
        eval,
    };

    return search;
}
