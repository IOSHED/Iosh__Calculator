
// Подключаем литер для красивого и правильного кода. 
#![warn(clippy::all, clippy::pedantic)]


#[macro_use] extern crate lalrpop_util;
extern crate i_calc;

use std::io::Write;
use i_calc::{interpreter::Interpreter, ast::calc::Calc, errors::CalcErrors};

lalrpop_mod!(pub parser);


const END_STRING: &str = "\r\n";
const END_PROGRAM: &str = "/end\r\n";
const GET_HISTORY: &str = "/history\r\n";


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


fn print_request_history(interpreter: &mut Interpreter, to: usize) -> () {

    fn check_len_history(history: &Vec<(String, Result<f64, CalcErrors>)>, mut to: usize) -> usize {
        let len_history = history.len();
        if to > len_history {
            to = len_history
        }
        to
    }

    fn get_len_big_element_in_history(history: &Vec<(String, Result<f64, CalcErrors>)>, min_len: usize) -> usize {
        let mut len_big_element = min_len;
        for (req_str, res) in history {
            let len_str = req_str.len();
            if len_str > len_big_element {
                len_big_element = len_str
            }

            if let Ok(res) = res {
                let len_res = format!("{res}").len();
                if len_res > len_big_element {
                    len_big_element = len_res
                }
            }
        }
        len_big_element
    }

    let history = interpreter.get_request_history(8);
    let to = check_len_history(&history, to);

    let left = "Result";
    let right = "String";

    let width = get_len_big_element_in_history(&history, left.len() + right.len());

    println!("| {:^width$} | {:^width$} |", left, right);
    println!("|-{:-^width$}-|-{:-^width$}-|", "", "");

    for i in 0..to {
        let (req_str, res) = &history[i];
        if let Ok(res) = res {
            println!("| {:^width$} | {:^width$} |", res, req_str.trim_end());
        }
    }
}


fn main() -> () {

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
                    print_request_history(&mut interpreter, 8); 
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
