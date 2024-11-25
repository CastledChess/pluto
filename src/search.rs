use crate::bound::Bound;
use crate::eval::Eval;
use crate::moves::DEFAULT_MOVE;
use crate::timecontrol::TimeControl;
use crate::transposition::{TranspositionTable, TranspositionTableEntry};
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{CastlingMode, Chess, Color, EnPassantMode, Move, MoveList, Position};
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

            if (self.time_control == TimeControl::MoveTime && elapsed > self.movetime as u128) ||
                (self.time_control == TimeControl::WOrBTime && elapsed > self.play_time as u128 / 30)
            { break; }

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

        if (self.time_control == TimeControl::MoveTime && elapsed > self.movetime as u128) ||
            (self.time_control == TimeControl::WOrBTime && elapsed > self.play_time as u128 / 30)
        { return 0; }

        if depth == 0 { return self.eval.simple_eval(pos); }

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

        let is_pv = beta - alpha != 1;
        let static_eval = self.eval.simple_eval(pos);

        if !is_pv && depth <= 7 && !pos.is_check() {
            let score = static_eval - 50 * depth as i32;
            if score >= beta {
                return static_eval;
            }
        }

        let moves = pos.legal_moves();

        if moves.len() == 0 {
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
