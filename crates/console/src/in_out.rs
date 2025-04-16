use std::{
    fs::{self, File},
    io::{self, Write},
};

use calc_core::config::Config;
use interpreter::{errors::CalcError, interpreter::Interpreter};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::printer::{print_error, print_help, print_start, Printer, Table};

pub enum MessageIO<T> {
    Break,
    Continue,
    Ok(T),
}

lazy_static! {
    static ref RE_END_PROGRAM: String = Config::get().lock().unwrap().commands.end.clone();
    static ref RE_END_STR: String = Config::get().lock().unwrap().commands.empty_input.clone();
    static ref RE_GET_HISTORY: Regex = Regex::new(&format!(
        r"^{}(?:\s+(all|\d+))?$",
        Config::get().lock().unwrap().commands.history.clone()
    ))
    .unwrap();
    static ref RE_HELP: String = Config::get().lock().unwrap().commands.help.clone();
    static ref OUTPUT_LINE_HISTORY: usize = Config::get().lock().unwrap().output_line_history;
}

pub fn handle_command(interpreter: &mut Interpreter, string: &str) -> MessageIO<String> {
    let string = string.trim_end();

    match string {
        s if s == RE_END_PROGRAM.as_str() => {
            save_interpreter(interpreter);
            MessageIO::Break
        }
        s if s == RE_HELP.as_str() => {
            if let Err(err) = print_help() {
                print_error(err);
            }
            MessageIO::Continue
        }
        s if s == RE_END_STR.as_str() => MessageIO::Continue,
        _ => {
            if let Some(capt) = RE_GET_HISTORY.captures(string) {
                handler_arg_history(interpreter, &capt)
            } else {
                MessageIO::Ok(string.to_string())
            }
        }
    }
}

pub fn handler_arg_history(interpreter: &mut Interpreter, capt: &Captures) -> MessageIO<String> {
    let output_line_history = capt
        .get(1)
        .and_then(|arg| arg.as_str().parse::<usize>().ok())
        .unwrap_or(interpreter.request_history.len());
    Table::print(interpreter, output_line_history);
    MessageIO::Continue
}

fn save_interpreter(interpreter: &mut Interpreter) {
    let serialized = serde_json::to_string(interpreter).unwrap();

    let file = File::create("interpreter.json").unwrap();
    let mut writer = io::BufWriter::new(file);
    writer.write_all(serialized.as_bytes()).unwrap();
}

fn get_input_user() -> io::Result<String> {
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn read_file_help() -> Result<String, CalcError> {
    let binding = Config::get();
    let config = binding.lock().unwrap();

    let path = &config.path_file_help;

    let text = fs::read_to_string(path)
        .map_err(|_| CalcError::CannotOpenFileWithText(path.to_string()))?;

    Ok(text)
}

pub fn get_input(interpreter: &mut Interpreter) -> MessageIO<String> {
    print_start();

    let string = match get_input_user() {
        Ok(s) => s,
        Err(_) => return MessageIO::Break,
    };

    handle_command(interpreter, &string)
}
