pub mod history;
pub mod info;
pub mod killers;
pub mod move_picker;
pub mod params;
pub mod pv;
pub mod search;
pub mod tt;

use history::HistoryTable;
use info::SearchInfo;
use killers::Killers;
use params::SearchParams;
use pv::PvTable;
use shakmaty::{zobrist::Zobrist64, Chess, Position};
use tt::TranspositionTable;

use crate::{config::Config, nnue::NNUEState, time_control::time_controller::TimeController};

pub struct SearchState {
    pub game: Chess,
    pub params: SearchParams,
    pub info: SearchInfo,
    pub tc: TimeController,
    pub nnue: NNUEState,
    pub tt: TranspositionTable,
    pub history: Vec<Zobrist64>,
    pub cfg: Config,
    pub pv: PvTable,
    pub km: Killers,
    pub hist: HistoryTable,
}

impl SearchState {
    pub fn new() -> Self {
        let cfg = Config::load().unwrap();

        Self {
            game: Chess::default(),
            tt: TranspositionTable::new(cfg.tt_size),
            info: SearchInfo::default(),
            tc: TimeController::default(),
            params: SearchParams::default(),
            nnue: NNUEState::from_board(Chess::default().board()),
            history: Vec::new(),
            pv: PvTable::default(),
            km: Killers::new(),
            hist: HistoryTable::new(),
            cfg,
        }
    }
}
