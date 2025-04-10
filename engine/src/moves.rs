/// Module providing chess move-related constants and utilities.
use shakmaty::{Move, Role, Square};

/// Default chess move constant for initialization purposes.
/// Represents a null move from A1 to A1 with a pawn, no promotion, and no capture.
/// This is used as a placeholder or sentinel value when a valid move is not available.
pub(crate) static DEFAULT_MOVE: Move = Move::Normal {
    role: Role::Pawn,
    from: Square::A1,
    to: Square::A1,
    promotion: None,
    capture: None,
};