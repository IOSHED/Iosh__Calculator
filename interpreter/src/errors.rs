use thiserror::Error;


#[derive(Error, Debug)]
pub enum CalcError {
    #[error("Input too big: {0}")]
    InputTooBig(u64),

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

    #[error("Incorrect number of arguments: expected {expected}, found {found}")]
    IncorrectNumberOfArguments { expected: usize, found: usize },

    #[error("Cannot open file with text: {0}")]
    CannotOpenFileWithText(String),

    #[error("Impossible to extract root correctly")]
    ImpossibleToExtractRootCorrectly,
}
