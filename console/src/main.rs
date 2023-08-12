// Подключаем крейты для красивого и правильного кода.
#![warn(clippy::all, clippy::pedantic)]

extern crate lazy_static;

use utils::{get_ast, get_interpreter, get_result};
use in_out::get_input;

use crate::{printer::print_error, in_out::MessageIO};

mod in_out;
mod printer;

fn main() -> () {
    let mut interpreter = get_interpreter();

    loop {
        let input = match get_input(&mut interpreter) {
            MessageIO::Break => break,
            MessageIO::Continue => continue,
            MessageIO::Ok(input) => input,
        };

        let ast = match get_ast(&input, print_error) {
            Some(ast) => ast,
            None => continue,
        };

        let result = match get_result(&mut interpreter, ast, &input, print_error) {
            Some(result) => result,
            None => continue,
        };

        println!("{}", result);
    }
}
