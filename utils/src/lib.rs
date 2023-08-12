
pub mod config;

use std::{io, fs};
use lalrpop_util::lalrpop_mod;

use config::Config;
use interpreter::{ast::calc::Calc, errors::CalcError, interpreter::Interpreter};


lalrpop_mod!(pub parser, "/lexer/parser.rs");


pub fn get_ast<'input>(input: &'input str, funct_caused_error: fn(CalcError) -> ()) -> Option<Calc<'input>> {
    let mut errors = Vec::new();

    match parser::CalcParser::new().parse(&mut errors, input) {
        Ok(ast) => Some(ast),
        Err(err) => {
            err.map_error(|e| {
                funct_caused_error(e);
            });
            
            None
        }
    }
}

pub fn load_interpreter() -> io::Result<Interpreter> {
    let contents = fs::read_to_string("interpreter.json")?;
    let interpreter: Interpreter = serde_json::from_str(&contents)?;
    Ok(interpreter)
}

pub fn get_interpreter() -> Interpreter {
    let instans = Config::get();
    let config = instans.lock().unwrap();

    match load_interpreter() {
        Ok(mut i) => {
            i.config = config.get_config_for_interpreter();
            i
        }
        Err(_) => Interpreter::new(config.get_config_for_interpreter()),
    }
}

pub fn get_result(
    interpreter: &mut Interpreter, 
    ast: Calc, 
    input: &str, 
    funct_caused_error: fn(CalcError) -> ()
) -> Option<f64> {

    match interpreter.eval(ast, &input) {
        Ok(n) => {
            if n == None {
                return None;
            }
            n
        }
        Err(err) => {
            funct_caused_error(err);
            None
        }
    }
}
