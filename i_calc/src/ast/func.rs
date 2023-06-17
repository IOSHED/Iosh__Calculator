use crate::{interpreter::Interpreter, errors::CalcErrors}; 

use super::{expr::{Expr, Evaluatable}, func_name::FuncName}; 

use std::f64::consts::PI; 


pub struct FactoryFunc;


impl FactoryFunc {
    pub fn match_(name: &FuncName, args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        match name {
            FuncName::Sin => Sin::ahead(args, calc),
            FuncName::Cos => Cos::ahead(args, calc),
            FuncName::Tg => Tg::ahead(args, calc),
            FuncName::Ctg => Ctg::ahead(args, calc),
            FuncName::Exponentiation => Exponentiation::ahead(args, calc),
            FuncName::SquareRoot => SquareRoot::ahead(args, calc),
        }
    }
}


trait Function {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors>;

    fn check_len_args(args: &[Box<Expr>], expect: usize) -> Result<(), CalcErrors> {
        let len_args = args.len();
        if len_args != expect {
            return Err(CalcErrors::IncorrectNumberArguments(len_args, expect));
        }
        Ok(())
    }
}


trait AppendArgs {
    fn append_args(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Vec<f64>, CalcErrors> {
        let mut arg = Vec::new();
        for i in args {
            match i.evaluate(calc) {
                Ok(res) => arg.push(res),
                Err(err) => return Err(err),
            }
        }
        Ok(arg)
    }
}


trait Trigonometry {
    fn radians_in_degrees(radians: f64) -> f64 {
        (radians * PI) / 180.
    }
}

pub struct Sin;

impl Trigonometry for Sin {}

impl Function for Sin {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res).sin()),
            Err(err) => Err(err),
        }
    }
}


pub struct Cos;

impl Trigonometry for Cos {}

impl Function for Cos {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res).cos()),
            Err(err) => Err(err),
        }
    }
}


pub struct Tg;

impl Trigonometry for Tg {}

impl Function for Tg {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res).tan()),
            Err(err) => Err(err),
        }
    }
}


pub struct Ctg;

impl Trigonometry for Ctg {}

impl Function for Ctg {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res).cos() / Self::radians_in_degrees(res).sin()),
            Err(err) => Err(err),
        }
    }
}


pub struct Exponentiation;

impl AppendArgs for Exponentiation {}

impl Function for Exponentiation {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        Self::check_len_args(args, 2)?;
        let arg = Self::append_args(args, calc)?;
        Ok(arg[0].powf(arg[1]))
    }
}


pub struct SquareRoot;

impl AppendArgs for SquareRoot {}

impl Function for SquareRoot {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        let arg = Self::append_args(args, calc)?;
        match Self::check_len_args(args, 1) {
            Ok(_) => Ok(arg[0].sqrt()),
            Err(_) => {
                Self::check_len_args(args, 2)?;
                Err(CalcErrors::UnknownError)
            }
        }
    }
}
