//! CastledEngine - A UCI chess engine implementation in Rust.
//! Main entry point and module declarations.

use crate::uci::Uci;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::cell::RefCell;
use std::io;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::rc::Rc;
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_sys::Worker;

mod bound; // Position score bound types
mod config; // Engine configuration settings
mod eval; // Position evaluation
mod moves; // Move generation and handling
mod nnue; // Neural Network evaluation
mod principal_variation; // Best move line tracking
mod search; // Search algorithm implementation
mod time_control; // Time management
mod transposition; // Transposition table for position caching
mod uci; // Universal Chess Interface protocol

/// Main entry point for the chess engine.
/// Initializes the UCI interface and enters the main command processing loop.
/// Follows the Universal Chess Interface (UCI) protocol for chess engine communication.
pub fn main() {
    println!("id name Pluto");
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

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::sync::{LazyLock, Mutex};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
static UCI: LazyLock<Mutex<Uci>> = LazyLock::new(|| Mutex::new(Uci::web()));

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn init_wasm() {
    log("id name CastledEngine");
    log("id author CastledChess");
    log("uciok");
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn main_wasm(command: &str) {
    let mut uci = UCI.lock().unwrap();

    uci.parse_command(&command);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = self)]
    fn postMessage(s: &str);
}
