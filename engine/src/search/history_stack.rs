use shakmaty::zobrist::Zobrist64;

pub struct HistoryStack {
    pub stack: Vec<HistoryStackEntry>,
}

pub struct HistoryStackEntry {
    key: Zobrist64,
    eval: Option<i32>,
}

impl HistoryStack {
    pub fn new() -> Self {
        HistoryStack { stack: Vec::new() }
    }
}

impl HistoryStack {
    pub fn push(&mut self, zobrist: Zobrist64, eval: Option<i32>) {
        self.stack.push(HistoryStackEntry { key: zobrist, eval });
    }

    pub fn pop(&mut self) -> Option<HistoryStackEntry> {
        self.stack.pop()
    }

    pub fn count_zobrist(&self, zobrist: Zobrist64) -> usize {
        self.stack
            .iter()
            .rev()
            .skip(1)
            .filter(|&h| h.key == zobrist)
            .count()
    }

    pub fn get_eval(&self, ply: usize) -> Option<i32> {
        if ply >= self.stack.len() {
            return None;
        }

        self.stack[ply].eval
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
