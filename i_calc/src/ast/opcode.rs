use std::fmt::{Debug, Error, Formatter};


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