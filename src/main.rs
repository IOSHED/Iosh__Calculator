
// Подключаем литер для красивого и правильного кода. 
#![warn(clippy::all, clippy::pedantic)]

#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;
extern crate interpreter;

mod printer;
mod io;
mod config;
mod calculator;

use calculator::get_interpreter;
use printer::print_start;
use io::get_input;

use crate::calculator::{get_ast, get_result};

lalrpop_mod!(pub parser);


fn main() -> () {

    let mut interpreter = get_interpreter();

    'lp: loop {
        print_start();

        let input = match get_input(&mut interpreter) {
            io::Messege::Break => break 'lp,
            io::Messege::Continue => continue 'lp,
            io::Messege::Ok(input) => input,
        };

        let ast = match get_ast(&input) {
            Some(ast) => ast,
            None => continue 'lp,
        };

        let result = match get_result(&mut interpreter, ast, &input) {
            Some(r) => r,
            None => continue 'lp,
        };

        println!("{result}");
    }
}
