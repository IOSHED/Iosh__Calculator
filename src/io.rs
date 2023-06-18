
use std::io::Write;
use i_calc::interpreter::Interpreter;

use crate::printer::print_request_history;

pub enum Messege<T> {
    Break,
    Continue,
    Ok(T),
}

const END_STRING: &str = "";
const END_PROGRAM: &str = "/end";
const GET_HISTORY: &str = "/history";

fn match_command(interpreter: &mut Interpreter, string: &str) -> Messege<String> {

    // match всех спец. команд для калькулятора. 
    match string.trim_end() {
        END_STRING => Messege::Continue,
        END_PROGRAM => Messege::Break,
        GET_HISTORY => {
            print_request_history(interpreter, 8); 
            Messege::Continue
        },
        _ => Messege::Ok(string.to_string())
    }
}

fn get_input_user() -> String {

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

pub fn get_input(interpreter: &mut Interpreter) -> Messege<String> {
    let string = get_input_user();

    match_command(interpreter, &string)
}

