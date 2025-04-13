use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::f64::consts::{E, PI};

use crate::traits::{GetElementByName, GetResult, RemoveElementIfMaxValue};
use crate::{
    ast::{
        calc::Calc,
        expr::{Evaluatable, Expr},
    },
    config::Config,
    constante::Constant,
    errors::CalcError,
    history::History,
    variable::Variable,
};

#[derive(Deserialize, Serialize)]
pub struct Interpreter {
    pub request_history: Vec<History>,
    pub variables: Vec<Variable>,
    pub constants: Vec<Constant>,
    pub config: Config,
}

impl Interpreter {
    #[must_use]
    pub fn new(config: Config) -> Result<Self, CalcError> {
        let speed_light: Decimal = 299_792_458.0.try_into().map_err(|_| CalcError::MathError)?; // СКОРОСТЬ СВЕТА
        let acceleration_free_fall: Decimal =
            9.80665.try_into().map_err(|_| CalcError::MathError)?; // СКОРОСТЬ СВОБОДНОГО ПАДЕНИЯ
        let gravitational_constant: Decimal = 0.000_000_000_006_672_0
            .try_into()
            .map_err(|_| CalcError::MathError)?; // ГРАВИТАЦИОННАЯ ПОСТОЯННАЯ
        let pi: Decimal = PI.try_into().unwrap();
        let e: Decimal = E.try_into().unwrap();

        let constants = vec![
            Constant::new("PI", pi),
            Constant::new("E", e),
            Constant::new("c", speed_light),
            Constant::new("g", acceleration_free_fall),
            Constant::new("G", gravitational_constant),
        ];

        Ok(Interpreter {
            request_history: Vec::with_capacity(config.max_size_history),
            variables: Vec::with_capacity(config.max_size_history),
            constants,
            config,
        })
    }

    pub fn eval(&mut self, calc: Calc, input: &str) -> Result<Option<Decimal>, CalcError> {
        match calc {
            Calc::InitVariable(name, expr) => match self.init_variable(name, *expr) {
                Some(err) => Err(err),
                None => Ok(None),
            },
            Calc::Expr(expr) => match self.eval_expr(&expr, input) {
                Ok(result) => Ok(Some(result)),
                Err(err) => Err(err),
            },
        }
    }

    #[must_use]
    pub fn get_request_history(&self, to: usize) -> Vec<(String, Result<Decimal, CalcError>)> {
        self.request_history
            .iter()
            .rev()
            .take(to)
            .map(|history| (history.input.clone(), history.result.clone()))
            .collect()
    }

    fn eval_expr(&mut self, expr: &Expr, input: &str) -> Result<Decimal, CalcError> {
        let result = expr.evaluate(self)?;
        self.insert_history(input, result);
        Ok(result)
    }

    fn init_variable(&mut self, name: &str, expr: Expr) -> Option<CalcError> {
        if self.constants.get_result(name).is_some() {
            return Some(CalcError::CannotCreateVariablesWithNameConstant);
        }

        match expr.evaluate(self) {
            Ok(result) => self.add_or_change_variable(name, result),
            Err(err) => Some(err),
        }
    }

    fn add_or_change_variable(&mut self, name: &str, result: Decimal) -> Option<CalcError> {
        if let Some(variable) = self.variables.get_element_by_name(name) {
            if variable.value == result {
                return None;
            }
            variable.value = result;
            return None;
        }
        self.variables
            .remove_element_if_max_value(self.config.max_number_variable);
        self.variables.push(Variable::new(name.to_string(), result));
        None
    }

    fn insert_history(&mut self, input: &str, result: Decimal) {
        self.request_history
            .remove_element_if_max_value(self.config.max_size_history);
        self.request_history.push(History::new(input, Ok(result)));
    }
}
