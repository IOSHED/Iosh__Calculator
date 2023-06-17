use std::fmt::{Debug, Error, Formatter};

use crate::{interpreter::Interpreter, errors::CalcErrors};

use super::expr::{Expr, Evaluatable};


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
    fn evaluate(&self,left: &Box<Expr>, right: &Box<Expr>, interpreter: &mut Interpreter) -> Result<f64, CalcErrors>;
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
    fn evaluate(&self, left: &Box<Expr>, right: &Box<Expr>, interpreter: &mut Interpreter) -> Result<f64, CalcErrors> {
        let left = left.evaluate(interpreter)?;
        let right = right.evaluate(interpreter)?;

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