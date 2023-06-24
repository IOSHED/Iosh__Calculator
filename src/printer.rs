use std::fmt::Debug;

use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};

use interpreter::errors::CalcErrors;
use interpreter::interpreter::Interpreter;

use crate::io::read_file_help;

fn get_len_big_element_in_history(history: &Vec<(String, Result<f64, CalcErrors>)>, min_len: usize) -> usize {
    let mut len_big_element = min_len;
    for (req_str, res) in history {
        let len_str = req_str.len();
        if len_str > len_big_element {
            len_big_element = len_str
        }

        if let Ok(res) = res {
            let len_res = format!("{res}").len();
            if len_res > len_big_element {
                len_big_element = len_res
            }
        }
    }
    len_big_element
}

fn check_len_history(interpreter: &mut Interpreter, mut to: usize) -> usize {
    let len_history = interpreter.request_history.len();
    if to > len_history {
        to = len_history
    }
    to
}

fn print_title(line: &str) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Rgb { r: 97, g: 50, b: 58 }),
        Print(format!("{}\n", line.to_uppercase())),
        ResetColor,
    ).unwrap()
}

fn print_numeric(line: &str) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Blue),
        Print(line.get(0..2).unwrap()),
        ResetColor,
        SetForegroundColor(Color::Cyan),
        Print(format!("{}\n", line.get(3..line.len()).unwrap())),
        ResetColor
    ).unwrap()
}

fn print_line(line: &str) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::White),
        Print(format!("{}\n", line)),
        ResetColor,
    ).unwrap()
}

pub fn print_request_history(interpreter: &mut Interpreter, to: usize) -> () {

    let to = check_len_history(interpreter, to);
    let mut history = interpreter.get_request_history(to);
    history.reverse();

    let left = "Result";
    let right = "String";

    let width = get_len_big_element_in_history(&history, left.len() + right.len());

    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Cyan),
        Print(format!("| {:^width$} | {:^width$} |\n", left, right)),
        Print(format!("|-{:-^width$}-|-{:-^width$}-|\n", "", "")),
        ResetColor,
    ).unwrap();

    for i in 0..to {
        let (req_str, res) = &history[i];
        if let Ok(res) = res {
            execute!(
                std::io::stdout(),
                SetForegroundColor(Color::Blue),
                Print(format!("| {:^width$} | {:^width$} |\n", res, req_str.trim_end())),
                ResetColor,
            ).unwrap();
        }
    }
}

pub fn print_start() -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Green),
        Print(">>> "),
        ResetColor,
    ).unwrap()
}

pub fn print_error<T: Debug>(err: T) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Red),
        Print("Error: "),
        ResetColor,
        SetForegroundColor(Color::Rgb { r: 97, g: 50, b: 58 }),
        Print(format!("{err:?}\n")),
        ResetColor,
    ).unwrap()
}

pub fn print_help() -> Result<(), CalcErrors> {
    fn check_str_char(line: &str, to: usize, ch: char) -> bool {
        line.chars().nth(to) == Some(ch)
    }

    let text = read_file_help()?;
    let lines: Vec<&str> = text.split("\n").collect();

    for line in lines {
        if check_str_char(line, 0, '#') {
            print_title(line)
        }
        else if check_str_char(line, 1, '.') {
            print_numeric(line)
        }
        else {
            print_line(line)
        }
    }
    Ok(())
} 