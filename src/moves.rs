use shakmaty::{Move, Role, Square};

pub(crate) static DEFAULT_MOVE: Move = Move::Normal {
    role: Role::Pawn,
    from: Square::A1,
    to: Square::A1,
    promotion: None,
    capture: None,
};
