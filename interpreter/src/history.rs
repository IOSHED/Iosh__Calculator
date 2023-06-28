use serde::{Serialize, Deserialize};

use crate::{errors::CalcError, traits::{GetResult, RemoveElementIfMaxValue}};


#[derive(Deserialize, Serialize, Clone)]
pub struct History {
    pub input: String,
    pub result: Result<f64, CalcError>,
}

impl History {
    pub fn new(input: &str, result: Result<f64, CalcError>) -> Self {
        History { input: input.to_string(), result }
    }
}

impl GetResult<Option<Result<f64, CalcError>>> for Vec<History> {
    fn get_result(&self, input: &str) -> Option<Result<f64, CalcError>> {
        self.iter()
            .find(|history| history.input == input)
            .map(|history| history.result.clone())
    }
}

impl RemoveElementIfMaxValue for Vec<History> {
    fn remove_element_if_max_value(&mut self, max_value: usize) -> () {
        if self.len() > max_value {
            self.remove(0);
        }
    }
}
