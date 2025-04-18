//! Transposition table module for chess engine.
//! Implements a hash table to store and retrieve previously evaluated positions.

use crate::bound::Bound;
use crate::moves::DEFAULT_MOVE;
use shakmaty::zobrist::Zobrist64;
use shakmaty::Move;

/// Entry in the transposition table storing information about a previously evaluated position.
#[repr(C, align(8))]
pub struct TranspositionTableEntry {
    /// Zobrist hash key of the position
    pub key: Zobrist64,
    /// Evaluation score of the position
    pub score: i32,
    /// Depth at which the position was evaluated
    pub depth: u8,
    /// Generation number to track entry age
    pub generation: u8,
    /// Type of score bound (exact, alpha, or beta)
    pub bound: Bound,
    /// Best move found at this position
    pub _move: Move,
}

/// Hash table storing evaluated chess positions for move ordering and pruning.
pub struct TranspositionTable {
    /// Vector of table entries
    pub table: Vec<TranspositionTableEntry>,
    /// Current generation number
    pub generation: u8,
    /// Size of the table
    length: usize,
}

impl TranspositionTable {
    /// Creates a new transposition table with specified size.
    ///
    /// # Arguments
    /// * `length` - Number of entries in the table
    ///
    /// # Returns
    /// * New TranspositionTable instance
    pub(crate) fn new(length: usize) -> TranspositionTable {
        TranspositionTable {
            table: vec![TranspositionTableEntry::default(); length],
            generation: 0,
            length,
        }
    }

    /// Probes the table for a position.
    ///
    /// # Arguments
    /// * `key` - Zobrist hash of the position to look up
    ///
    /// # Returns
    /// * Copy of the table entry at the corresponding index
    pub fn probe(&self, key: Zobrist64) -> TranspositionTableEntry {
        let index = key.0 as usize % self.length;
        self.table[index].clone()
    }

    /// Stores a position in the table.
    ///
    /// # Arguments
    /// * `key` - Zobrist hash of the position
    /// * `depth` - Search depth at which position was evaluated
    /// * `score` - Evaluation score
    /// * `bound` - Type of score bound
    /// * `_move` - Best move found at this position
    pub fn store(&mut self, key: Zobrist64, depth: u8, score: i32, bound: Bound, _move: Move) {
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

    /// Increments the generation counter at the start of a new search.
    pub fn new_search(&mut self) {
        self.generation += 1;
    }

    /// Clears all entries in the table.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.table = vec![TranspositionTableEntry::default(); self.length];
    }
}

/// Implements cloning for table entries.
impl Clone for TranspositionTableEntry {
    /// Creates an exact copy of a table entry.
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

/// Provides default values for table entries.
impl Default for TranspositionTableEntry {
    /// Creates a new entry with default values.
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

