use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::load()));
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path_file_help: String,
    pub commands: Commands,
    pub output_line_history: usize,
    pub max_size_history: usize,
    pub max_number_variable: usize,
}

impl From<Config> for interpreter::config::Config {
    fn from(value: Config) -> Self {
        interpreter::config::Config::new(value.max_size_history, value.max_number_variable)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Commands {
    pub end: String,
    pub help: String,
    pub history: String,
    pub empty_input: String,
}

impl Config {
    #[must_use]
    pub fn load() -> Self {
        let file = File::open("./configurate/user.json")
            .expect("Failed to open file `/configurate/user.json`. Check its availability.");
        let reader = BufReader::new(file);
        let config: Config =
            serde_json::from_reader(reader).expect("Failed to parse config. Check for all fields.");
        config
    }

    #[must_use]
    pub fn get() -> Arc<Mutex<Config>> {
        INSTANCE.clone()
    }

    #[must_use]
    pub fn get_config_for_interpreter(&self) -> interpreter::config::Config {
        interpreter::config::Config::new(self.max_size_history, self.max_number_variable)
    }
}
