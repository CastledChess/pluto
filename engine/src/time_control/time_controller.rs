use crate::time_control::time_mode::TimeMode;
use crate::{config::Config, search::params::SearchParams};
use chrono::Local;
use shakmaty::{Chess, Color, Position};

pub struct TimeController {
    pub time_mode: TimeMode,
    start_time: i64,
    pub play_time: u128,
}

impl TimeController {
    pub fn start(&mut self) {
        self.start_time = Local::now().timestamp_millis();
    }

    pub fn setup(&mut self, params: &SearchParams, game: &Chess, cfg: &Config) {
        self.play_time = match self.time_mode {
            TimeMode::MoveTime => params.move_time,
            TimeMode::WOrBTime => match game.turn() {
                Color::White => params.w_time / cfg.tc_time_divisor.value as u128,
                Color::Black => params.b_time / cfg.tc_time_divisor.value as u128,
            },
            _ => 0,
        };

        self.start();
    }

    pub fn elapsed(&self) -> i64 {
        Local::now().timestamp_millis() - self.start_time
    }

    pub fn is_time_up(&self) -> bool {
        if !TimeMode::is_finite(&self.time_mode) {
            return false;
        }

        let elapsed = Local::now().timestamp_millis() - self.start_time;

        elapsed as u128 > self.play_time
    }
}

impl Default for TimeController {
    fn default() -> Self {
        TimeController {
            start_time: Local::now().timestamp_millis(),
            time_mode: TimeMode::Infinite,
            play_time: 0,
        }
    }
}
