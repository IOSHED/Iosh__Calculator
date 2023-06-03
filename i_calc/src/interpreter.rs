
use std::{f64::consts::{PI, E}, collections::BTreeMap};
use crate::{
    ast::{Expr, Opcode, FuncName, Calc}, 
    errors::CalcErrors, 
    constante::{SPEED_LIGHT, ACCELERATION_FREE_FALL, GRAVITATIONAL_CONSTANT}
};


pub struct Interpreter<'input> {
    request_historys: BTreeMap<String, Result<f64, CalcErrors>>,
    variables: BTreeMap<String, f64>,
    constante: BTreeMap<&'input str, f64>,
}


impl<'input> Interpreter<'input> {

    pub fn new() -> Self {
        let constante = BTreeMap::from([
            ("PI", PI),
            ("E", E),
            ("c", SPEED_LIGHT), 
            ("g", ACCELERATION_FREE_FALL),
            ("G", GRAVITATIONAL_CONSTANT)
        ]);

        Interpreter { 
            request_historys: BTreeMap::new(),
            variables: BTreeMap::new(),
            constante, 
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
                    Ok(r) => Ok(Some(r)),
                    Err(err) => Err(err),
                }
            },
        }
    }


    pub fn get_request_historys(&self, to: usize) -> Vec<(String, Result<f64, CalcErrors>)> {

        let vector_requers_history: Vec<(String, Result<f64, CalcErrors>)> = self.request_historys
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let mut result_vec = Vec::new();

        for i in 0..to {
            if i >= vector_requers_history.len() {
                break;
            } else {
                let new_index = vector_requers_history[i].clone();
                result_vec.push(new_index);
            }
        }
        result_vec
    }


    fn eval_expr(&mut self, expr: &Expr, input: &str) -> Result<f64, CalcErrors> {

        if let Some(result) = self.request_historys.get(input) {
            *result
        } else {
            let result = self.match_eval(expr);
            self.request_historys.insert(String::from(input), result);
            result
        }
    }
 

    fn init_variable(&mut self, name: &str, expr: &Box<Expr>) -> Option<CalcErrors> {

        if None == self.constante.get(name) {
            
            match self.match_eval(expr) {
                Ok(res) => {
                    self.variables.insert(String::from(name), res);
                    return None
                },
                Err(err) => return Some(err),
            }
            
        }
        Some(CalcErrors::CannotCreateVariablesWithNameConstant)
    }


    fn match_eval(&mut self, expr: &Expr) -> Result<f64, CalcErrors> {

        match expr {

            Expr::Number(n) => Ok(*n),

            Expr::Func(name, expr) => self.match_func(name, expr),

            Expr::Variable(name) => self.match_variable(name),

            Expr::Op(left, op, right) => self.match_op(left, op, right),

            Expr::Error(err) => Err(*err),
        }
    }


    fn match_variable(&self, name: &str) -> Result<f64, CalcErrors> {

        match self.variables.get(name) {
            Some(n) => Ok(*n),
            None => match self.constante.get(name) {
                Some(n) => Ok(*n),
                None => Err(CalcErrors::CallingNonexistentVariable),
            } 
        }
    }


    fn match_op(&mut self, left: &Box<Expr>, op: &Opcode, right: &Box<Expr>) -> Result<f64, CalcErrors> {

        let left = self.match_eval(left)?;
        let right = self.match_eval(right)?;

        match op {
            Opcode::Mul => Ok(left * right),
            Opcode::Div => {
                let r = left / right;
                if right == 0.0 {
                    return Err(CalcErrors::DivisionZeroProhibited)
                }
                Ok(r)
            },

            Opcode::Mod => Ok(left % right),
            Opcode::IntDiv => {
                let r = left / right;
                if right == 0.0 {
                    return Err(CalcErrors::DivisionZeroProhibited)
                }
                Ok(r.trunc())
            },

            Opcode::Add => Ok(left + right),
            Opcode::Sub => Ok(left - right),
        }
    }


    fn match_func(&mut self, name: &FuncName, expr: &Expr) -> Result<f64, CalcErrors> {

        match name {
            FuncName::Sin => match self.match_eval(expr) {
                Ok(a) => Ok(Self::radion_in_degrees(a).sin()),
                Err(err) => Err(err)
            },

            FuncName::Cos => match self.match_eval(expr) {
                Ok(a) => Ok(Self::radion_in_degrees(a).cos()),
                Err(err) => Err(err)
            },
        
            FuncName::Tg => match self.match_eval(expr) {
                Ok(a) => Ok(Self::radion_in_degrees(a).tan()),
                Err(err) => Err(err)
            },

            FuncName::Ctg => match self.match_eval(expr) {
                Ok(a) => Ok(a.cos() / Self::radion_in_degrees(a).sin()),
                Err(err) => Err(err)
            },

            FuncName::Exponentiation => match self.match_eval(expr) {
                Ok(a) => Ok(todo!()),
                Err(err) => Err(err)
            },

            FuncName::SquareRoot => match self.match_eval(expr) {
                Ok(a) => Ok(todo!()),
                Err(err) => Err(err)
            },
        }
    }
    

    fn radion_in_degrees(radian: f64) -> f64 {

        (radian * PI) / 180.
    }
}
