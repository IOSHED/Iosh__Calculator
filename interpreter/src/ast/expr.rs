use super::{
    func::FactoryFunc,
    func_name::FuncName,
    opcode::{Opcode, Operation},
};
use crate::{errors::CalcError, interpreter::Interpreter, traits::GetResult};
use rust_decimal::Decimal;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]
pub enum Expr<'input> {
    Number(Decimal),
    Variable(&'input str),
    Op(Box<Expr<'input>>, Opcode, Box<Expr<'input>>),
    Func(FuncName, Vec<Box<Expr<'input>>>),
    Error(CalcError),
}

pub trait Evaluatable {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Decimal, CalcError>;
}

impl Debug for Expr<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::{Error, Func, Number, Op, Variable};
        match self {
            Number(n) => write!(fmt, "{n:?}"),
            Op(l, op, r) => write!(fmt, "({l:?} {op:?} {r:?})"),
            Func(func, args) => {
                let str = args
                    .iter()
                    .map(|val| format!("{val:?}"))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(fmt, "{func:?}({str})")
            }
            Error(msg) => write!(fmt, "Ошибка: {msg:?}"),
            Variable(name) => write!(fmt, "{name:?}"),
        }
    }
}

impl Expr<'_> {
    pub fn get_variable(interpreter: &mut Interpreter, name: &&str) -> Result<Decimal, CalcError> {
        interpreter
            .variables
            .get_result(name)
            .or_else(|| interpreter.constants.get_result(name))
            .ok_or(CalcError::CallingNonexistentVariable((*name).to_string()))
    }
}

impl Evaluatable for Expr<'_> {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Decimal, CalcError> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Func(name, expr) => FactoryFunc::match_(name, expr, interpreter),
            Expr::Variable(name) => Self::get_variable(interpreter, name),
            Expr::Op(left, op, right) => op.evaluate(left.clone(), right.clone(), interpreter),
            Expr::Error(err) => Err(err.clone()),
        }
    }
}
