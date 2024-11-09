use crate::{errors::CalcError, interpreter::Interpreter};

use super::{
    expr::{Evaluatable, Expr},
    func_name::FuncName,
};

use std::f64::consts::PI;

pub struct FactoryFunc;

impl FactoryFunc {
    pub fn match_(
        name: &FuncName, args: &[Box<Expr>], calc: &mut Interpreter,
    ) -> Result<f64, CalcError> {
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
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError>;

    fn check_len_args(args: &[Box<Expr>], expect: usize) -> Result<(), CalcError> {
        let len_args = args.len();
        if len_args != expect {
            return Err(CalcError::IncorrectNumberOfArguments(len_args, expect));
        }
        Ok(())
    }

    fn check_len_args_or_stand_default_value<'a>(
        args: &'a [Box<Expr<'a>>], expects: usize, default_value: Vec<f64>,
    ) -> Result<Vec<Box<Expr<'a>>>, CalcError> {
        let mut new_args = args.to_vec();

        match Self::check_len_args(args, expects) {
            Ok(()) => return Ok(args.to_vec()),
            Err(_) => {
                new_args.extend(
                    default_value.iter()
                        .skip(new_args.len())
                        .take(expects - new_args.len())
                        .map(|&value| Box::new(Expr::Number(value)))
                );
            }
        }

        Ok(new_args)
    }
}

trait AppendArgs {
    fn append_args(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Vec<f64>, CalcError> {
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
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError> {
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
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError> {
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
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError> {
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
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => {
                Ok(Self::radians_in_degrees(res).cos() / Self::radians_in_degrees(res).sin())
            }
            Err(err) => Err(err),
        }
    }
}

pub struct Exponentiation;

impl AppendArgs for Exponentiation {}

impl Function for Exponentiation {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError> {
        let args_add_default = Self::check_len_args_or_stand_default_value(args, 2, vec![0., 2.])?;
        let arg = Self::append_args(&args_add_default, calc)?;
        Ok(arg[0].powf(arg[1]))
    }
}

pub struct SquareRoot;

impl AppendArgs for SquareRoot {}

impl SquareRoot {
    fn nth_root(n: f64, a: f64) -> f64 {
        let eps = 1e-10; // Точность вычисления.
        let mut x: f64 = 1.0; // Начальное приближение.
        while (x.powf(n) - a).abs() > eps {
            x = ((n - 1.0) * x + a / x.powf(n - 1.0)) / n;
        }
        x
    }
}

impl Function for SquareRoot {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<f64, CalcError> {
        let args_add_default = Self::check_len_args_or_stand_default_value(args, 2, vec![0., 2.])?;
        let arg = Self::append_args(&args_add_default, calc)?;
        let res = Self::nth_root(arg[0], arg[1]);

        if res.is_nan() {
            return Err(CalcError::ImpossibleToExtractRootCorrectly);
        }
        Ok(res)
    }
}
