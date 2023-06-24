use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub max_size_history: usize,
    pub max_number_variable: usize,
}

impl Config {
    pub fn new(max_size_history: usize, max_number_variable: usize) -> Self {
        Config { max_size_history, max_number_variable }
    }
}