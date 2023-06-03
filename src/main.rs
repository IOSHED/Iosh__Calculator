
// Подключаем литер для красивого и правильного кода. 
#![warn(clippy::all, clippy::pedantic)]


#[macro_use] extern crate lalrpop_util;
extern crate i_calc;

use std::io::Write;
use i_calc::{interpreter::Interpreter, ast::Calc};

lalrpop_mod!(pub parser);


const END_STRING: &str = "\r\n";
const END_PROGRAM: &str = "/end";
const GET_HISTORY: &str = "/history";


fn get_input_user() -> String {

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}


fn get_ast<'input>(input: &'input str) -> Option<Calc<'input>> {

    let mut errors = Vec::new();

    match parser::CalcParser::new().parse(&mut errors, input) {
        Ok(ast) => Some(ast),
        Err(err) => {
            println!("Error: {err:?}");
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
            println!("Error: {err:?}");
            None
        }
    }
}


fn main() {

    let mut interpreter = Interpreter::new();

    loop {
        print!(">>> ");

        let input = {

            let str = get_input_user();

            // match всех спец. команд для калькулятора. 
            match &str[..] {
                END_STRING => continue,
                END_PROGRAM => break,
                GET_HISTORY => {
                    println!("{:#?}", interpreter.get_request_historys(8)); 
                    continue;
                },
                _ => str
            }
        };


        let ast = {
            let calc = get_ast(&input);

            match calc {
                Some(ast) => ast,
                None => continue,
            }
        };


        let result = {
            let res = get_result(&mut interpreter, ast, &input);

            match res {
                Some(r) => r,
                None => continue,
            }
        };

        println!("{result}");
    }
}
