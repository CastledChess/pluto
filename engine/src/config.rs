//! Configuration module for the chess engine.
//! Handles loading and parsing of engine settings from TOML configuration file.

use serde_derive::Deserialize;
use std::process::exit;
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
    /// Maximum depth for quiescence search in plies
    pub qsearch_depth: u8,
    /// Base depth for Reverse Futility Pruning (RFP)
    pub rfp_depth: u8,
    /// Multiplier applied to RFP depth calculations
    pub rfp_depth_multiplier: i32,
    /// Size of the transposition table in bytes
    pub tt_size: usize,
    /// Value threshold for move ordering using transposition table entries
    pub mo_tt_entry_value: i32,
    /// Value bonus applied to captures during move ordering
    pub mo_capture_value: i32,
    pub mo_killer_move_value: i32,
    pub nb_killer_moves: usize,
    pub max_depth_killer_moves: usize,
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
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };

        Ok(data.config)
    }
}

