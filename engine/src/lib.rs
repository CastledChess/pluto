#[cfg(target_arch = "wasm32")]
use crate::uci::UciController;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
use std::sync::{LazyLock, Mutex};

#[cfg(target_arch = "wasm32")]
static UCI: LazyLock<Mutex<UciController>> = LazyLock::new(|| Mutex::new(UciController::web()));

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_wasm() {
    log("id name CastledEngine");
    log("id author CastledChess");
    log("uciok");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_wasm(command: &str) {
    let mut uci = UCI.lock().unwrap();

    uci.parse_command(&command);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = self)]
    fn postMessage(s: &str);
}
