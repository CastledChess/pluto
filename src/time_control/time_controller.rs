use std::time::SystemTime;
use shakmaty::{Chess, Color, Position};
use crate::search::search_params::SearchParams;
use crate::time_control::time_mode::TimeMode;

/// Manages time control for chess engine operations.
/// Handles different time modes and tracks elapsed time during search.
pub struct TimeController {
    /// Current time control mode (e.g., Infinite, MoveTime, WOrBTime)
    pub time_mode: TimeMode,
    /// Timestamp when search started
    start_time: SystemTime,
    /// Allocated time for current search in milliseconds
    play_time: u128,
}

impl TimeController {
    /// Starts the time control by setting the start time to current system time.
    pub fn start(&mut self) {
        self.start_time = SystemTime::now();
    }

    /// Configures time control based on search parameters and game state.
    ///
    /// # Arguments
    /// * `params` - Search parameters containing time allocations
    /// * `game` - Current chess position for determining active player
    pub fn setup(&mut self, params: &SearchParams, game: &Chess) {
        self.play_time = match self.time_mode {
            TimeMode::MoveTime => params.move_time,
            TimeMode::WOrBTime => match game.turn() {
                Color::White => params.w_time / 30,
                Color::Black => params.b_time / 30
            },
            _ => 0
        };

        self.start();
    }

    /// Returns elapsed time since search start in milliseconds.
    pub fn elapsed(&self) -> u128 {
        let duration = SystemTime::now().duration_since(self.start_time);
        duration.unwrap().as_millis()
    }

    /// Checks if allocated time for current search has been exhausted.
    ///
    /// # Returns
    /// * `true` if time is up, `false` otherwise or if in infinite time mode
    pub fn is_time_up(&self) -> bool {
        if !TimeMode::is_finite(&self.time_mode) {
            return false;
        }

        let duration = SystemTime::now().duration_since(self.start_time);
        let elapsed = duration.unwrap().as_millis();

        elapsed > self.play_time
    }
}

/// Implements default initialization for TimeController struct
impl Default for TimeController {
    fn default() -> Self {
        TimeController {
            start_time: SystemTime::now(),
            time_mode: TimeMode::Infinite,
            play_time: 0,
        }
    }
}