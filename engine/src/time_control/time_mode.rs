#[derive(Debug)]
pub enum TimeMode {
    Infinite,
    MoveTime,
    WOrBTime,
}

impl TimeMode {
    pub(crate) fn is_finite(tc: &TimeMode) -> bool {
        match tc {
            TimeMode::MoveTime => true,
            TimeMode::WOrBTime => true,
            _ => false,
        }
    }
}

impl PartialEq for TimeMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimeMode::Infinite, TimeMode::Infinite) => true,
            (TimeMode::MoveTime, TimeMode::MoveTime) => true,
            (TimeMode::WOrBTime, TimeMode::WOrBTime) => true,
            _ => false,
        }
    }
}

