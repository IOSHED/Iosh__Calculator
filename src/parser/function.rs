
use super::{
    function_standart::{pair, zero_or_more, right, left, one_or_more, any_char, match_literal, string_concatenation}, 
    parser::{Parser}, 
    indentifier::{number, number_or_variable, operation},
    block::Block
};

// # ПАРСЕР
// Вырезает из входной истроки её блок, находящийся в круглых скобках.

/*
 * Эта функция создает парсер, который распознает строку, 
 * заключенную в кавычки и окруженную скобками. Он использует несколько других парсеров, 
 * таких как `match_literal` и `zero_or_more`, чтобы сначала распознать открывающую скобку, 
 * затем ноль или более символов, не являющихся закрывающей скобкой, и 
 * наконец закрывающую скобку. После этого он применяет функцию `map` к полученным символам, 
 * чтобы объединить их в одну строку. 
 * Результатом парсера является успешный результат с распознанной строкой или ошибка, 
 * если входная последовательность не соответствует описанию.
 */

static mut COUNT: i32 = 1;
fn string_parentheses<'a>(from: &'static str, to: &'static str) -> impl Parser<'a, String> {
    unsafe { COUNT = 1 };
    right(
        match_literal(from),
        left(
            slise_block(from, to),
            match_literal(to),
        ),
    ).map(|chars| chars.into_iter().collect())
}



// # ПАРСЕР
// Находит символы, находящиеся в блоке .

fn slise_block<'a>(from: &'static str, to: &'static str) -> impl Parser<'a, Vec<char>> {
    zero_or_more(
        any_char.pred(
            move |c| {
                if *c == from.chars().nth(0).unwrap() {
                    unsafe { COUNT += 1 };
                } 
                if *c == to.chars().nth(0).unwrap() {
                    unsafe { COUNT -= 1 };
                } 
                unsafe { COUNT != 0 } 
            }
        )
    )
}



// # ПАРСЕР
// Если итерируемый символ содержит пробел, то мы можем дальше его обработать.

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    any_char.pred( |c| c.is_whitespace())
}



// # ПАРСЕР
// Пропускаем 1 пробел или больше.

fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}



// # ПАРСЕР
// Пропускаем 0 пробелов или больше.

fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}




// # ПАРСЕР
// Создаёт из строки блока в структуру Block.

pub fn single_block<'a>() -> impl Parser<'a, Block> {
    right(
        space0(),
        string_parentheses("(", ")"),
    ).map(
        |meaning| {   
            Block::new(meaning)
        }
    )
}



// # ПАРСЕР
// Парсит пару: число(индификатор)-операция.

fn pair_number_operation<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    zero_or_more(
        pair(
            number_or_variable(),
            whitespace_wrap(operation),
        ),
    )
}


// # ПАРСЕР
// Парсит выражение, состоящие из лишних пробелов и пары число-операция

pub fn expression<'a>() -> impl Parser<'a, String> {
    whitespace_wrap(
        pair(
            pair_number_operation(),
            whitespace_wrap(number)
        ).map(|value| string_concatenation(value))
    )
}



// # ПАРСЕР
// Пропускаем 0 пробелов или больше между чем угодно.

pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    right(space0(), left(parser, space0()))
}

