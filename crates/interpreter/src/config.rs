use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub max_size_history: usize,
    pub max_number_variable: usize,
}

impl Config {
    #[must_use]
    pub fn new(max_size_history: usize, max_number_variable: usize) -> Self {
        Config {
            max_size_history,
            max_number_variable,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_size_history: 50,
            max_number_variable: 50,
        }
    }
}
