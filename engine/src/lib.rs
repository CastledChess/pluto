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

mod bound;
mod config;
mod eval;
mod logger;
mod nnue;
mod search;
mod time_control;
mod uci;

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
