use std::fmt::{Debug, Error, Formatter};

use crate::{errors::CalcError, interpreter::Interpreter};

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
        &self,
        left: &Box<Expr>,
        right: &Box<Expr>,
        interpreter: &mut Interpreter,
    ) -> Result<f64, CalcError>;
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
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
        &self,
        left: &Box<Expr>,
        right: &Box<Expr>,
        interpreter: &mut Interpreter,
    ) -> Result<f64, CalcError> {
        let left = left.evaluate(interpreter)?;
        let right = right.evaluate(interpreter)?;

        FactoryOp::match_(*self, left, right)
    }
}
