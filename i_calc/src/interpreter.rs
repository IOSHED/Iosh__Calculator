
use std::{f64::consts::{PI, E}, collections::BTreeMap};

use crate::{
    ast::{expr::{Expr, Evaluatable}, calc::Calc}, 
    errors::CalcErrors, 
};

pub struct Interpreter<'input> {
    pub request_history: BTreeMap<String, Result<f64, CalcErrors>>,
    pub variables: BTreeMap<String, f64>,
    pub constants: BTreeMap<&'input str, f64>,
}

impl<'input> Interpreter<'input> {
    
    pub fn new() -> Self {
        
        let speed_light: f64 = 299792458.0;  // СКОРОСТЬ СВЕТА
        let acceleration_free_fall: f64 = 9.80665;  // СКОРОСТЬ СВОБОДНОГО ПАДЕНИЯ
        let gravitational_constant: f64 = 0.0000000000066720;  // ГРАВИТАЦИОННАЯ ПОСТОЯННАЯ 

        let constants = BTreeMap::from([
            ("PI", PI),
            ("E", E),
            ("c", speed_light),
            ("g", acceleration_free_fall),
            ("G", gravitational_constant)
        ]);

        Interpreter {
            request_history: BTreeMap::new(),
            variables: BTreeMap::new(),
            constants
        }
    }

    pub fn eval(&mut self, calc: Calc, input: &str) -> Result<Option<f64>, CalcErrors> {
        match calc {
            Calc::InitVariable(name, expr) => {
                match self.init_variable(name, &expr) {
                    Some(err) => Err(err),
                    None => Ok(None),
                }
            },
            Calc::Expr(expr) => {
                match self.eval_expr(&expr, input) {
                    Ok(result) => Ok(Some(result)),
                    Err(err) => Err(err),
                }
            },
        }
    }

    pub fn get_request_history(&self, to: usize) -> Vec<(String, Result<f64, CalcErrors>)> {
        self.request_history
            .iter()
            .rev()  
            .take(to) 
            .rev()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn eval_expr(&mut self, expr: &Expr, input: &str) -> Result<f64, CalcErrors> {
        if let Some(result) = self.request_history.get(input) {
            return *result;
        }

        let result = expr.evaluate(self)?;
        self.request_history.insert(String::from(input), Ok(result));
        Ok(result)
    }
    
    fn init_variable(&mut self, name: &str, expr: &Box<Expr>) -> Option<CalcErrors> {

        if self.constants.get(name).is_some() {
            return Some(CalcErrors::CannotCreateVariablesWithNameConstant);
        }

        match expr.evaluate(self) {
            Ok(result) => self.add_or_change_variable(name, result),
            Err(err) => Some(err),
        }
    }

    fn add_or_change_variable(&mut self, name: &str, result: f64) -> Option<CalcErrors> {
        self.variables.entry(name.to_string()).and_modify(|v| *v = result).or_insert(result);
        None
    }
}
