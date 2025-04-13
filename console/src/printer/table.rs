use crate::printer::{color, Printer};
use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use rust_decimal::Decimal;
use interpreter::{errors::CalcError, history::History, interpreter::Interpreter};

/// Получение длины самого большого элемента в `History` - Vec<(String, Result<Decimal, `CalcError`>)>.
/// Cчитается даже длина для второй части - Result, преобразованный в тип String.
/// Не считается длина элемента, если он является ошибкой, то есть Err(_).
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
/// ```
fn get_len_of_longest_valid_element_in_history(
    history: &[(String, Result<Decimal, CalcError>)], min_len: usize,
) -> usize {
    let max_len = history
        .iter()
        .filter_map(|(req_str, res)| match res {
            Ok(_) => Some(req_str.len()),
            Err(_) => None,
        })
        .max()
        .unwrap_or(0);

    max_len.max(min_len)
}

/// struct `Table`
/// * `width` - ширина таблицы.
/// * `left_name` - имя левой колонки таблицы.
/// * `right_name` - имя правой колонки таблицы.
/// * `content` - содержимое таблицы.
#[derive(Clone)]
pub struct Table<'a> {
    width: usize,
    left_name: String,
    right_name: String,
    content: &'a Vec<(String, Result<Decimal, CalcError>)>,
}

impl<'a> Table<'a> {
    /// Сoздание новой `Table`.
    ///
    /// * `left_name` - имя левой колонки таблицы.
    /// * `right_name` - имя правой колонки таблицы.
    /// * `history` - содержимое таблицы, котрое печатается в два столбика.
    pub fn new(
        left_name: &str, right_name: &str, history: &'a Vec<(String, Result<Decimal, CalcError>)>,
    ) -> Self {
        let width = get_len_of_longest_valid_element_in_history(
            history,
            left_name.len() + right_name.len(),
        );

        Table {
            width,
            content: history,
            left_name: left_name.to_string(),
            right_name: right_name.to_string(),
        }
    }

    fn print_table_title(self) -> Self {
        let width = self.width;
        execute!(
            std::io::stdout(),
            SetForegroundColor(color::BLUE),
            Print("| "),
            SetForegroundColor(color::CYAN),
            Print(format!("{:^width$}", self.left_name,)),
            SetForegroundColor(color::BLUE),
            Print(" | "),
            SetForegroundColor(color::CYAN),
            Print(format!("{:^width$}", self.right_name)),
            SetForegroundColor(color::BLUE),
            Print(" |\n"),
            ResetColor,
        )
        .unwrap();
        self
    }

    fn print_table_border(self) -> Self {
        let width = self.width;
        execute!(
            std::io::stdout(),
            SetForegroundColor(color::BLUE),
            Print("|-"),
            Print(format!("{:-^width$}", "",)),
            SetForegroundColor(color::BLUE),
            Print("-|-"),
            Print(format!("{:-^width$}", "")),
            SetForegroundColor(color::BLUE),
            Print("-|\n"),
            ResetColor,
        )
        .unwrap();
        self
    }

    /// Печатет шапку таблицы.
    pub fn print_table_header(self) -> Self {
        self.clone().print_table_title();
        self.clone().print_table_border();
        self
    }

    /// Печатает строчку таблицы.
    pub fn print_table_line(self, res: Decimal, req_str: &str) -> Self {
        let width = self.width;
        execute!(
            std::io::stdout(),
            SetForegroundColor(color::BLUE),
            Print("| "),
            SetForegroundColor(color::CYAN),
            Print(format!("{res:^width$}",)),
            SetForegroundColor(color::BLUE),
            Print(" | "),
            SetForegroundColor(color::CYAN),
            Print(format!("{:^width$}", req_str.trim_end())),
            SetForegroundColor(color::BLUE),
            Print(" |\n"),
            ResetColor,
        )
        .unwrap();
        self
    }

    /// Печатает строчки таблицы.
    pub fn print_table_lines(self, to: usize) {
        for (_, (req_str, res)) in self.content.iter().enumerate().take(to) {
            match res {
                Ok(res) => self.clone().print_table_line(*res, req_str),
                Err(_) => continue,
            };
        }
    }
}

impl Printer for Table<'_> {
    fn print(interpreter: &mut Interpreter, to: usize) {
        let (left_name, right_name) = ("Result", "String");

        let to = History::get_len_history(interpreter, to);

        let mut history = interpreter.get_request_history(to);
        history.reverse();

        Table::new(left_name, right_name, &history)
            .print_table_header()
            .print_table_lines(to);
    }
}
