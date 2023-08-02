use interpreter::{ast::calc::Calc, interpreter::Interpreter};

use crate::printer::print_error;

pub fn get_result(interpreter: &mut Interpreter, ast: Calc, input: &str) -> Option<f64> {
    match interpreter.eval(ast, &input) {
        Ok(n) => {
            if n == None {
                return None;
            }
            n
        }
        Err(err) => {
            print_error(err);
            None
        }
    }
}
