/// Position evaluation module containing piece-square tables and evaluation functions.
use crate::nnue::{NNUEState, NNUE};
use shakmaty::{Chess, Color, Position};

pub struct Eval {}

impl Eval {
    /// Neural Network evaluation using NNUE architecture.
    ///
    /// # Arguments
    /// * `state` - Current NNUE network state
    /// * `pos` - Current chess position to evaluate
    ///
    /// # Returns
    /// * Integer score from White's perspective
    pub fn nnue_eval(state: &NNUEState, pos: &Chess) -> i32 {
        #[rustfmt::skip]
        let (us, them) = match pos.turn() {
            Color::White => (state.stack[state.current].white, state.stack[state.current].black),
            Color::Black => ( state.stack[state.current].black, state.stack[state.current].white),
        };

        NNUE.evaluate(&us, &them)
    }
}
