use std::fmt::{Debug, Error, Formatter};


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CalcErrors {
    InputTooBig,
    SyntaxError,
    CallingNonexistentVariable,
    CannotCreateVariablesWithNameConstant,
    DivisionZeroProhibited,
    UnknownError,
    IncorrectNumberArguments(usize, usize),
    CanNotOpenFileWithText
}


impl Debug for CalcErrors {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            CalcErrors::InputTooBig => write!(fmt, "Введено слишком большое число."),

            CalcErrors::SyntaxError => write!(fmt, "Синтаксическая ошибка."),

            CalcErrors::CallingNonexistentVariable => write!(fmt, "Вызов несуществующей переменой"),

            CalcErrors::CannotCreateVariablesWithNameConstant => write!(
                fmt, "Нельзя создавать переменные именем константы."
            ),
            CalcErrors::DivisionZeroProhibited => write!(fmt, "Нельзя делить на ноль."),

            CalcErrors::UnknownError => write!(fmt, "Неизвестная ошибка."),

            CalcErrors::IncorrectNumberArguments(expect, found) => write!(
                fmt, "Введено неправильное количество аргументов. Ожидалось {found}. Передано {expect}"
            ),

            CalcErrors::CanNotOpenFileWithText => write!(
                fmt, "Невозможно открыть файл с текстом"
            ),

        }
    }
}