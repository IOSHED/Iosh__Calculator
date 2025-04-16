extern crate lazy_static;

use calc_core::{get_ast, get_interpreter, get_result};
use in_out::get_input;

use crate::panic_hook::debug_panic_hook;
use crate::{in_out::MessageIO, printer::print_error};

mod in_out;
mod panic_hook;
mod printer;

fn main() {
    std::panic::set_hook(Box::new(debug_panic_hook));

    let mut interpreter = get_interpreter();

    loop {
        let input = match get_input(&mut interpreter) {
            MessageIO::Break => break,
            MessageIO::Continue => continue,
            MessageIO::Ok(input) => input,
        };

        let ast = match get_ast(&input, print_error) {
            Some(ast) => ast,
            None => continue,
        };

        let result = match get_result(&mut interpreter, ast, &input, print_error) {
            Some(result) => result,
            None => continue,
        };

        println!("{result}");
    }
}
