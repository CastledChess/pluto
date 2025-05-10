use crate::logger::Logger;
use crate::nnue::NNUEState;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use crate::postMessage;
use crate::search::search::Search;
use crate::search::tt::TranspositionTable;
use crate::time_control::time_mode::TimeMode;
use chrono::Local;
use queues::{queue, IsQueue, Queue};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::zobrist::ZobristHash;
use shakmaty::{CastlingMode, Chess, Position};

pub struct UciController {
    search: Search,
}

impl Default for UciController {
    fn default() -> UciController {
        UciController {
            search: Search::new(),
        }
    }
}

impl UciController {
    pub fn parse_command(&mut self, command: &str) {
        let tokens_vec: Vec<&str> = command.split_whitespace().collect();
        let mut tokens: Queue<&str> = queue![];

        for token in tokens_vec {
            tokens.add(token).unwrap();
        }

        self.parse_tokens(&mut tokens);
    }

    fn parse_tokens(&mut self, tokens: &mut Queue<&str>) {
        let first_token = tokens.remove().unwrap();

        self.search.state.tc.time_mode = TimeMode::Infinite;
        self.search.state.params.depth = u8::MAX;

        match first_token {
            "print" => self.handle_print(tokens),
            "bench" => self.handle_bench(),
            "uci" => self.handle_uci(),
            "isready" => self.handle_isready(),
            "quit" => self.handle_quit(),
            "setoption" => self.handle_setoption(tokens),
            "ucinewgame" => self.handle_ucinewgame(),
            "position" => self.handle_position(tokens),
            "go" => self.handle_go(tokens),
            _ => Logger::log(&format!("Unknown command: {}", first_token)),
        }
    }

    fn handle_print(&self, tokens: &mut Queue<&str>) {
        let scope = tokens.remove().unwrap();

        match scope {
            "spsa" => self.handle_print_spsa(tokens),
            _ => Logger::log(&format!("unknown scope: {}", scope)),
        }
    }

    fn handle_print_spsa(&self, tokens: &mut Queue<&str>) {
        let target = tokens.remove().unwrap();

        match target {
            "workload" => self.handle_print_spsa_workload(),
            _ => Logger::log(&format!("unknown target: {}", target)),
        }
    }

    fn handle_print_spsa_workload(&self) {
        Logger::log(&self.search.state.cfg.qsearch_depth.fmt_spsa());
        Logger::log(&self.search.state.cfg.rfp_depth.fmt_spsa());
        Logger::log(&self.search.state.cfg.rfp_base_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.rfp_reduction_improving.fmt_spsa());
        Logger::log(&self.search.state.cfg.fp_base_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.fp_depth_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.fp_margin_depth_factor.fmt_spsa());
        Logger::log(&self.search.state.cfg.nmp_depth.fmt_spsa());
        Logger::log(&self.search.state.cfg.nmp_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.nmp_divisor.fmt_spsa());
        Logger::log(&self.search.state.cfg.nmp_divisor_improving.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmp_move_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmp_depth_factor.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmr_depth.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmr_move_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmr_quiet_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmr_quiet_divisor.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmr_base_margin.fmt_spsa());
        Logger::log(&self.search.state.cfg.lmr_base_divisor.fmt_spsa());
        Logger::log(&self.search.state.cfg.mo_tt_entry_value.fmt_spsa());
        Logger::log(&self.search.state.cfg.mo_capture_value.fmt_spsa());
        Logger::log(&self.search.state.cfg.mo_killer_value.fmt_spsa());
        Logger::log(&self.search.state.cfg.tc_time_divisor.fmt_spsa());
        Logger::log(&self.search.state.cfg.tc_elapsed_factor.fmt_spsa());
    }

    fn handle_bench(&mut self) {
        self.search.state.tt.clear();

        let positions = vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ",
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ",
        ];

        let mut total = 0;
        let start_time = Local::now().timestamp_millis();

        for position in positions {
            let fen: Fen = position.parse().ok().unwrap();
            let game = fen.into_position(CastlingMode::Standard).ok().unwrap();

            self.search.state.game = game;
            self.search.state.params.depth = 14;
            self.search.state.tc.time_mode = TimeMode::Infinite;

            self.search.go(false);

            total += self.search.state.info.nodes;
        }
        let elapsed = Local::now().timestamp_millis() - start_time;

        println!(
            "{} nodes {} nps",
            total,
            total as u128 * 1000 / (elapsed + 1) as u128
        );
    }

    fn handle_go(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove();

        match token.is_ok() {
            true => match token.unwrap() {
                "btime" => self.handle_btime(tokens),
                "wtime" => self.handle_wtime(tokens),
                "binc" => self.handle_binc(tokens),
                "winc" => self.handle_winc(tokens),
                "depth" => self.handle_go_depth(tokens),
                "movetime" => self.handle_go_movetime(tokens),
                "infinite" => self.handle_go_infinite(tokens),
                _ => Logger::log(&format!("Unknown go command: {}", token.unwrap())),
            },
            false => {
                self.search.go(true);
            }
        }
    }

    fn handle_winc(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let _inc = token.parse::<u32>().unwrap();

        // TODO: save winc

        self.handle_go(tokens);
    }

    fn handle_binc(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let _inc = token.parse::<u32>().unwrap();

        // TODO: save binc

        self.handle_go(tokens);
    }

    fn handle_btime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u128>().unwrap();

        self.search.state.params.depth = u8::MAX;
        self.search.state.tc.time_mode = TimeMode::WOrBTime;
        self.search.state.params.b_time = time;

        self.handle_go(tokens);
    }

    fn handle_wtime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u128>().unwrap();

        self.search.state.params.depth = u8::MAX;
        self.search.state.tc.time_mode = TimeMode::WOrBTime;
        self.search.state.params.w_time = time;

        self.handle_go(tokens);
    }

    fn handle_go_depth(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let depth = token.parse::<u8>().unwrap();

        self.search.state.params.depth = depth;
        self.search.state.tc.time_mode = TimeMode::Infinite;

        self.handle_go(tokens);
    }

    fn handle_go_movetime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u128>().unwrap();

        self.search.state.params.move_time = time;
        self.search.state.tc.time_mode = TimeMode::MoveTime;
        self.search.state.params.depth = u8::MAX;

        self.handle_go(tokens);
    }

    fn handle_position(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();

        match token {
            "startpos" => {
                self.handle_position_startpos(tokens);
            }
            "fen" => self.handle_position_fen(tokens),
            _ => Logger::log(&format!("Unknown position command: {}", token)),
        }
    }

    fn handle_position_startpos(&mut self, tokens: &mut Queue<&str>) {
        self.search.state.game = Chess::default();
        self.search.state.hstack.clear();

        if let Ok(moves) = tokens.remove() {
            if moves != "moves" {
                return;
            }

            while let Ok(move_str) = tokens.remove() {
                let uci_move = move_str.parse::<UciMove>().ok();
                let game = self.search.state.game.clone();
                let legal = uci_move.unwrap().to_move(&game).ok().unwrap();

                self.search.state.game = game.play(&legal).unwrap();
                self.search.state.hstack.push(
                    self.search
                        .state
                        .game
                        .zobrist_hash(shakmaty::EnPassantMode::Legal),
                    None,
                )
            }
        }

        self.search.state.nnue = NNUEState::from_board(self.search.state.game.board());
    }

    fn handle_position_fen(&mut self, tokens: &mut Queue<&str>) {
        let mut fen_vec: Vec<&str> = vec![tokens.remove().ok().unwrap()];
        let mut token: &str = "";

        loop {
            let result = tokens.remove().ok();

            match result {
                None => break,
                Some(value) => token = value,
            }

            if token == "moves" {
                break;
            }

            fen_vec.push(token);
        }

        let fen: Fen = fen_vec.join(" ").as_str().parse().ok().unwrap();

        self.search.state.game = fen.into_position(CastlingMode::Standard).ok().unwrap();
        self.search.state.hstack.clear();

        if token == "moves" {
            while let Ok(move_str) = tokens.remove() {
                let uci_move = move_str.parse::<UciMove>().ok();
                let game = self.search.state.game.clone();
                let legal = uci_move.unwrap().to_move(&game).ok().unwrap();

                self.search.state.game = game.play(&legal).unwrap();
                self.search.state.hstack.push(
                    self.search
                        .state
                        .game
                        .zobrist_hash(shakmaty::EnPassantMode::Legal),
                    None,
                )
            }
        }

        self.search.state.nnue = NNUEState::from_board(self.search.state.game.board());
    }

    fn handle_setoption(&mut self, tokens: &mut Queue<&str>) {
        if tokens.size() < 4 {
            return;
        }

        tokens.remove().unwrap(); // name
        let name = tokens.remove().unwrap();
        tokens.remove().unwrap(); // value
        let value = tokens.remove().unwrap();

        if name.is_empty() || value.is_empty() {
            return;
        }

        match name {
            "MoveOverhead" => Logger::log("info string MoveOverhead is not yet supported."),
            "Threads" => Logger::log("info string Multithreading is not yet supported."),
            "Hash" => {
                let size = value.parse::<u32>().unwrap();
                let bytes = size * 1024 * 1024;
                let entries = bytes / 24; // 24 is the actual size of one entry

                self.search.state.tt = TranspositionTable::new(entries as usize);
            }
            "QSearchDepth" => {
                self.search.state.cfg.qsearch_depth.value = value.parse::<u8>().unwrap()
            }
            "RFPDepth" => self.search.state.cfg.rfp_depth.value = value.parse::<u8>().unwrap(),
            "RFPBaseMargin" => {
                self.search.state.cfg.rfp_base_margin.value = value.parse::<i32>().unwrap()
            }
            "RFPReductionImproving" => {
                self.search.state.cfg.rfp_reduction_improving.value = value.parse::<i32>().unwrap()
            }
            "FPDepthMargin" => {
                self.search.state.cfg.fp_depth_margin.value = value.parse::<u8>().unwrap()
            }
            "FPBaseMargin" => {
                self.search.state.cfg.fp_base_margin.value = value.parse::<i32>().unwrap()
            }
            "FPMarginDepthFactor" => {
                self.search.state.cfg.fp_margin_depth_factor.value = value.parse::<i32>().unwrap()
            }
            "NMPDepth" => self.search.state.cfg.nmp_depth.value = value.parse::<u8>().unwrap(),
            "NMPMargin" => self.search.state.cfg.nmp_margin.value = value.parse::<u8>().unwrap(),
            "NMPDivisor" => self.search.state.cfg.nmp_divisor.value = value.parse::<u8>().unwrap(),
            "NMPDivisorImproving" => {
                self.search.state.cfg.nmp_divisor_improving.value = value.parse::<u8>().unwrap()
            }
            "LMPMoveMargin" => {
                self.search.state.cfg.lmp_move_margin.value = value.parse::<usize>().unwrap()
            }
            "LMPDepthFactor" => {
                self.search.state.cfg.lmp_depth_factor.value = value.parse::<u8>().unwrap()
            }
            "LMRDepth" => self.search.state.cfg.lmr_depth.value = value.parse::<u8>().unwrap(),
            "LMRMoveMargin" => {
                self.search.state.cfg.lmr_move_margin.value = value.parse::<usize>().unwrap()
            }
            "LMRQuietMargin" => {
                self.search.state.cfg.lmr_quiet_margin.value = value.parse::<f64>().unwrap()
            }
            "LMRQuietDivisor" => {
                self.search.state.cfg.lmr_quiet_divisor.value = value.parse::<f64>().unwrap()
            }
            "LMRBaseMargin" => {
                self.search.state.cfg.lmr_base_margin.value = value.parse::<f64>().unwrap()
            }
            "LMRBaseDivisor" => {
                self.search.state.cfg.lmr_base_divisor.value = value.parse::<f64>().unwrap()
            }
            "MOTTEntryValue" => {
                self.search.state.cfg.mo_tt_entry_value.value = value.parse::<i32>().unwrap()
            }
            "MOCaptureValue" => {
                self.search.state.cfg.mo_capture_value.value = value.parse::<i32>().unwrap()
            }
            "MOKillerValue" => {
                self.search.state.cfg.mo_killer_value.value = value.parse::<i32>().unwrap()
            }
            "TCTimeDivisor" => {
                self.search.state.cfg.tc_time_divisor.value = value.parse::<u64>().unwrap()
            }
            "TCElapsedFactor" => {
                self.search.state.cfg.tc_elapsed_factor.value = value.parse::<i64>().unwrap()
            }

            _ => Logger::log(&format!("info string unknown option: {}", name)),
        }
    }

    fn handle_go_infinite(&mut self, tokens: &mut Queue<&str>) {
        self.search.state.params.depth = u8::MAX;
        self.search.state.tc.time_mode = TimeMode::Infinite;

        self.handle_go(tokens);
    }

    fn handle_ucinewgame(&mut self) {
        self.search.state.game = Chess::default();
    }

    fn handle_isready(&self) {
        Logger::log("readyok");
    }

    fn handle_quit(&self) {
        std::process::exit(0);
    }

    fn handle_uci(&self) {
        Logger::log(r#"id name Pluto"#);
        Logger::log(r#"id author CastledChess"#);

        Logger::log(format!("{}", self.search.state.cfg.move_overhead).as_str());
        Logger::log(format!("{}", self.search.state.cfg.threads).as_str());
        Logger::log(format!("{}", self.search.state.cfg.hash).as_str());

        // Values to tune
        Logger::log(format!("{}", self.search.state.cfg.qsearch_depth).as_str());
        Logger::log(format!("{}", self.search.state.cfg.rfp_depth).as_str());
        Logger::log(format!("{}", self.search.state.cfg.rfp_base_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.rfp_reduction_improving).as_str());
        Logger::log(format!("{}", self.search.state.cfg.fp_base_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.fp_depth_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.fp_margin_depth_factor).as_str());
        Logger::log(format!("{}", self.search.state.cfg.nmp_depth).as_str());
        Logger::log(format!("{}", self.search.state.cfg.nmp_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.nmp_divisor).as_str());
        Logger::log(format!("{}", self.search.state.cfg.nmp_divisor_improving).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmp_move_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmp_depth_factor).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmr_depth).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmr_move_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmr_quiet_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmr_quiet_divisor).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmr_base_margin).as_str());
        Logger::log(format!("{}", self.search.state.cfg.lmr_base_divisor).as_str());
        Logger::log(format!("{}", self.search.state.cfg.mo_tt_entry_value).as_str());
        Logger::log(format!("{}", self.search.state.cfg.mo_capture_value).as_str());
        Logger::log(format!("{}", self.search.state.cfg.mo_killer_value).as_str());
        Logger::log(format!("{}", self.search.state.cfg.tc_time_divisor).as_str());
        Logger::log(format!("{}", self.search.state.cfg.tc_elapsed_factor).as_str());

        Logger::log(r#"uciok"#);
    }
}
