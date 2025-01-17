pub struct SearchParams {
    pub depth: u8,
    pub move_time: u128,
    pub w_time: u128,
    pub b_time: u128,
    pub MAX_DEPTH: usize,
    pub NUM_KILLERS: usize,
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams {
            depth: 5,
            move_time: 0,
            w_time: 0,
            b_time: 0,
            MAX_DEPTH: 64,
            NUM_KILLERS: 2,
        }
    }
}