pub mod history;
pub mod history_stack;
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
use shakmaty::{Chess, Position};
use tt::TranspositionTable;

use crate::config::Config;
use crate::search::history_stack::HistoryStack;
use crate::{nnue::NNUEState, time_control::time_controller::TimeController};

pub struct SearchState {
    pub game: Chess,
    pub params: SearchParams,
    pub info: SearchInfo,
    pub tc: TimeController,
    pub nnue: NNUEState,
    pub tt: TranspositionTable,
    pub hstack: HistoryStack,
    pub pv: PvTable,
    pub km: Killers,
    pub cfg: Config,
    pub hist: HistoryTable,
}

impl SearchState {
    pub fn new() -> Self {
        let cfg = Config::default();

        Self {
            game: Chess::default(),
            tt: TranspositionTable::new(cfg.hash.value * 1024 * 1024 / 24),
            info: SearchInfo::default(),
            tc: TimeController::default(),
            params: SearchParams::default(),
            nnue: NNUEState::from_board(Chess::default().board()),
            hstack: HistoryStack::new(),
            pv: PvTable::default(),
            km: Killers::new(),
            hist: HistoryTable::new(),
            cfg,
        }
    }
}
