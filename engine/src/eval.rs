/// Position evaluation module containing piece-square tables and evaluation functions.
use crate::nnue::{NNUEState, NNUE};
use shakmaty::{Chess, Color, Position};

pub struct Eval {}

impl Eval {
    pub fn has_pieces(pos: &Chess) -> bool {
        let material = pos.board().material_side(pos.turn());

        if material.knight > 0 || material.bishop > 0 || material.rook > 0 || material.queen > 0 {
            return true;
        }

        false
    }

    pub fn nnue_eval(state: &NNUEState, pos: &Chess) -> i32 {
        #[rustfmt::skip]
        let (us, them) = match pos.turn() {
            Color::White => (state.stack[state.current].white, state.stack[state.current].black),
            Color::Black => ( state.stack[state.current].black, state.stack[state.current].white),
        };

        NNUE.evaluate(&us, &them)
    }
}
