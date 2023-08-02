use crate::printer::print_error;

use interpreter::ast::calc::Calc;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

pub fn get_ast<'input>(input: &'input str) -> Option<Calc<'input>> {
    let mut errors = Vec::new();

    match parser::CalcParser::new().parse(&mut errors, input) {
        Ok(ast) => Some(ast),
        Err(err) => {
            print_error(err);
            None
        }
    }
}
