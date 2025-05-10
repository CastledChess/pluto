use crate::uci::UciController;
use std::process::exit;
use std::sync::mpsc;
use std::{env, io, thread};

mod bound;
mod config;
mod eval;
mod logger;
mod nnue;
mod search;
mod time_control;
mod uci;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let args: Vec<String> = env::args().collect();

    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8MB stack size
        .spawn(move || {
            let mut uci_controller = UciController::default();

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
