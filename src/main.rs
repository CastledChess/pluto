mod perft;
mod uci;

use crate::perft::perform_perft;

fn main() {
    perform_perft(6);
}
