use std::fmt::{Debug, Error, Formatter};


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CalcErrors {
    InputTooBig,
    SyntaxError,
    CallingNonexistentVariable,
    CannotCreateVariablesWithNameConstant,
    DivisionZeroProhibited,
    UnknownError,
}

impl Debug for CalcErrors {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            CalcErrors::InputTooBig => write!(fmt, "Введено слишком большое число."),
            CalcErrors::SyntaxError => write!(fmt, "Синтаксическая ошибка."),
            CalcErrors::CallingNonexistentVariable => write!(fmt, "Вызов несуществующей переменой"),
            CalcErrors::CannotCreateVariablesWithNameConstant => write!(fmt, "Нельзя создавать переменные именем константы."),
            CalcErrors::DivisionZeroProhibited => write!(fmt, "Нельзя делить на ноль."),
            CalcErrors::UnknownError => write!(fmt, "Неизвестная ошибка."),
        }
    }
}