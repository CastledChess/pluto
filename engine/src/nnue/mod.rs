use shakmaty::{Board, Piece, Square};

pub const FEATURES: usize = 768;
pub const HIDDEN: usize = 128;

// Clipped ReLu bounds
// pub const CR_MIN: i16 = 0;
// pub const CR_MAX: i16 = 255;

// Quantization factors
pub const QA: i16 = 255;
pub const QB: i16 = 64;

// Eval scaling factor
pub const SCALE: i32 = 400;

pub static NNUE: Network =
    unsafe { std::mem::transmute(*include_bytes!("../../../bin/simple-100/quantised.bin")) };

#[inline]
/// Clipped ReLU - Activation Function.
/// Note that this takes the i16s in the accumulator to i32s.
fn crelu(x: i16) -> i32 {
    i32::from(x).clamp(0, i32::from(QA))
}

/// This is the quantised format that bullet outputs.
#[repr(C)]
pub struct Network {
    /// Column-Major `HIDDEN x 768` matrix.
    feature_weights: [Accumulator; FEATURES],
    /// Vector with dimension `HIDDEN`.
    feature_bias: Accumulator,
    /// Column-Major `1 x (2 * HIDDEN)`
    /// matrix, we use it like this to make the
    /// code nicer in `Network::evaluate`.
    output_weights: [i16; 2 * HIDDEN],
    /// Scalar output bias.
    output_bias: i16,
}

impl Network {
    /// Calculates the output of the network, starting from the already
    /// calculated hidden layer (done efficiently during makemoves).
    pub fn evaluate(&self, us: &Accumulator, them: &Accumulator) -> i32 {
        // Initialise output with bias.
        let mut output = i32::from(self.output_bias);

        // Side-To-Move Accumulator -> Output.
        for (&input, &weight) in us.vals.iter().zip(&self.output_weights[..HIDDEN]) {
            output += crelu(input) * i32::from(weight);
        }

        // Not-Side-To-Move Accumulator -> Output.
        for (&input, &weight) in them.vals.iter().zip(&self.output_weights[HIDDEN..]) {
            output += crelu(input) * i32::from(weight);
        }

        // Apply eval scale.
        output *= SCALE;

        // Remove quantisation.
        output /= i32::from(QA) * i32::from(QB);

        output
    }
}

#[derive(Clone, Copy)]
pub struct Accumulators {
    pub white: Accumulator,
    pub black: Accumulator,
}

pub(crate) const ON: bool = true;
pub(crate) const OFF: bool = false;

impl Default for Accumulators {
    fn default() -> Self {
        Self {
            white: NNUE.feature_bias,
            black: NNUE.feature_bias,
        }
    }
}

pub struct NNUEState {
    pub stack: [Accumulators; 128],
    pub current: usize,
}

impl NNUEState {
    pub fn new() -> Self {
        NNUEState {
            stack: [Accumulators::default(); 128],
            current: 0,
        }
    }

    pub fn from_board(board: &Board) -> Self {
        let mut state = NNUEState::new();

        for sq in board.occupied().into_iter() {
            state.manual_update::<ON>(board.piece_at(sq).unwrap(), sq);
        }

        state
    }

    pub fn refresh(&mut self, board: &Board) {
        // reset the accumulator stack
        self.current = 0;
        self.stack[self.current] = Accumulators::default();

        // update the first accumulator
        for sq in board.occupied().into_iter() {
            let piece = board.piece_at(sq).unwrap();

            self.manual_update::<ON>(piece, sq);
        }
    }

    pub fn push(&mut self) {
        self.stack[self.current + 1] = self.stack[self.current];
        self.current += 1;
    }

    pub fn pop(&mut self) {
        self.current -= 1;
    }

    pub fn manual_update<const ON: bool>(&mut self, piece: Piece, sq: Square) {
        let (white_idx, black_idx) = nnue_index(piece, sq);

        if ON {
            self.stack[self.current].white.add_feature(white_idx, &NNUE);
            self.stack[self.current].black.add_feature(black_idx, &NNUE);
        } else {
            self.stack[self.current]
                .white
                .remove_feature(white_idx, &NNUE);
            self.stack[self.current]
                .black
                .remove_feature(black_idx, &NNUE);
        }
    }

    pub fn move_update(&mut self, piece: Piece, from: Square, to: Square) {
        let from_idx = nnue_index(piece, from);
        let to_idx = nnue_index(piece, to);

        self.stack[self.current]
            .white
            .remove_feature(from_idx.0, &NNUE);
        self.stack[self.current]
            .black
            .remove_feature(from_idx.1, &NNUE);

        self.stack[self.current].white.add_feature(to_idx.0, &NNUE);
        self.stack[self.current].black.add_feature(to_idx.1, &NNUE);
    }
}

pub fn nnue_index(piece: Piece, sq: Square) -> (usize, usize) {
    const COLOR_STRIDE: usize = 64 * 6;
    const PIECE_STRIDE: usize = 64;
    let p = piece.role as usize - 1;
    let c = piece.color as usize;

    let white_idx = c * COLOR_STRIDE + p * PIECE_STRIDE + sq.flip_vertical() as usize;
    let black_idx = (1 ^ c) * COLOR_STRIDE + p * PIECE_STRIDE + sq as usize;

    (black_idx, white_idx)
}

/// A column of the feature-weights matrix.
/// Note the `align(64)`.
#[derive(Clone, Copy)]
#[repr(C, align(64))]
pub struct Accumulator {
    vals: [i16; HIDDEN],
}

impl Accumulator {
    /// Add a feature to an accumulator.
    pub fn add_feature(&mut self, feature_idx: usize, net: &Network) {
        for (i, d) in self
            .vals
            .iter_mut()
            .zip(&net.feature_weights[feature_idx].vals)
        {
            *i += *d
        }
    }

    /// Remove a feature from an accumulator.
    pub fn remove_feature(&mut self, feature_idx: usize, net: &Network) {
        for (i, d) in self
            .vals
            .iter_mut()
            .zip(&net.feature_weights[feature_idx].vals)
        {
            *i -= *d
        }
    }
}
