/// Represents different time control modes for chess gameplay.
#[derive(Debug)]
pub enum TimeMode {
    /// No time limit, search continues until stopped externally
    Infinite,
    /// Fixed time per move in milliseconds
    MoveTime,
    /// Time control based on remaining time for White or Black
    WOrBTime,
}

impl TimeMode {
    /// Determines if the time control mode has a finite duration.
    ///
    /// # Arguments
    /// * `tc` - Reference to TimeMode to check
    ///
    /// # Returns
    /// * `true` for MoveTime and WOrBTime modes
    /// * `false` for Infinite mode
    pub(crate) fn is_finite(tc: &TimeMode) -> bool {
        match tc {
            TimeMode::MoveTime => true,
            TimeMode::WOrBTime => true,
            _ => false,
        }
    }
}

/// Implements equality comparison for TimeMode enum
impl PartialEq for TimeMode {
    /// Checks if two TimeMode values are equal.
    ///
    /// # Arguments
    /// * `self` - Reference to this TimeMode instance
    /// * `other` - Reference to TimeMode instance to compare against
    ///
    /// # Returns
    /// * `true` if both instances represent the same time mode
    /// * `false` otherwise
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimeMode::Infinite, TimeMode::Infinite) => true,
            (TimeMode::MoveTime, TimeMode::MoveTime) => true,
            (TimeMode::WOrBTime, TimeMode::WOrBTime) => true,
            _ => false,
        }
    }
}