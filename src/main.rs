
// Подключаем литер для красивого и правильного кода. 
#![warn(clippy::all, clippy::pedantic)]


#[macro_use] extern crate lalrpop_util;
extern crate i_calc;

use std::io::Write;
use i_calc::{interpreter::Interpreter};

lalrpop_mod!(pub parser);


const END_STRING: &str = "\r\n" ;


fn get_input_user() -> String {
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input
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
                "/end" => break,
                "/history" => {
                    println!("{:#?}", interpreter.get_request_historys(8)); 
                    continue;
                },
                _ => str
            }
        };

        let mut errors = Vec::new();

        let ast = {
            match parser::CalcParser::new().parse(&mut errors, &input) {
                Ok(ast) => ast,
                Err(err) => {
                    println!("Error: {err:?}");
                    continue;
                }
            }
        };

        let result = {
            match interpreter.eval(ast, &input) {
                Ok(n) => {
                    if n == f64::NAN {
                        continue;
                    } else {
                        n
                    }
                },
                Err(err) => {
                    println!("Error: {err:?}");
                    continue;
                }
            }
        };

        println!("{result:.9}");
    }
}
