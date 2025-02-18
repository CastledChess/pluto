//! CastledEngine - A UCI chess engine implementation in Rust.
//! Main entry point and module declarations.

use crate::uci::Uci;
use std::io;

mod bound;           // Position score bound types
mod config;         // Engine configuration settings
mod eval;           // Position evaluation
mod moves;          // Move generation and handling
mod nnue;           // Neural Network evaluation
mod principal_variation;  // Best move line tracking
mod search;         // Search algorithm implementation
mod time_control;   // Time management
mod transposition;  // Transposition table for position caching
mod uci;            // Universal Chess Interface protocol

/// Main entry point for the chess engine.
/// Initializes the UCI interface and enters the main command processing loop.
/// Follows the Universal Chess Interface (UCI) protocol for chess engine communication.
fn main() {
    println!("id name CastledEngine");
    println!("id author CastledChess");
    println!("uciok");

    let mut uci = Uci::default();
    let mut input = String::new();

    // Main command processing loop
    loop {
        input.clear();

        // Read UCI commands from standard input
        io::stdin().read_line(&mut input).ok().unwrap();

        // Process the received command
        uci.parse_command(&input);
    }
}