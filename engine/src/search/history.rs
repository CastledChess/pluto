use shakmaty::{Role, Square};

pub struct HistoryTable {
    table: [[i32; 64]; 6],
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            table: [[0; 64]; 6],
        }
    }

    pub fn update(&mut self, piece: Role, to: Square, value: i32) {
        self.table[piece as usize - 1][to as usize] += value;
    }

    pub fn get(&self, piece: Role, to: Square) -> i32 {
        self.table[piece as usize - 1][to as usize]
    }

    pub fn new_search(&mut self) {
        for p in self.table.iter_mut() {
            for val in p.iter_mut() {
                *val /= 2;
            }
        }
    }
}
