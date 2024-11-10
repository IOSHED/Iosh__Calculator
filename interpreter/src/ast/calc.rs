use super::expr::Expr;
use std::fmt::{Debug, Error, Formatter};

pub enum Calc<'input> {
    InitVariable(&'input str, Box<Expr<'input>>),
    Expr(Box<Expr<'input>>),
}

impl Debug for Calc<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Calc::{Expr, InitVariable};
        match self {
            InitVariable(name, ref expr) => write!(fmt, "{name} = {expr:?}"),
            Expr(expr) => write!(fmt, "{expr:?}"),
        }
    }
}
