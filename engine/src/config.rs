//! Configuration module for the chess engine.
//! Handles loading and parsing of engine settings from TOML configuration file.

use serde_derive::Deserialize;
use std::{process::exit, usize};
/// Wrapper structure for the TOML configuration data.
/// Used for deserializing the configuration file.
#[derive(Deserialize)]
pub struct Data {
    /// The actual configuration settings
    config: Config,
}

/// Main configuration structure containing engine parameters.
#[derive(Deserialize)]
pub struct Config {
    pub tt_size: usize,
    pub qsearch_depth: u8,
    pub rfp_depth: u8,
    pub rfp_base_margin: i32,
    pub nmp_depth: u8,
    pub nmp_margin: u8,
    pub nmp_divisor: u8,
    pub lmp_move_margin: usize,
    pub lmp_depth_factor: u8,
    pub lmr_depth: u8,
    pub lmr_move_margin: usize,
    pub lmr_quiet_margin: f64,
    pub lmr_quiet_divisor: f64,
    pub lmr_base_margin: f64,
    pub lmr_base_divisor: f64,
    pub mo_tt_entry_value: i32,
    pub mo_capture_value: i32,
    pub mo_killer_value: i32,
    pub tc_time_divisor: u64,
    pub tc_elapsed_factor: i64,
}

impl Config {
    /// Loads configuration from the TOML file.
    ///
    /// Attempts to read and parse the Config.toml file located in the parent directory.
    /// The file is included at compile time using `include_bytes!`.
    ///
    /// # Returns
    /// * `Ok(Config)` - Successfully loaded configuration
    /// * `Err(String)` - Error message if loading fails
    ///
    /// # Panics
    /// Exits the program with status code 1 if the TOML file cannot be parsed
    pub fn load() -> Result<Self, String> {
        let filename = "../Config.toml";
        let bytes = include_bytes!("../Config.toml");

        let contents = String::from_utf8_lossy(bytes).to_string();

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Unable to load data from `{}` {:?}", filename, e);
                exit(1);
            }
        };

        Ok(data.config)
    }
}
