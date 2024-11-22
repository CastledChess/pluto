use shakmaty::{Chess, Position};
use std::time::SystemTime;

pub fn perform_perft(depth: u32) {
    let start = SystemTime::now();
    let result = perft(&Chess::default(), depth);
    let duration = SystemTime::now().duration_since(start);
    match duration {
        Ok(clock) => {
            println!(
                "Perft {} Result: {} Time: {}s {}ms",
                depth,
                result,
                clock.as_secs(),
                clock.subsec_nanos() / 1000000
            );
        }
        Err(_) => {
            panic!();
        }
    }
}

pub fn perft<P: Position + Clone>(pos: &P, depth: u32) -> u64 {
    if depth < 1 {
        1
    } else {
        let moves = pos.legal_moves();

        if depth == 1 {
            moves.len() as u64
        } else {
            moves
                .iter()
                .map(|m| {
                    let mut child = pos.clone();
                    child.play_unchecked(m);
                    perft(&child, depth - 1)
                })
                .sum()
        }
    }
}
