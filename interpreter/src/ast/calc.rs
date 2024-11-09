use super::expr::Expr;
use std::fmt::{Debug, Error, Formatter};

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
