use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};

use interpreter::errors::CalcError;

use crate::{in_out::read_file_help, printer::color};

/// Печатает заголовок текста цветом (97, 50, 58) заглавными буквами.

fn print_title(line: &str) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(color::RED),
        Print(format!("{}\n", line.to_uppercase())),
        ResetColor,
    )
    .unwrap()
}

/// Печатает строку Color::Cyan, а цифру стоящую перед ней Color::Blue.

fn print_numeric(line: &str) -> () {
    let split_line = line.split_at(2);
    execute!(
        std::io::stdout(),
        SetForegroundColor(color::BLUE),
        Print(split_line.0),
        ResetColor,
        SetForegroundColor(color::CYAN),
        Print(format!("{}\n", split_line.1)),
        ResetColor
    )
    .unwrap()
}

/// Печатает строку цветом Color::White.

fn print_line(line: &str) -> () {
    execute!(
        std::io::stdout(),
        SetForegroundColor(color::WHITE),
        Print(format!("{}\n", line)),
        ResetColor,
    )
    .unwrap()
}

/// Печатает файл с инструкцией.

pub fn print_help() -> Result<(), CalcError> {
    fn check_str_char(line: &str, to: usize, ch: char) -> bool {
        line.chars().nth(to) == Some(ch)
    }

    let text = read_file_help()?;
    let lines: Vec<&str> = text.split("\n").collect();

    lines.iter().for_each(|line| match line {
        line if check_str_char(line, 0, '#') => print_title(line),
        line if check_str_char(line, 1, '.') => print_numeric(line),
        _ => print_line(line),
    });
    Ok(())
}
