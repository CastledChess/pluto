use shakmaty::Move;

pub struct Killers {
    table: [Vec<Move>; 64],
}

impl Killers {
    pub fn new() -> Self {
        Self {
            table: [const { Vec::new() }; 64],
        }
    }

    pub fn get(&self, ply: usize) -> &Vec<Move> {
        &self.table[ply]
    }

    pub fn store(&mut self, ply: usize, m: Move) {
        if ply > 64 {
            return;
        }

        if !self.get(ply).contains(&m) {
            self.table[ply].pop();
            self.table[ply].insert(0, m);
        }
    }
}
