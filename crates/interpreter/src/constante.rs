use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::traits::GetResult;

#[derive(Deserialize, Serialize)]
pub struct Constant {
    name: String,
    value: Decimal,
}

impl Constant {
    pub fn new(name: &str, value: Decimal) -> Self {
        Constant {
            name: name.to_string(),
            value,
        }
    }
}

impl GetResult<Option<Decimal>> for Vec<Constant> {
    fn get_result(&self, input: &str) -> Option<Decimal> {
        self.iter()
            .find(|history| history.name == input)
            .map(|history| history.value)
    }
}
