use crate::{errors::CalcError, interpreter::Interpreter};

use super::{
    expr::{Evaluatable, Expr},
    func_name::FuncName,
};

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::f64::consts::PI;

pub struct FactoryFunc;

impl FactoryFunc {
    pub fn match_(
        name: &FuncName, args: &[Box<Expr>], calc: &mut Interpreter,
    ) -> Result<Decimal, CalcError> {
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
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError>;

    fn check_len_args(args: &[Box<Expr>], expect: usize) -> Result<(), CalcError> {
        let len_args = args.len();
        if len_args != expect {
            return Err(CalcError::IncorrectNumberOfArguments(len_args, expect));
        }
        Ok(())
    }

    fn check_len_args_or_stand_default_value<'a>(
        args: &'a [Box<Expr<'a>>], expects: usize, default_value: Vec<Decimal>,
    ) -> Result<Vec<Box<Expr<'a>>>, CalcError> {
        let mut new_args = args.to_vec();

        match Self::check_len_args(args, expects) {
            Ok(()) => return Ok(args.to_vec()),
            Err(_) => {
                new_args.extend(
                    default_value
                        .iter()
                        .skip(new_args.len())
                        .take(expects - new_args.len())
                        .map(|&value| Box::new(Expr::Number(value))),
                );
            }
        }

        Ok(new_args)
    }
}

trait DecimalMath {
    fn sin(&self) -> Result<Decimal, CalcError>;
    fn cos(&self) -> Result<Decimal, CalcError>;
    fn tan(&self) -> Result<Decimal, CalcError>;
    fn powf(&self, exponent: Decimal) -> Result<Decimal, CalcError>;
}

impl DecimalMath for Decimal {
    fn sin(&self) -> Result<Decimal, CalcError> {
        match self.to_f64() {
            Some(v) => {
                let sin_value = v.sin();
                Decimal::from_f64(sin_value).ok_or(CalcError::MathError)
            }
            None => Err(CalcError::MathError),
        }
    }

    fn cos(&self) -> Result<Decimal, CalcError> {
        match self.to_f64() {
            Some(v) => {
                let sin_value = v.cos();
                Decimal::from_f64(sin_value).ok_or(CalcError::MathError)
            }
            None => Err(CalcError::MathError),
        }
    }

    fn tan(&self) -> Result<Decimal, CalcError> {
        match self.to_f64() {
            Some(v) => {
                let sin_value = v.tan();
                Decimal::from_f64(sin_value).ok_or(CalcError::MathError)
            }
            None => Err(CalcError::MathError),
        }
    }

    fn powf(&self, exponent: Decimal) -> Result<Decimal, CalcError> {
        let base = self.to_f64().ok_or(CalcError::MathError)?;
        let exp = exponent.to_f64().ok_or(CalcError::MathError)?;
        Decimal::from_f64(base.powf(exp)).ok_or(CalcError::MathError)
    }
}

trait AppendArgs {
    fn append_args(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Vec<Decimal>, CalcError> {
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
    fn radians_in_degrees(radians: Decimal) -> Result<Decimal, CalcError> {
        let pi: Decimal = PI.try_into().map_err(|_| CalcError::MathError)?;
        let div_to: Decimal = 180.into();
        Ok((radians * pi) / div_to)
    }
}

pub struct Sin;

impl Trigonometry for Sin {}

impl Function for Sin {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res)?.sin()?),
            Err(err) => Err(err),
        }
    }
}

pub struct Cos;

impl Trigonometry for Cos {}

impl Function for Cos {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res)?.cos()?),
            Err(err) => Err(err),
        }
    }
}

pub struct Tg;

impl Trigonometry for Tg {}

impl Function for Tg {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => Ok(Self::radians_in_degrees(res)?.tan()?),
            Err(err) => Err(err),
        }
    }
}

pub struct Ctg;

impl Trigonometry for Ctg {}

impl Function for Ctg {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError> {
        Self::check_len_args(args, 1)?;
        match args[0].evaluate(calc) {
            Ok(res) => {
                Ok(Self::radians_in_degrees(res)?.cos()? / Self::radians_in_degrees(res)?.sin()?)
            }
            Err(err) => Err(err),
        }
    }
}

pub struct Exponentiation;

impl AppendArgs for Exponentiation {}

impl Function for Exponentiation {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError> {
        let args_add_default =
            Self::check_len_args_or_stand_default_value(args, 2, vec![0.into(), 2.into()])?;
        let arg = Self::append_args(&args_add_default, calc)?;
        arg[0].powf(arg[1])
    }
}

pub struct SquareRoot;

impl AppendArgs for SquareRoot {}

impl SquareRoot {
    fn nth_root(n: Decimal, a: Decimal) -> Result<Decimal, CalcError> {
        let eps = 1e-10; // Точность вычисления.
        let mut x: Decimal = 1.into(); // Начальное приближение.
        while (x.powf(n)? - a).abs() > Decimal::try_from(eps).map_err(|_| CalcError::MathError)? {
            x = ((n - Decimal::from(1)) * x + a / x.powf(n - Decimal::from(1))?) / n;
        }
        Ok(x)
    }
}

impl Function for SquareRoot {
    fn ahead(args: &[Box<Expr>], calc: &mut Interpreter) -> Result<Decimal, CalcError> {
        let args_add_default =
            Self::check_len_args_or_stand_default_value(args, 2, vec![0.into(), 2.into()])?;
        let arg = Self::append_args(&args_add_default, calc)?;
        let res = Self::nth_root(arg[0], arg[1])?;

        // if res.is_sign_negative() {
        //     return Err(CalcError::ImpossibleToExtractRootCorrectly);
        // }
        Ok(res)
    }
}
