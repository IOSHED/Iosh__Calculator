
use std::{f64::consts::{PI, E}, collections::BTreeMap};
use crate::{
    ast::{expr::Expr, opcode::Opcode, calc::Calc, func::FactoryFunc}, 
    errors::CalcErrors, 
};

/// A calculator interpreter that can evaluate mathematical expressions and store variables
pub struct Interpreter<'input> {
    /// A mapping of input strings to their evaluation results
    request_history: BTreeMap<String, Result<f64, CalcErrors>>,
    /// A mapping of variable names to their values
    variables: BTreeMap<String, f64>,
    /// A mapping of constant names to their values
    constants: BTreeMap<&'input str, f64>,
}

impl<'input> Interpreter<'input> {
    
    /// Creates a new `Interpreter` with an empty request history, variable mapping and constant mapping
    pub fn new() -> Self {
        
        const SPEED_LIGHT: f64 = 299792458.0;  // СКОРОСТЬ СВЕТА
        const ACCELERATION_FREE_FALL: f64 = 9.80665;  // СКОРОСТЬ СВОБОДНОГО ПАДЕНИЯ
        const GRAVITATIONAL_CONSTANT: f64 = 0.0000000000066720;  // ГРАВИТАЦИОННАЯ ПОСТОЯННАЯ 

        let constants = BTreeMap::from([
            ("PI", PI),
            ("E", E),
            ("c", SPEED_LIGHT),
            ("g", ACCELERATION_FREE_FALL),
            ("G", GRAVITATIONAL_CONSTANT)
        ]);

        Interpreter {
            request_history: BTreeMap::new(),
            variables: BTreeMap::new(),
            constants
        }
    }

    /// Evaluates a `Calc` expression and returns a `Result` containing the evaluation result or an error
    pub fn eval(&mut self, calc: Calc, input: &str) -> Result<Option<f64>, CalcErrors> {
        match calc {
            Calc::InitVariable(name, expr) => {
                // Initialize a new variable with the given name and expression
                match self.init_variable(name, &expr) {
                    Some(err) => Err(err),
                    None => Ok(None),
                }
            },
            Calc::Expr(expr) => {
                // Evaluate the given expression
                match self.eval_expr(&expr, input) {
                    Ok(result) => Ok(Some(result)),
                    Err(err) => Err(err),
                }
            },
        }
    }

    /// Returns a history of evaluated inputs and their results, up to a given limit
    pub fn get_request_history(&self, to: usize) -> Vec<(String, Result<f64, CalcErrors>)> {
        self.request_history
            .iter()
            .take(to)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Evaluates the given `Expr` and returns a `Result` containing the evaluation result or an error
    fn eval_expr(&mut self, expr: &Expr, input: &str) -> Result<f64, CalcErrors> {
        // Check if the evaluation result for this input has already been stored in the request history
        if let Some(result) = self.request_history.get(input) {
            return *result;
        }

        // Evaluate the expression and store the result in the request history
        let result = expr.evaluate(self)?;
        self.request_history.insert(String::from(input), Ok(result));
        Ok(result)
    }

    /// Initializes a new variable with the given name and expression, returning an error if the name is already a constant
    fn init_variable(&mut self, name: &str, expr: &Box<Expr>) -> Option<CalcErrors> {
        // Check if the name is already a constant
        if self.constants.get(name).is_some() {
            return Some(CalcErrors::CannotCreateVariablesWithNameConstant);
        }

        // Evaluate the expression and store the result as the value of the variable
        match expr.evaluate(self) {
            Ok(result) => {
                self.variables.insert(name.to_string(), result);
                None
            },
            Err(err) => Some(err),
        }
    }
}

/// A trait for types that can be evaluated to a floating point number
pub trait Evaluatable {
    /// Evaluates the value of the type using the given `Interpreter`
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<f64, CalcErrors>;
}

impl<'input> Evaluatable for Expr<'input> {
    /// Evaluates the value of the expression using the given `Interpreter`
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<f64, CalcErrors> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Func(name, expr) => FactoryFunc::match_(
                name, expr, interpreter
            ),
            Expr::Variable(name) => {
                // Look up the value of the variable in the `Interpreter`'s mapping of variables and constants
                interpreter.variables.get(*name)
                    .copied()
                    .or_else(|| interpreter.constants.get(name).copied())
                    .ok_or(CalcErrors::CallingNonexistentVariable)
            },
            Expr::Op(left, op, right) => op.evaluate(left, right, interpreter),
            Expr::Error(err) => Err(*err),
        }
    }
}

/// A trait for binary operators that can be evaluated to a floating point number
pub trait Operation {
    /// Evaluates the value of the operator applied to the left and right expressions using the given `Interpreter`
    fn evaluate(&self,left: &Box<Expr>, right: &Box<Expr>, interpreter: &mut Interpreter) -> Result<f64, CalcErrors>;
}

impl Operation for Opcode {
    /// Evaluates the value of the operator applied to the left and right expressions using the given `Interpreter`
    fn evaluate(&self, left: &Box<Expr>, right: &Box<Expr>, interpreter: &mut Interpreter) -> Result<f64, CalcErrors> {
        // Evaluate the left and right expressions
        let left = left.evaluate(interpreter)?;
        let right = right.evaluate(interpreter)?;

        // Apply the operator to the evaluated expressions
        match self {
            Opcode::Mul => Ok(left * right),
            Opcode::Div => {
                if right == 0.0 {
                    return Err(CalcErrors::DivisionZeroProhibited);
                }
                Ok(left / right)
            },
            Opcode::Mod => Ok(left % right),
            Opcode::IntDiv => {
                if right == 0.0 {
                    return Err(CalcErrors::DivisionZeroProhibited)
                }
                Ok((left / right).trunc())
            },
            Opcode::Add => Ok(left + right),
            Opcode::Sub => Ok(left - right),
        }
    }
}