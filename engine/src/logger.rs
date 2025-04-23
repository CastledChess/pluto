#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use crate::postMessage;

pub struct Logger {}

impl Logger {
    pub fn log(message: &str) {
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        println!("{}", message);

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        postMessage(message);
    }
}
