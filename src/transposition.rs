use crate::bound::Bound;
use crate::moves::DEFAULT_MOVE;
use shakmaty::zobrist::Zobrist64;
use shakmaty::Move;

pub struct TranspositionTableEntry {
    pub key: Zobrist64,
    pub depth: u32,
    pub score: i32,
    pub bound: Bound,
    pub _move: Move,
    pub generation: u32,
}

pub struct TranspositionTable {
    pub table: Vec<TranspositionTableEntry>,
    pub generation: u32,
    length: usize,
}

impl TranspositionTable {
    pub(crate) fn new(length: usize) -> TranspositionTable {
        TranspositionTable {
            table: vec![TranspositionTableEntry::default(); length],
            generation: 0,
            length,
        }
    }

    pub fn probe(&self, key: Zobrist64) -> &TranspositionTableEntry {
        let index = key.0 as usize % self.length;

        &self.table[index]
    }

    pub fn store(&mut self, key: Zobrist64, depth: u32, score: i32, bound: Bound, _move: Move) {
        let index = key.0 as usize % self.table.len();
        let entry = TranspositionTableEntry {
            key,
            depth,
            score,
            bound,
            _move,
            generation: self.generation,
        };

        self.table[index] = entry;
    }

    pub fn new_search(&mut self) {
        self.generation += 1;
    }

    pub fn clear(&mut self) {
        self.table = vec![TranspositionTableEntry::default(); self.length];
    }
}

impl Clone for TranspositionTableEntry {
    fn clone(&self) -> Self {
        TranspositionTableEntry {
            key: self.key,
            depth: self.depth,
            score: self.score,
            generation: self.generation,
            bound: self.bound.clone(),
            _move: self._move.clone(),
        }
    }
}

impl Default for TranspositionTableEntry {
    fn default() -> Self {
        TranspositionTableEntry {
            key: Zobrist64(0),
            depth: 0,
            score: 0,
            generation: 0,
            bound: Bound::Alpha,
            _move: DEFAULT_MOVE.clone(),
        }
    }
}
