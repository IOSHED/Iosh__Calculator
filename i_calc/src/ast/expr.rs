use std::fmt::{Debug, Error, Formatter};
use crate::errors::CalcErrors;
use super::{opcode::Opcode, func_name::FuncName};


pub enum Expr<'input> {
    Number(f64),
    Variable(&'input str),
    Op(Box<Expr<'input>>, Opcode, Box<Expr<'input>>),
    Func(FuncName, Vec<Box<Expr<'input>>>),
    Error(CalcErrors),
}


impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({l:?} {op:?} {r:?})"),
            Func(func, ref args) => {
                let str = args
                                    .iter()
                                    .map(|val| format!("{:?}", val))
                                    .collect::<Vec<String>>().join(" ");
                write!(fmt, "{func:?}({str})")
            },
            Error(msg) => write!(fmt, "Ошибка: {msg:?}"),
            Variable(name) => write!(fmt, "{name:?}"),
        }
    }
}