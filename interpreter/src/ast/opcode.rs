use crate::{errors::CalcError, interpreter::Interpreter};
use rust_decimal::Decimal;
use std::fmt::{Debug, Error, Formatter};

use super::{
    expr::{Evaluatable, Expr},
    operation::FactoryOp,
};

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Mod,
    IntDiv,

    Add,
    Sub,
}

pub trait Operation {
    fn evaluate(
        &self, left: Box<Expr>, right: Box<Expr>, interpreter: &mut Interpreter,
    ) -> Result<Decimal, CalcError>;
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::{Add, Div, IntDiv, Mod, Mul, Sub};
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            IntDiv => write!(fmt, "div"),
            Mod => write!(fmt, "mod"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Operation for Opcode {
    fn evaluate(
        &self, left: Box<Expr>, right: Box<Expr>, interpreter: &mut Interpreter,
    ) -> Result<Decimal, CalcError> {
        let left = left.evaluate(interpreter)?;
        let right = right.evaluate(interpreter)?;

        FactoryOp::match_(*self, left, right)
    }
}
