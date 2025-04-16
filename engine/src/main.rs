//! CastledEngine - A UCI chess engine implementation in Rust.
//! Main entry point and module declarations.

use crate::uci::UciController;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::cell::RefCell;
use std::process::exit;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::rc::Rc;
use std::{env, io, thread};
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
    let args: Vec<String> = env::args().collect();

    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8MB stack size
        .spawn(move || {
            let mut uci_controller = UciController::default();

            uci_controller.parse_command("uci");

            while let Ok(command) = rx.recv() {
                uci_controller.parse_command(&command);
            }
        })
        .expect("Thread creation failed");

    if args.len() > 1 {
        let command = args[1..].join(" ");
        tx.send(command).unwrap();
        drop(tx);
        handle.join().unwrap();
        exit(0);
    }

    let mut input = String::new();

    loop {
        input.clear();

        io::stdin().read_line(&mut input).ok().unwrap();
        let command = input.trim().to_string();

        if command == "quit" {
            break;
        }

        tx.send(command).unwrap();
    }

    drop(tx);
    handle.join().unwrap();
}

use std::sync::mpsc;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::sync::{LazyLock, Mutex};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
static UCI: LazyLock<Mutex<UciController>> = LazyLock::new(|| Mutex::new(UciController::web()));

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
