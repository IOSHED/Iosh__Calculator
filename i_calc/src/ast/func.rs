use std::f64::consts::PI;

use crate::{interpreter::Interpreter, errors::CalcErrors};

use super::{expr::Expr, func_name::FuncName};


pub struct FactoryFunc;

impl FactoryFunc {
    pub fn match_(name: &FuncName, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        match name {
            FuncName::Sin => Sin.ahead(args, calc),

            FuncName::Cos => Cos.ahead(args, calc),
        
            FuncName::Tg => Tg.ahead(args, calc),

            FuncName::Ctg => Ctg.ahead(args, calc),

            FuncName::Exponentiation => Exponentiation.ahead(args, calc),

            FuncName::SquareRoot => SquareRoot.ahead(args, calc),
        }
    }
}


trait Function {

    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors>;

    fn cheak_len_args(&self, args: &Vec<Box<Expr>>, expect: usize) -> Result<(), CalcErrors> {
        let len_args = args.len();
        if len_args != expect {
            return Err(CalcErrors::IncorrectNumberArguments(len_args, expect))
        }
        Ok(())
    }
}

trait AppendArgs {

    fn appened_args(&self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<Vec<f64>, CalcErrors> {
        let mut arg = Vec::new();

        for i in args {
            match calc.match_eval(&i) {
                Ok(res) => arg.push(res),
                Err(err) => return Err(err)
            }
        }

        Ok(arg)
    }
}

trait Trigonometry {
    fn radion_in_degrees(&self, radian: f64) -> f64 {

        (radian * PI) / 180.
    }
}

pub struct Sin;

impl Trigonometry for Sin {}

impl Function for Sin {

    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        self.cheak_len_args(args, 1)?;
        match calc.match_eval(&args[0]) {
            Ok(res) => Ok(self.radion_in_degrees(res).sin()),
            Err(err) => Err(err)
        }

    }
}


pub struct Cos;

impl Trigonometry for Cos {}

impl Function for Cos {
    
    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        self.cheak_len_args(args, 1)?;
        match calc.match_eval(&args[0]) {
            Ok(res) => Ok(self.radion_in_degrees(res).cos()),
            Err(err) => Err(err)
        }

    }
}


pub struct Tg;

impl Trigonometry for Tg {}

impl Function for Tg {
    
    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        self.cheak_len_args(args, 1)?;
        match calc.match_eval(&args[0]) {
            Ok(res) => Ok(self.radion_in_degrees(res).tan()),
            Err(err) => Err(err)
        }

    }
}


pub struct Ctg;

impl Trigonometry for Ctg {}

impl Function for Ctg {
    
    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        self.cheak_len_args(args, 1)?;
        match calc.match_eval(&args[0]) {
            Ok(res) => Ok(
                self.radion_in_degrees(res).cos() / self.radion_in_degrees(res).sin()
            ),
            Err(err) => Err(err)
        }

    }
}


pub struct Exponentiation;

impl AppendArgs for Exponentiation {}

impl Function for Exponentiation {
    
    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        self.cheak_len_args(args, 2)?;
        let arg = self.appened_args(args, calc)?;
        Ok(arg[0].powf(arg[1]))
    }
}


pub struct SquareRoot;

impl AppendArgs for SquareRoot {}

impl Function for SquareRoot {
    
    fn ahead(self, args: &Vec<Box<Expr>>, calc: &mut Interpreter) -> Result<f64, CalcErrors> {
        let arg = self.appened_args(args, calc)?;
        match self.cheak_len_args(args, 1) {
            Ok(_) => Ok(arg[0].sqrt()),
            Err(_) => {
                self.cheak_len_args(args, 2)?;
                Err(CalcErrors::UnknownError)
            }
        }
    }
}
