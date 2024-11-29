use crate::moves::DEFAULT_MOVE;
use shakmaty::{CastlingMode, Move};
use std::array::from_fn;

pub struct PvTable {
    pub length: [i32; 64],
    pub table: [[Move; 64]; 64],
}

impl PvTable {
    pub fn default() -> PvTable {
        PvTable {
            length: [0; 64],
            table: from_fn(|_| from_fn(|_| DEFAULT_MOVE.clone())),
        }
    }
}

impl PvTable {
    pub fn store(&mut self, ply: usize, m: Move) {
        self.table[ply][ply] = m.clone();

        for next_ply in ply as i32 + 1..self.length[ply + 1] {
            self.table[ply][next_ply as usize] = self.table[ply + 1][next_ply as usize].clone();
        }

        self.length[ply] = self.length[ply + 1];
    }

    pub fn update_length(&mut self, ply: usize) {
        self.length[ply] = ply as i32;
    }

    pub fn collect(&self) -> Vec<String> {
        self.table[0][0..self.length[0] as usize]
            .iter()
            .map(|m| m.to_uci(CastlingMode::Standard).to_string())
            .collect()
    }

    pub fn get_best_move(&self) -> Move {
        self.table[0][0].clone()
    }
}