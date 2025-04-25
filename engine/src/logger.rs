#[cfg(target_arch = "wasm32")]
use crate::postMessage;

pub struct Logger {}

impl Logger {
    pub fn log(message: &str) {
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", message);

        #[cfg(target_arch = "wasm32")]
        postMessage(message);
    }
}
