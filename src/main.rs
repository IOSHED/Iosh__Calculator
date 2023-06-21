
// Подключаем литер для красивого и правильного кода. 
#![warn(clippy::all, clippy::pedantic)]


#[macro_use] extern crate lalrpop_util;
extern crate i_calc;

mod printer;
mod io;
mod reader_file;

use i_calc::{interpreter::Interpreter, ast::calc::Calc};
use printer::{print_start, print_error};
use io::get_input;

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

fn main() -> () {

    let mut interpreter = Interpreter::new();

    'lp: loop {
        print_start();

        let input = match get_input(&mut interpreter) {
            io::Messege::Break => break 'lp,
            io::Messege::Continue => continue 'lp,
            io::Messege::Ok(input) => input,
        };


        let ast = {
            let calc = get_ast(&input);

            match calc {
                Some(ast) => ast,
                None => continue 'lp,
            }
        };


        let result = {
            let res = get_result(&mut interpreter, ast, &input);

            match res {
                Some(r) => r,
                None => continue 'lp,
            }
        };

        println!("{result}");
    }
}
