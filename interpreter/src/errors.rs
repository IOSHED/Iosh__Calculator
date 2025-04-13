use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum CalcError {
    #[error("Input too big")]
    InputTooBig,

    #[error("Syntax error")]
    SyntaxError,

    #[error("Calling nonexistent variable: {0}")]
    CallingNonexistentVariable(String),

    #[error("Cannot create variable with name 'const'")]
    CannotCreateVariablesWithNameConstant,

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Unknown error")]
    UnknownError,

    #[error("Incorrect number of arguments: expected {0}, found {1}")]
    IncorrectNumberOfArguments(usize, usize),

    #[error("Cannot open file with text: {0}")]
    CannotOpenFileWithText(String),

    #[error("Impossible to extract root correctly")]
    ImpossibleToExtractRootCorrectly,

    #[error("Math error")]
    MathError,
}
