use serde_derive::Deserialize;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub qsearch_depth: u8,
    pub rfp_depth: u8,
    pub rfp_depth_multiplier: i32,
    pub tt_size: usize,
    pub mo_tt_entry_value: i32,
    pub mo_capture_value: i32,
    pub mo_killer_move_value: i32,
    pub nb_killer_moves: usize,
    pub max_depth_killer_moves: usize,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let filename = "../Config.toml";
        let bytes = include_bytes!("../Config.toml");

        let contents = String::from_utf8_lossy(bytes).to_string();

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };

        Ok(data.config)
    }
}
