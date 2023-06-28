//! Файл для вывода на экран информации для пользователя.
//! Не в коем случае не использовать на прямую, только через файл in_out.rs

use std::fmt::Debug;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use interpreter::errors::CalcError;
use interpreter::interpreter::Interpreter;

use crate::in_out::read_file_help;

/// Получение длины самого большого элемента в `History` - Vec<(String, Result<f64, CalcError>)>.
/// Cчитается даже длина для второй части - Result, преобразованный в тип String.
/// Не считается длина элемента, если он является ошибкой, то есть Err(_)
///
/// * `history` - сама история, по которой будет идти поиск.
/// * `min_len` - минимальная значение для длины, которое должно вернуться.
///
/// # Example
///
/// ```
/// 
/// let history = vec![
///     ("2 + 2".to_string(), Ok(4.0)),
///     ("2*2".to_string(), Ok(4.0)),
/// ];
///
/// assert_eq!(get_len_of_longest_valid_element_in_history(&history, 5), 5);
/// assert_eq!(get_len_of_longest_valid_element_in_history(&history, 1), 3);
/// 
/// ```

fn get_len_of_longest_valid_element_in_history(
    history: &Vec<(String, Result<f64, CalcError>)>,
    min_len: usize,
) -> usize {
    let max_len = history.iter()
        .filter_map(|(req_str, res)| match res {
            Ok(_) => Some(req_str.len()),
            Err(_) => None,
        })
        .max()
        .unwrap_or(0);
    
    max_len.max(min_len)
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
        SetForegroundColor(Color::Rgb {
            r: 97,
            g: 50,
            b: 58
        }),
        Print(format!("{}\n", line.to_uppercase())),
        ResetColor,
    )
    .unwrap()
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
    )
    .unwrap()
}

fn print_line(line: &str) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::White),
        Print(format!("{}\n", line)),
        ResetColor,
    )
    .unwrap()
}

pub fn print_request_history(interpreter: &mut Interpreter, to: usize) -> () {
    let to = check_len_history(interpreter, to);
    let mut history = interpreter.get_request_history(to);
    history.reverse();

    let left = "Result";
    let right = "String";

    let width = get_len_of_longest_valid_element_in_history(&history, left.len() + right.len());

    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Cyan),
        Print(format!("| {:^width$} | {:^width$} |\n", left, right)),
        Print(format!("|-{:-^width$}-|-{:-^width$}-|\n", "", "")),
        ResetColor,
    )
    .unwrap();

    for i in 0..to {
        let (req_str, res) = &history[i];
        if let Ok(res) = res {
            execute!(
                std::io::stdout(),
                SetForegroundColor(Color::Blue),
                Print(format!(
                    "| {:^width$} | {:^width$} |\n",
                    res,
                    req_str.trim_end()
                )),
                ResetColor,
            )
            .unwrap();
        }
    }
}

pub fn print_start() -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Green),
        Print(">>> "),
        ResetColor,
    )
    .unwrap()
}

pub fn print_error<T: Debug>(err: T) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Red),
        Print("Error: "),
        ResetColor,
        SetForegroundColor(Color::Rgb {
            r: 97,
            g: 50,
            b: 58
        }),
        Print(format!("{err:?}\n")),
        ResetColor,
    )
    .unwrap()
}

pub fn print_help() -> Result<(), CalcError> {
    fn check_str_char(line: &str, to: usize, ch: char) -> bool {
        line.chars().nth(to) == Some(ch)
    }

    let text = read_file_help()?;
    let lines: Vec<&str> = text.split("\n").collect();

    for line in lines {
        if check_str_char(line, 0, '#') {
            print_title(line)
        } else if check_str_char(line, 1, '.') {
            print_numeric(line)
        } else {
            print_line(line)
        }
    }
    Ok(())
}
