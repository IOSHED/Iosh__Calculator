
use crate::error::Errors;

use super::{
    parser::{ParseResult, Parser},
    function_standart::either, 
    function::{whitespace_wrap}
};



// В этой костанте представлены все обрабатываемые парсерами операциями.

const OPERATION: &'static str = "+-*^:/";



// # ПАРСЕР
// Функция находит место в строке, которое не определяется как "-"б цифра или буква
// И возвращает сначала строку после индификатора и оставшуюся строку.

/*
 * Этот код определяет функцию `variable`,
 * которая принимает в качестве аргумента строку и возвращает пару значений: 
 * остаток строки после первого найденного идентификатора и сам идентификатор. 
 * Идентификатор в данном случае определяется как последовательность символов, 
 * начинающаяся с буквы и состоящая из букв и цифр, возможно с дефисами между ними. 
 * Если входная строка не начинается с буквы, функция вернет ошибку `ParseResult<String>`. 
 * В противном случае функция будет искать следующие символы, добавляя их к идентификатору, 
 * пока не встретит символ, который не является буквой, цифрой или дефисом. 
 * Затем функция вернет пару значений:
 * остаток строки после найденного идентификатора и сам идентификатор.
 */

/*
 * Пример использования функции `variable`:

    ```
    let input = "my-identifier-123 some other text";
    let result = variable(input);

    match result {
        Ok((rest, id)) => {
            println!("Found variable: {}", id);
            println!("Rest of the string: {}", rest);
        },
        Err(e) => {
            println!("Error parsing string: {:?}", e);
        }
    }
    ```

 * В данном примере мы передаем строку `"my-identifier-123 some other text"`
 * в функцию `variable`. Функция находит первый идентификатор `"my-identifier-123"`
 * и возвращает его, а также остаток строки `" some other text"`. 
 * Затем мы выводим найденный идентификатор и остаток строки на экран. 
 * Если входная строка не начинается с буквы, то функция вернет ошибку `ParseResult<String>`, 
 * которую мы также можем обработать.
 */

pub fn variable(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(Errors::InvalidIndentifier(input)),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' || next == '_' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}



// # ПАРСЕР
// Индифицируем цифры.

pub fn number<'a>(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    while let Some(next) = chars.next() {
        if next.is_numeric() || next == '.' || next == ',' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}



// # ПАРСЕР
// Индифицируем математические операции.

pub fn operation<'a>(input: &str) -> ParseResult<String> {
    let mut chars = input.chars();

    if let Some(next) = chars.next() {
        if OPERATION.contains(next) {
            Ok((&input[1..], next.to_string()))

        } else {
            Err(Errors::InvalidIndentifier(input))
        }

    } else {
        Err(Errors::InvalidIndentifier(input))
    }
} 



// # ПАРСЕР
// Обработка числа или индификатора или блока скобок.

pub fn number_or_variable<'a>() -> impl Parser<'a, String> {
    whitespace_wrap(
        either(
            number,
            variable,
        )
    )
    
}