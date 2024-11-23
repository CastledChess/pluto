use crate::bound::Bound;
use shakmaty::zobrist::Zobrist64;
use shakmaty::{Move, Role, Square};

impl Clone for TranspositionTableEntry {
    fn clone(&self) -> Self {
        TranspositionTableEntry {
            key: self.key,
            depth: self.depth,
            score: self.score,
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
            bound: Bound::Alpha,
            _move: Move::Normal {
                role: Role::Pawn,
                from: Square::A1,
                to: Square::A1,
                promotion: None,
                capture: None,
            },
        }
    }
}

pub struct TranspositionTableEntry {
    pub key: Zobrist64,
    pub depth: u32,
    pub score: i32,
    pub bound: Bound,
    pub _move: Move,
}
