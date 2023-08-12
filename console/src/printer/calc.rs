use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use interpreter::errors::CalcError;

use crate::printer::color;

/// Печатает ошибку в виде "Error: передоваемая ошибка" красным цветом.

pub fn print_error(err: CalcError) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(color::RED_ERROR),
        Print("Error: "),
        ResetColor,
        SetForegroundColor(color::RED),
        Print(format!("{err}\n")),
        ResetColor,
    )
    .unwrap()
}

/// Печатает `>>>`, зелёным цветом.

pub fn print_start() -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(color::GREEN),
        Print(">>> "),
        ResetColor,
    )
    .unwrap()
}
