use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    errors::CalcError,
    interpreter::Interpreter,
    traits::{GetResult, RemoveElementIfMaxValue},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct History {
    pub input: String,
    pub result: Result<Decimal, CalcError>,
}

impl History {
    //

    #[must_use]
    pub fn new(input: &str, result: Result<Decimal, CalcError>) -> Self {
        History {
            input: input.to_string(),
            result,
        }
    }

    /// Находит минимум между историей `Interpreter` и `to`.
    /// Используется, для того чтобы пользователь не попытался вывеести больше записей истории,
    /// Чем есть на самом деле.
    ///
    /// * `interpreter` - интерпритатор.
    /// * `to` - минимальное возвращаемое значение.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// let mut interpreter = Interpreter::new(Config::default());
    /// interpreter.request_history = vec![
    ///     Hisrory::new("2 - 3", Some(-1.0)),
    ///     History::new("2 4", Some(8.0)),
    /// ];
    ///
    /// assert_eq!(check_len_history(&interpreter, 1), 1);
    /// assert_eq!(check_len_history(&interpreter, 5), 2);
    /// ```
    #[must_use]
    pub fn get_len_history(interpreter: &Interpreter, to: usize) -> usize {
        interpreter.request_history.len().min(to)
    }
}

impl GetResult<Option<Result<Decimal, CalcError>>> for Vec<History> {
    fn get_result(&self, input: &str) -> Option<Result<Decimal, CalcError>> {
        self.iter()
            .find(|history| history.input == input)
            .map(|history| history.result.clone())
    }
}

impl RemoveElementIfMaxValue for Vec<History> {
    fn remove_element_if_max_value(&mut self, max_value: usize) {
        if self.len() > max_value {
            self.remove(0);
        }
    }
}
