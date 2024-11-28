use serde_derive::Deserialize;
use std::fs;
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
}

impl Config {
    pub fn load(filename: &str) -> Result<Self, String> {
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", filename);
                exit(1);
            }
        };

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