
use std::{io::{Write, BufReader, BufRead, Read}, fs::File};
use interpreter::{interpreter::Interpreter, errors::CalcErrors};
use regex::{Regex, Captures};

use crate::{printer::{print_request_history, print_help, print_error}, config::Config};

pub enum Messege<T> {
    Break,
    Continue,
    Ok(T),
}

lazy_static! {
    static ref RE_END_PROGRAM: String = Config::get().lock().unwrap().command.end.clone();
    static ref RE_END_STR: String = Config::get().lock().unwrap().command.empty_input.clone();
    static ref RE_GET_HISTORY: Regex = Regex::new(
        &format!(r"^{}(?:\s+(all|\d+))?$", Config::get().lock().unwrap().command.history.clone())
    ).unwrap();
    static ref RE_HELP: String = Config::get().lock().unwrap().command.help.clone();
    static ref OUTPUT_LINE_HISTORY: usize = Config::get().lock().unwrap().output_line_history;
}

fn handle_command(interpreter: &mut Interpreter, string: &str) -> Messege<String> {

    let string = string.trim_end();

    // Match всех спец.команд для калькулятора. 
    if string == RE_END_PROGRAM.as_str() {
        save_interpreter(interpreter);
        return Messege::Break;
    } 

    else if string == RE_HELP.as_str() {
        match print_help() {
            Ok(_) => (),
            Err(err) => print_error(err),
        }
        
        return Messege::Continue;
    }

    else if string == RE_END_STR.as_str() {
        return Messege::Continue;
    }

    else if let Some(capt) = RE_GET_HISTORY.captures(string) {
        handler_arg_history(interpreter, capt)
    }

    else {
        return Messege::Ok(string.to_string());
    }
}

fn handler_arg_history(interpreter: &mut Interpreter, capt: Captures) -> Messege<String> {
    let mut output_line_history = OUTPUT_LINE_HISTORY.to_string().parse::<usize>().unwrap();
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

fn save_interpreter(interpreter: &mut Interpreter) {
    let serialized = serde_json::to_string(&interpreter).unwrap();

    let mut file = File::create("interpreter.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

fn get_input_user() -> String {

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

pub fn load_interpreter() -> Result<Interpreter, std::io::Error> {
    let mut file = File::open("interpreter.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let interpreter: Interpreter = serde_json::from_str(&contents).unwrap();
    Ok(interpreter)
}

pub fn read_file_help() -> Result<String, CalcErrors> {
    let instans = Config::get();
    let config = instans.lock().unwrap();
    
    let mut text = String::new();

    let file = match File::open(config.path_file_help.clone()) {
        Ok(f) => f,
        Err(_) => return Err(CalcErrors::CanNotOpenFileWithText)
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        text = format!("{text}{line}\n");
    }

    Ok(text)
}

pub fn get_input(interpreter: &mut Interpreter) -> Messege<String> {
    let string = get_input_user();

    handle_command(interpreter, &string)
}
