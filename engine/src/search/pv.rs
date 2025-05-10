use shakmaty::{CastlingMode, Move};

pub struct PvTable {
    pub length: [i32; 64],
    pub table: Vec<Vec<Option<Move>>>,
}

impl PvTable {
    pub fn default() -> PvTable {
        PvTable {
            length: [0; 64],
            table: vec![vec![None; 64]; 64],
        }
    }
}

impl PvTable {
    pub fn store(&mut self, ply: usize, m: Move) {
        self.table[ply][ply] = Some(m);

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
            .map(|m| match m {
                Some(m) => m.to_uci(CastlingMode::Standard).to_string(),
                None => "".to_string(),
            })
            .collect()
    }

    pub fn get_best_move(&self) -> Option<Move> {
        self.table[0][0].clone()
    }
}
