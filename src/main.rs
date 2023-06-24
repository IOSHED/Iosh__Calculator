
// Подключаем литер для красивого и правильного кода. 
#![warn(clippy::all, clippy::pedantic)]


#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;
extern crate interpreter;

mod printer;
mod io;
mod config;

use config::Config;
use interpreter::{interpreter::Interpreter, ast::calc::Calc};
use printer::{print_start, print_error};
use io::{get_input, load_interpreter};

lalrpop_mod!(pub parser);

fn get_ast<'input>(input: &'input str) -> Option<Calc<'input>> {

    let mut errors = Vec::new();

    match parser::CalcParser::new().parse(&mut errors, input) {
        Ok(ast) => Some(ast),
        Err(err) => {
            print_error(err);
            None
        }
    }
}

fn get_result(interpreter: &mut Interpreter, ast: Calc, input: &str) -> Option<f64> {

    match interpreter.eval(ast, &input) {
        Ok(n) => {
            if n == None {
                return None
            } 
            n
        },
        Err(err) => {
            print_error(err);
            None
        }
    }
}

fn get_interpreter() -> Interpreter {
    let instans = Config::get();
    let config = instans.lock().unwrap();

    match load_interpreter() {
        Ok(mut i) => {
            i.config = config.get_config_for_interpreter();
            i
        },
        Err(_) => Interpreter::new(config.get_config_for_interpreter())
    }
}

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
