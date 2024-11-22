use crate::uci::{Uci, UciParser};

mod perft;
mod search;
mod uci;
// use crate::perft::perform_perft;

fn main() {
    // perform_perft(6);

    let search = search::default();
    let mut uci = Uci { search };

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        uci.parse_command(&input);
    }
}
