//! Модуль для вывода на экран информации для пользователя.
//! Не в коем случае не использовать на прямую, только через файл in_out.rs.

mod calc;
mod color;
mod help;
mod table;

use interpreter::interpreter::Interpreter;

pub use calc::{print_error, print_start};
pub use help::print_help;
pub use table::Table;

pub trait Printer {
    fn print(interpreter: &mut Interpreter, to: usize) -> ();
}
