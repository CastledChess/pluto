pub struct SearchInfo {
    pub nodes: u32,
    pub depth: u8,
}

impl Default for SearchInfo {
    fn default() -> Self {
        SearchInfo { nodes: 0, depth: 0 }
    }
}

