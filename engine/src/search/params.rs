pub struct SearchParams {
    pub depth: u8,
    pub move_time: u128,
    pub w_time: u128,
    pub b_time: u128,
}

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
