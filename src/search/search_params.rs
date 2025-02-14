pub struct SearchParams {
    pub depth: u8,
    pub move_time: u128,
    pub w_time: u128,
    pub b_time: u128,
    pub max_depth: usize,
    pub num_killers: usize,
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams {
            depth: 5,
            move_time: 0,
            w_time: 0,
            b_time: 0,
            max_depth: 64,
            num_killers: 2,
        }
    }
}