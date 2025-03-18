/// Parameters controlling the chess engine's search behavior,
/// including depth limits and time controls.
pub struct SearchParams {
    /// Maximum depth to search in plies (half-moves)
    pub depth: u8,
    /// Time allocated for the current move in milliseconds
    pub move_time: u128,
    /// Remaining time for White in milliseconds
    pub w_time: u128,
    /// Remaining time for Black in milliseconds
    pub b_time: u128,
}

/// Implements default initialization for SearchParams struct
impl Default for SearchParams {
    fn default() -> Self {
        SearchParams {
            depth: 5,
            move_time: 0,
            w_time: 0,
            b_time: 0,
        }
    }
}