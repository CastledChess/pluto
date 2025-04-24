//! CastledEngine - A UCI chess engine implementation in Rust.
//! Main entry point and module declarations.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use crate::uci::UciController;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::rc::Rc;
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_sys::Worker;

mod bound; // Position score bound types
mod config; // Engine configuration settings
mod eval; // Position evaluation
mod logger;
mod moves; // Move generation and handling
mod nnue; // Neural Network evaluation
mod search; // Search algorithm implementation
mod time_control; // Time management
mod uci; // Universal Chess Interface protocol

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
