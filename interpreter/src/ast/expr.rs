use std::fmt::{Debug, Error, Formatter};
use crate::{errors::CalcErrors, interpreter::Interpreter, traits::GetResult};
use super::{opcode::{Opcode, Operation}, func_name::FuncName, func::FactoryFunc};


#[derive(Clone)]
pub enum Expr<'input> {
    Number(f64),
    Variable(&'input str),
    Op(Box<Expr<'input>>, Opcode, Box<Expr<'input>>),
    Func(FuncName, Vec<Box<Expr<'input>>>),
    Error(CalcErrors),
}

pub trait Evaluatable {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<f64, CalcErrors>;
}


impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(l, op, r) => write!(fmt, "({l:?} {op:?} {r:?})"),
            Func(func, args) => {
                let str = args
                    .iter()
                    .map(|val| format!("{:?}", val))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(fmt, "{func:?}({str})")
            },
            Error(msg) => write!(fmt, "Ошибка: {msg:?}"),
            Variable(name) => write!(fmt, "{name:?}"),
        }
    }
}

impl<'input> Expr<'input> {
    pub fn get_variable(interpreter: &mut Interpreter, name: &&str) -> Result<f64, CalcErrors> {
        interpreter.variables.get_result(name)
            .or_else(|| interpreter.constants.get_result(name))
            .ok_or(CalcErrors::CallingNonexistentVariable)
    }
}


impl<'input> Evaluatable for Expr<'input> {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<f64, CalcErrors> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Func(name, expr) => FactoryFunc::match_(name, expr, interpreter),
            Expr::Variable(name) => Self::get_variable(interpreter, name),
            Expr::Op(left, op, right) => op.evaluate(left, right, interpreter),
            Expr::Error(err) => Err(*err),
        }
    }
}