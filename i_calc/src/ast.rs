use std::fmt::{Debug, Error, Formatter};

use crate::errors::CalcErrors;


pub enum Calc<'input> {
    InitVariable(&'input str, Box<Expr<'input>>),
    Expr(Box<Expr<'input>>),
}


impl<'input> Debug for Calc<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Calc::*;
        match self {
            InitVariable(name, ref expr) => write!(fmt, "{name} = {expr:?}"),
            Expr(expr) => write!(fmt, "{expr:?}"),
        }
    }
}


pub enum Expr<'input> {
    Number(f64),
    Variable(&'input str),
    Op(Box<Expr<'input>>, Opcode, Box<Expr<'input>>),
    Func(FuncName, Box<Expr<'input>>),
    Error(CalcErrors),
}


impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({l:?} {op:?} {r:?})"),
            Func(func, ref args) => write!(fmt, "{func:?}({args:?})"),
            Error(msg) => write!(fmt, "error: {msg:?}"),
            Variable(name) => write!(fmt, "{name:?}"),
        }
    }
}


#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Mod,
    IntDiv,

    Add,
    Sub,
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


#[derive(Copy, Clone)]
pub enum FuncName {
    Exponentiation,  // Возмедение в степень.
    SquareRoot,  // Квадратный корень.

    Sin,
    Cos,

    Tg,
    Ctg,
}


impl Debug for FuncName {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            FuncName::Sin => write!(fmt, "sin"),
            FuncName::Cos => write!(fmt, "cos"),
            FuncName::Tg => write!(fmt, "tg"),
            FuncName::Ctg => write!(fmt, "ctg"),
            FuncName::Exponentiation => write!(fmt, "exp"),
            FuncName::SquareRoot => write!(fmt, "sqrt"),
        }
    }
}
