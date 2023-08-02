use core::fmt::Debug;
use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};

use crate::printer::color;

/// Печатает ошибку в виде "Error: передоваемая ошибка" красным цветом.

pub fn print_error<T: Debug>(err: T) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(color::RED_ERROR),
        Print("Error: "),
        ResetColor,
        SetForegroundColor(color::RED),
        Print(format!("{err:?}\n")),
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
