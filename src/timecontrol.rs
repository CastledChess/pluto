pub enum TimeControl {
    Infinite,
    MoveTime,
    WOrBTime,
    None,
}

impl PartialEq for TimeControl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimeControl::Infinite, TimeControl::Infinite) => true,
            (TimeControl::MoveTime, TimeControl::MoveTime) => true,
            (TimeControl::WOrBTime, TimeControl::WOrBTime) => true,
            (TimeControl::None, TimeControl::None) => true,
            _ => false,
        }
    }
}
