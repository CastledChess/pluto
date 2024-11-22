use queues::{queue, IsQueue, Queue};
use std::ptr::null;

pub trait UciParser {
    fn parse_command(&self, command: &str);
    fn parse_tokens(&self, tokens: &mut Queue<&str>);
    fn handle_go(&self, tokens: &mut Queue<&str>);
    fn handle_btime(&self, tokens: &mut Queue<&str>);
    fn handle_wtime(&self, tokens: &mut Queue<&str>);
    fn handle_go_depth(&self, tokens: &mut Queue<&str>);
    fn handle_go_movetime(&self, tokens: &mut Queue<&str>);
    fn handle_position(&self, tokens: &mut Queue<&str>);
    fn handle_position_startpos(&self, tokens: &mut Queue<&str>);
    fn handle_position_fen(&self, tokens: &mut Queue<&str>);
    fn handle_setoption(&self, tokens: &mut Queue<&str>);
    fn handle_go_infinite(&self, tokens: &mut Queue<&str>);
    fn handle_ucinewgame(&self);
    fn handle_isready(&self);
    fn handle_quit(&self);
    fn handle_uci(&self);
}

pub struct Uci();

impl UciParser for Uci {
    fn parse_command(&self, command: &str) {
        let tokens_vec: Vec<&str> = command.split_whitespace().collect();
        let mut tokens: Queue<&str> = queue![];

        for token in tokens_vec {
            tokens.add(token).unwrap();
        }

        self.parse_tokens(&mut tokens);
    }

    fn parse_tokens(&self, tokens: &mut Queue<&str>) {
        let first_token = tokens.remove().unwrap();

        match first_token {
            "uci" => println!("uci"),
            "isready" => println!("isready"),
            "quit" => println!("quit"),
            "setoption" => println!("setoption"),
            "ucinewgame" => println!("ucinewgame"),
            "position" => println!("position"),
            "go" => println!("go"),
            _ => println!("Unknown command: {}", first_token),
        }
    }

    fn handle_go(&self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();

        match token {
            "btime" => self.handle_btime(tokens),
            "wtime" => self.handle_wtime(tokens),
            "depth" => self.handle_go_depth(tokens),
            "movetime" => self.handle_go_movetime(tokens),
            "infinite" => self.handle_go_infinite(tokens),
            _ =>
            // search.go();
            {
                println!("Unknown go command: {}", token);
            }
        }
    }

    fn handle_btime(&self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u64>().unwrap();

        // search.max_depth = 1000;
        // search.time_control = worbtime;
        // search.btime = time;

        self.handle_go(tokens);
    }

    fn handle_wtime(&self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u64>().unwrap();

        // search.max_depth = 1000;
        // search.time_control = worbtime;
        // search.wtime = time;

        self.handle_go(tokens);
    }

    fn handle_go_depth(&self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let depth = token.parse::<u64>().unwrap();

        // search.max_depth = depth;
        // search.time_control = none;

        self.handle_go(tokens);
    }

    fn handle_go_movetime(&self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u64>().unwrap();

        // search.max_depth = 1000;
        // search.time_control = movetime;
        // search.movetime = time;

        self.handle_go(tokens);
    }

    fn handle_position(&self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();

        match token {
            "startpos" => self.handle_position_startpos(tokens),
            "fen" => self.handle_position_fen(tokens),
            _ => println!("Unknown position command: {}", token),
        }
    }

    fn handle_position_startpos(&self, tokens: &mut Queue<&str>) {
        // search.position = startpos;

        let token = tokens.remove().unwrap();

        if token == "moves" {
            while (tokens.size() > 0) {
                let move_str = tokens.remove().unwrap();
                let uci_move = move_str.parse();

                // search.position.make_move(uci_move);
            }
        }
    }

    fn handle_position_fen(&self, tokens: &mut Queue<&str>) {
        let fen = tokens.remove().unwrap();

        // search.position = fen(fen);

        let token = tokens.remove().unwrap();

        if token == "moves" {
            while (tokens.size() > 0) {
                let move_str = tokens.remove().unwrap();
                let uci_move = move_str.parse();

                // search.position.make_move(uci_move);
            }
        }
    }

    fn handle_setoption(&self, tokens: &mut Queue<&str>) {
        tokens.remove().unwrap(); // name
        let name = tokens.remove().unwrap();
        tokens.remove().unwrap(); // value
        let value = tokens.remove().unwrap();

        if (name == null() || value == null()) {
            return;
        }

        match name {
            "MoveOverhead" => println!("info string set move overhead"), // search.move_overhead = value.parse();
            _ => println!("info string unknown option: {}", name),
        }
    }

    fn handle_go_infinite(&self, tokens: &mut Queue<&str>) {
        // search.max_depth = 1000;
        // search.time_control = infinite;

        self.handle_go(tokens);
    }

    fn handle_ucinewgame(&self) {
        // search.position = startpos;
        // search.clear();
    }

    fn handle_isready(&self) {
        // wait for engine to be ready

        println!("readyok");
    }

    fn handle_quit(&self) {
        std::process::exit(0);
    }

    fn handle_uci(&self) {
        println!("id name CastledEngine");
        println!("id author CastledChess");

        // TODO: print all options
        // for option in vec![
        //     "MoveOverhead",
        //     "UCI_Chess960",
        //     "UCI_AnalyseMode",
        //     "UCI_LimitStrength",
        //     "UCI_Elo",
        //     "UCI_ShowWDL",
        //     "UCI_ShowCurrLine",
        //     "UCI_ShowRefutations
        // "].iter() {
        //     println!("option name {} type spin default 0", option);
        // }
    }
}
