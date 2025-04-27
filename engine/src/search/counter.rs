use shakmaty::{Move, Square};

pub struct CounterMovesTable {
    table: Vec<Vec<Option<Move>>>,
}

impl CounterMovesTable {
    pub fn new() -> Self {
        Self {
            table: vec![vec![None; 64]; 64],
        }
    }

    pub fn set(&mut self, from: Square, to: Square, m: Move) {
        self.table[from as usize][to as usize] = Some(m);
    }

    pub fn get(&self, from: Square, to: Square) -> &Option<Move> {
        &self.table[from as usize][to as usize]
    }
}
