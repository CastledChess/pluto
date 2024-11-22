use shakmaty::Chess;

pub trait SearchEngine {
    fn go(&self);
}

pub enum TimeControl {
    Infinite,
    MoveTime,
    WOrBTime,
    None,
}

pub struct Search {
    pub game: Chess,
    pub time_control: TimeControl,
    pub depth: u32,
    pub movetime: u32,
    pub wtime: u32,
    pub btime: u32,
}

impl SearchEngine for Search {
    fn go(&self) {}
}

pub(crate) fn default() -> Search {
    Search {
        game: Chess::default(),
        time_control: TimeControl::Infinite,
        depth: 1000,
        movetime: 0,
        wtime: 0,
        btime: 0,
    }
}
