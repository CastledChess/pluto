//! Principal Variation (PV) handling module.
//! Manages the storage and retrieval of best move sequences found during search.

use crate::moves::DEFAULT_MOVE;
use shakmaty::{CastlingMode, Move};
use std::array::from_fn;

/// Represents a Principal Variation table storing the best move sequences.
/// Uses a triangular table structure to efficiently store move sequences at different plies.
pub struct PvTable {
    /// Stores the length of the principal variation at each ply
    pub length: [i32; 64],
    /// 2D array storing moves for each ply, with maximum depth of 64 plies
    pub table: [[Move; 64]; 64],
}

impl PvTable {
    /// Creates a new PvTable with default values.
    ///
    /// # Returns
    /// * A new PvTable instance initialized with default moves and zero lengths
    pub fn default() -> PvTable {
        PvTable {
            length: [0; 64],
            table: from_fn(|_| from_fn(|_| DEFAULT_MOVE.clone())),
        }
    }
}

impl PvTable {
    /// Stores a move in the PV table at the specified ply.
    /// Also copies the moves from the following ply's PV to maintain the sequence.
    ///
    /// # Arguments
    /// * `ply` - Current search depth
    /// * `m` - Move to store at this ply
    pub fn store(&mut self, ply: usize, m: Move) {
        self.table[ply][ply] = m.clone();

        for next_ply in ply as i32 + 1..self.length[ply + 1] {
            self.table[ply][next_ply as usize] = self.table[ply + 1][next_ply as usize].clone();
        }

        self.length[ply] = self.length[ply + 1];
    }

    /// Updates the length of the principal variation at the specified ply.
    ///
    /// # Arguments
    /// * `ply` - The ply depth to update
    pub fn update_length(&mut self, ply: usize) {
        self.length[ply] = ply as i32;
    }

    /// Collects the principal variation into a vector of UCI move strings.
    ///
    /// # Returns
    /// * Vector of strings representing the moves in UCI format
    pub fn collect(&self) -> Vec<String> {
        self.table[0][0..self.length[0] as usize]
            .iter()
            .map(|m| m.to_uci(CastlingMode::Standard).to_string())
            .collect()
    }

    /// Returns the best move from the principal variation.
    ///
    /// # Returns
    /// * The first move in the principal variation
    pub fn get_best_move(&self) -> Move {
        self.table[0][0].clone()
    }
}