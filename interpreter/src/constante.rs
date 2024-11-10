use serde::{Deserialize, Serialize};

use crate::traits::GetResult;

#[derive(Deserialize, Serialize)]
pub struct Constant {
    name: String,
    value: f64,
}

impl Constant {
    pub fn new(name: &str, value: f64) -> Self {
        Constant {
            name: name.to_string(),
            value,
        }
    }
}

impl GetResult<Option<f64>> for Vec<Constant> {
    fn get_result(&self, input: &str) -> Option<f64> {
        self.iter()
            .find(|history| history.name == input)
            .map(|history| history.value)
    }
}
