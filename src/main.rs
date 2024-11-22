use crate::uci::{Uci, UciParser};
use std::io;
use std::io::Write;

mod eval;
mod search;
mod uci;

fn main() {
    println!("id name CastledEngine");
    println!("id author CastledChess");
    println!("uciok");

    let search = search::default();
    let mut uci = Uci { search };
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin().read_line(&mut input).ok().unwrap();

        uci.parse_command(&input);
    }
}
