pub mod config;

use config::Config;
use interpreter::{ast::calc::Calc, errors::CalcError, interpreter::Interpreter};
use lalrpop_util::lalrpop_mod;
use rust_decimal::Decimal;
use std::{fs, io};

lalrpop_mod!(pub parser, "/lexer/parser.rs");

pub fn get_ast(input: &str, funct_caused_error: fn(CalcError) -> ()) -> Option<Calc> {
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

#[must_use]
pub fn get_interpreter() -> Interpreter {
    let instance = Config::get();
    let config = instance.lock().unwrap();

    match load_interpreter() {
        Ok(mut i) => {
            i.config = config.get_config_for_interpreter();
            i
        }
        Err(_) => Interpreter::new(config.get_config_for_interpreter())
            .expect("Unexpected error with id #000001"),
    }
}

pub fn get_result(
    interpreter: &mut Interpreter, ast: Calc, input: &str, funct_caused_error: fn(CalcError) -> (),
) -> Option<Decimal> {
    match interpreter.eval(ast, input) {
        Ok(n) => {
            n?;
            n
        }
        Err(err) => {
            funct_caused_error(err);
            None
        }
    }
}
