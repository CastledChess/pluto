use crate::uci::Uci;
use std::io;

mod bound;
mod eval;
mod moves;
mod transposition;
mod uci;
mod search;
mod time_control;
mod config;
mod principal_variation;

fn main() {
    println!("id name CastledEngine");
    println!("id author CastledChess");
    println!("uciok");

    let mut uci = Uci::default();
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin().read_line(&mut input).ok().unwrap();

        uci.parse_command(&input);
    }
}
