use std::time::SystemTime;
use shakmaty::{Chess, Color, Position};
use crate::search::search_params::SearchParams;
use crate::time_control::time_mode::TimeMode;

pub struct TimeController {
    pub time_mode: TimeMode,
    start_time: SystemTime,
    play_time: u128,
}

impl TimeController {
    pub fn start(&mut self) {
        self.start_time = SystemTime::now();
    }

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

    pub fn elapsed(&self) -> u128 {
        let duration = SystemTime::now().duration_since(self.start_time);
        duration.unwrap().as_millis()
    }

    pub fn is_time_up(&self) -> bool {
        if !TimeMode::is_finite(&self.time_mode) {
            return false;
        }

        let duration = SystemTime::now().duration_since(self.start_time);
        let elapsed = duration.unwrap().as_millis();

        elapsed > self.play_time
    }
}

impl Default for TimeController {
    fn default() -> Self {
        TimeController {
            start_time: SystemTime::now(),
            time_mode: TimeMode::Infinite,
            play_time: 0,
        }
    }
}