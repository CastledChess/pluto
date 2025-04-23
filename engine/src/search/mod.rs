pub mod search;
pub mod search_info;
pub mod search_params;

use search_info::SearchInfo;
use search_params::SearchParams;
use shakmaty::{zobrist::Zobrist64, Chess, Move, Position};

use crate::{
    config::Config, nnue::NNUEState, principal_variation::PvTable,
    time_control::time_controller::TimeController, transposition::TranspositionTable,
};

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
    pub km: Vec<Vec<Option<Move>>>,
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
            km: vec![vec![None; cfg.nb_killer_moves]; cfg.max_depth_killer_moves],
            cfg,
        }
    }
}
