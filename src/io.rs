
use std::io::Write;
use i_calc::interpreter::Interpreter;
use regex::{Regex, Captures};

use crate::printer::{print_request_history, print_help, print_error};

pub enum Messege<T> {
    Break,
    Continue,
    Ok(T),
}

const END_STRING: &str = "";
const END_PROGRAM: &str = "/end";
const GET_HISTORY: &str = "/history";
const HELP: &str = "/help";

fn handle_command(interpreter: &mut Interpreter, string: &str) -> Messege<String> {

    let string = string.trim_end();

    let re_end_program = END_PROGRAM;
    let re_end_str = END_STRING;
    let re_get_history = Regex::new(&format!(r"^{}(?:\s+(all|\d+))?$", GET_HISTORY)).unwrap();
    let re_help = HELP;

    // Match всех спец.команд для калькулятора. 
    if string == re_end_program {
        return Messege::Break;
    } 

    else if string == re_help {
        match print_help() {
            Ok(_) => (),
            Err(err) => print_error(err),
        }
        
        return Messege::Continue;
    }

    else if string == re_end_str {
        return Messege::Continue;
    }

    else if let Some(capt) = re_get_history.captures(string) {
        handler_arg_history(interpreter, capt)
    }

    else {
        return Messege::Ok(string.to_string());
    }
}

fn handler_arg_history(interpreter: &mut Interpreter, capt: Captures) -> Messege<String> {
    let mut output_line_history = 10;
    if let Some(arg) = capt.get(1) {
        if let Ok(num) = arg.as_str().parse::<usize>() {
            output_line_history = num;
        } else {
            output_line_history = interpreter.request_history.len();
        }
    }
    print_request_history(interpreter, output_line_history);
    Messege::Continue
}

fn get_input_user() -> String {

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

pub fn get_input(interpreter: &mut Interpreter) -> Messege<String> {
    let string = get_input_user();

    handle_command(interpreter, &string)
}
