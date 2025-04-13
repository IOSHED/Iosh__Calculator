use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::traits::{GetElementByName, GetResult, RemoveElementIfMaxValue};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub value: Decimal,
}

impl Variable {
    pub fn new(name: String, value: Decimal) -> Self {
        Variable { name, value }
    }
}

impl GetResult<Option<Decimal>> for Vec<Variable> {
    fn get_result(&self, input: &str) -> Option<Decimal> {
        self.iter()
            .find(|variable| variable.name == input)
            .map(|variable| variable.value)
    }
}

impl RemoveElementIfMaxValue for Vec<Variable> {
    fn remove_element_if_max_value(&mut self, max_value: usize) {
        if self.len() > max_value {
            self.remove(0);
        }
    }
}

impl<'a> GetElementByName<'a, Option<&'a mut Variable>> for Vec<Variable> {
    fn get_element_by_name(&'a mut self, name: &str) -> Option<&'a mut Variable> {
        self.iter_mut().find(|variable| variable.name == name)
    }
}
