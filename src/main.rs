use crate::uci::{Uci, UciParser};
use std::io;

mod bound;
mod eval;
mod search;
mod timecontrol;
mod transposition;
mod uci;

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
