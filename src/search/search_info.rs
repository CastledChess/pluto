/// Contains information about the current search process including
/// statistical data and search progress metrics.
pub struct SearchInfo {
    /// Total number of nodes visited during the search
    pub nodes: u32,
    /// Current depth of the search in plies
    pub depth: u8,
}

/// Implements default initialization for SearchInfo struct
impl Default for SearchInfo {
    fn default() -> Self {
        SearchInfo {
            nodes: 0,
            depth: 0,
        }
    }
}