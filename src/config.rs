use std::sync::{Arc, Mutex};
use std::{fs::File, io::BufReader};
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::load()));
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path_file_help: String,
    pub command: Command,
    pub output_line_history: usize,
    pub max_size_history: usize,
    pub max_number_variable: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub end: String,
    pub help: String,
    pub history: String,
    pub empty_input: String,
}

impl Config {
    pub fn load() -> Self {
        let file = File::open("config.json")
            .expect("Failed to open file config.json. Check its availability.");
        let reader = BufReader::new(file);
        let config: Config =
            serde_json::from_reader(reader).expect("Failed to parse config. Check for all fields.");
        config
    }

    pub fn get() -> Arc<Mutex<Config>> {
        INSTANCE.clone()
    }

    pub fn get_config_for_interpreter(&self) -> interpreter::config::Config {
        interpreter::config::Config::new(
            self.max_size_history.clone(),
            self.max_number_variable.clone(),
        )
    }
}
