use serde::{Serialize, Deserialize};

use crate::{errors::CalcErrors, traits::{GetResult, RemoveElementIfMaxValue}};


#[derive(Deserialize, Serialize, Clone)]
pub struct History {
    pub input: String,
    pub result: Result<f64, CalcErrors>,
}

impl History {
    pub fn new(input: &str, result: Result<f64, CalcErrors>) -> Self {
        History { input: input.to_string(), result }
    }
}

impl GetResult<Option<Result<f64, CalcErrors>>> for Vec<History> {
    fn get_result(&self, input: &str) -> Option<Result<f64, CalcErrors>> {
        self.iter()
            .find(|history| history.input == input)
            .map(|history| history.result)
    }
}

impl RemoveElementIfMaxValue for Vec<History> {
    fn remove_element_if_max_value(&mut self, max_value: usize) -> () {
        if self.len() > max_value {
            self.remove(0);
        }
    }
}
