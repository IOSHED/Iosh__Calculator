// Подключаем крейты для красивого и правильного кода.
#![warn(clippy::all, clippy::pedantic)]

extern crate lazy_static;

use crate::calculator::{get_ast, get_interpreter, get_result};
use in_out::get_input;
use lalrpop_util::lalrpop_mod;
use messege::MessageIO;

mod calculator;
mod config;
mod in_out;
mod messege;
mod printer;

lalrpop_mod!(pub parser);

fn main() -> () {
    let mut interpreter = get_interpreter();

    loop {
        let input = match get_input(&mut interpreter) {
            MessageIO::Break => break,
            MessageIO::Continue => continue,
            MessageIO::Ok(input) => input,
        };

        let ast = match get_ast(&input) {
            Some(ast) => ast,
            None => continue,
        };

        let result = match get_result(&mut interpreter, ast, &input) {
            Some(result) => result,
            None => continue,
        };

        println!("{}", result);
    }
}
