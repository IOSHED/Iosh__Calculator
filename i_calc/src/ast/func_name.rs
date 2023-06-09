use std::fmt::{Debug, Error, Formatter};


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
