//! UCI (Universal Chess Interface) protocol implementation module.
//! Handles communication between the chess engine and UCI-compatible chess GUIs.

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

/// UCI option types supported by the engine.
#[allow(dead_code)]
enum UciOptionType {
    /// Boolean option (true/false)
    Check,
    /// Integer option with minimum and maximum bounds
    Spin,
    /// Option with predefined choices
    Combo,
    /// Action trigger without value
    Button,
    /// Text input option
    String,
}

/// Represents a configurable UCI option with its properties.
struct UciOption {
    /// Name identifier of the option
    name: String,
    /// Default value as string representation
    default: String,
    /// Type of the option (determines how it's handled)
    option_type: UciOptionType,
    /// Minimum allowed value for numeric options
    min: i32,
    /// Maximum allowed value for numeric options
    max: i32,
}

/// Main UCI protocol handler implementing the Universal Chess Interface.
pub struct UciController {
    search: Search,
    /// Available configuration options for the engine
    options: Vec<UciOption>,
}

impl Default for UciController {
    /// Creates a new UCI instance with default settings.
    fn default() -> UciController {
        UciController {
            search: Search::new(),
            options: vec![
                UciOption {
                    name: "Threads".to_string(),
                    default: "1".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 1,
                    max: 1,
                },
                UciOption {
                    name: "Hash".to_string(),
                    default: "200".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 0,
                    max: 1024,
                },
                UciOption {
                    name: "Qself.search.state.epth".to_string(),
                    default: "2".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 0,
                    max: 10,
                },
                UciOption {
                    name: "RFPDepth".to_string(),
                    default: "7".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 1,
                    max: 10,
                },
                UciOption {
                    name: "RFPDepthMultiplier".to_string(),
                    default: "50".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 0,
                    max: 100,
                },
                UciOption {
                    name: "MOTTEntryValue".to_string(),
                    default: "200".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 50,
                    max: 350,
                },
                UciOption {
                    name: "MOCaptureValue".to_string(),
                    default: "50".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 10,
                    max: 100,
                },
                UciOption {
                    name: "MOKillerMoveValue".to_string(),
                    default: "100".to_string(),
                    option_type: UciOptionType::Spin,
                    min: 50,
                    max: 200,
                },
            ],
        }
    }
}

impl UciController {
    /// Parses a UCI command string and processes it.
    ///
    /// # Arguments
    /// * `command` - String containing the UCI command to process
    pub fn parse_command(&mut self, command: &str) {
        let tokens_vec: Vec<&str> = command.split_whitespace().collect();
        let mut tokens: Queue<&str> = queue![];

        for token in tokens_vec {
            tokens.add(token).unwrap();
        }

        self.parse_tokens(&mut tokens);
    }

    /// Processes a queue of command tokens.
    ///
    /// # Arguments
    /// * `tokens` - Queue of command tokens to process
    fn parse_tokens(&mut self, tokens: &mut Queue<&str>) {
        let first_token = tokens.remove().unwrap();

        self.search.state.tc.time_mode = TimeMode::Infinite;
        self.search.state.params.depth = u8::MAX;

        match first_token {
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
            self.search.state.params.depth = 5;
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

    /// Handles the 'go' command with various self.search.state.parameters.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing self.search.state.parameters
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

    /// Sets up timing parameters for black.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing time value in milliseconds
    fn handle_btime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u128>().unwrap();

        self.search.state.params.depth = u8::MAX;
        self.search.state.tc.time_mode = TimeMode::WOrBTime;
        self.search.state.params.b_time = time;

        self.handle_go(tokens);
    }

    /// Sets up timing parameters for white.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing time value in milliseconds
    fn handle_wtime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u128>().unwrap();

        self.search.state.params.depth = u8::MAX;
        self.search.state.tc.time_mode = TimeMode::WOrBTime;
        self.search.state.params.w_time = time;

        self.handle_go(tokens);
    }

    /// Sets up self.search.state.with fixed depth.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing depth value in plies
    fn handle_go_depth(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let depth = token.parse::<u8>().unwrap();

        self.search.state.params.depth = depth;
        self.search.state.tc.time_mode = TimeMode::Infinite;

        self.handle_go(tokens);
    }

    /// Sets up self.search.state.with fixed time per move.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing time value in milliseconds
    fn handle_go_movetime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u128>().unwrap();

        self.search.state.params.move_time = time;
        self.search.state.tc.time_mode = TimeMode::MoveTime;
        self.search.state.params.depth = u8::MAX;

        self.handle_go(tokens);
    }

    /// Processes position setup commands.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing position type and moves
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

    /// Sets up the initial chess position and applies moves.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing moves to apply
    fn handle_position_startpos(&mut self, tokens: &mut Queue<&str>) {
        self.search.state.game = Chess::default();
        self.search.state.history.clear();

        if let Ok(moves) = tokens.remove() {
            if moves != "moves" {
                return;
            }

            while let Ok(move_str) = tokens.remove() {
                let uci_move = move_str.parse::<UciMove>().ok();
                let game = self.search.state.game.clone();
                let legal = uci_move.unwrap().to_move(&game).ok().unwrap();

                self.search.state.game = game.play(&legal).unwrap();
                self.search.state.history.push(
                    self.search
                        .state
                        .game
                        .zobrist_hash(shakmaty::EnPassantMode::Legal),
                )
            }
        }

        self.search.state.nnue = NNUEState::from_board(self.search.state.game.board());
    }

    /// Sets up a position from FEN string and applies moves.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing FEN string and moves
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
        self.search.state.history.clear();

        if token == "moves" {
            while let Ok(move_str) = tokens.remove() {
                let uci_move = move_str.parse::<UciMove>().ok();
                let game = self.search.state.game.clone();
                let legal = uci_move.unwrap().to_move(&game).ok().unwrap();

                self.search.state.game = game.play(&legal).unwrap();
                self.search.state.history.push(
                    self.search
                        .state
                        .game
                        .zobrist_hash(shakmaty::EnPassantMode::Legal),
                )
            }
        }

        self.search.state.nnue = NNUEState::from_board(self.search.state.game.board());
    }

    /// Processes option setting commands.
    ///
    /// # Arguments
    /// * `tokens` - Queue containing option name and value
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
            // TODO: Implement option handling
            "MoveOverhead" => Logger::log("info string set move overhead"),
            "Threads" => Logger::log("info string set threads"),
            "Hash" => {
                let size = value.parse::<u32>().unwrap();
                let bytes = size * 1024 * 1024;
                let entries = bytes / 24; // 24 is the actual size of one entry

                self.search.state.tt = TranspositionTable::new(entries as usize);
            }
            "QSearchDepth" => self.search.state.cfg.qsearch_depth = value.parse::<u8>().unwrap(),
            "RFPDepth" => self.search.state.cfg.rfp_depth = value.parse::<u8>().unwrap(),
            "RFPDepthMultiplier" => {
                self.search.state.cfg.rfp_depth_multiplier = value.parse::<i32>().unwrap()
            }
            "MOTTEntryValue" => {
                self.search.state.cfg.mo_tt_entry_value = value.parse::<i32>().unwrap()
            }
            "MOCaptureValue" => {
                self.search.state.cfg.mo_capture_value = value.parse::<i32>().unwrap()
            }
            "MOKillerMoveValue" => {
                self.search.state.cfg.mo_killer_move_value = value.parse::<i32>().unwrap()
            }
            _ => Logger::log(&format!("info string unknown option: {}", name)),
        }
    }

    /// Sets up infinite self.search.state.mode.
    ///
    /// # Arguments
    /// * `tokens` - Queue of remaining tokens to process
    fn handle_go_infinite(&mut self, tokens: &mut Queue<&str>) {
        self.search.state.params.depth = u8::MAX;
        self.search.state.tc.time_mode = TimeMode::Infinite;

        self.handle_go(tokens);
    }

    /// Resets the game to initial position.
    fn handle_ucinewgame(&mut self) {
        self.search.state.game = Chess::default();
    }

    /// Responds to isready command.
    fn handle_isready(&self) {
        Logger::log("readyok");
    }

    /// Handles quit command by exiting the program.
    fn handle_quit(&self) {
        std::process::exit(0);
    }

    /// Sends engine identification and available options.
    fn handle_uci(&self) {
        Logger::log(r#"id name Pluto"#);
        Logger::log(r#"id author CastledChess"#);

        for option in &self.options {
            let type_str = match option.option_type {
                UciOptionType::Check => "check",
                UciOptionType::Spin => "spin",
                UciOptionType::Combo => "combo",
                UciOptionType::Button => "button",
                UciOptionType::String => "string",
            };

            Logger::log(&format!(
                "option name {} type {} default {} min {} max {}",
                option.name, type_str, option.default, option.min, option.max
            ));
        }

        Logger::log(r#"uciok"#);
    }
}
