
use utils::parser;

macro_rules! testy {
    /// * expected - полученное в результате парсинга значение.
    /// * received - ожидаемое значение.
    ($expected: expr, $received: expr) => {
        let mut errors = Vec::new();
        assert_eq!(
            &format!(
                "{:?}",
                parser::CalcParser::new()
                    .parse(&mut errors, $expected)
                    .unwrap()
            ),
            $received
        );
    };
}

macro_rules! testy_is_ok {
    /// * name - имя структуры lalrpop, с которой будут парсится данные.
    /// * expected - полученное в результате парсинга значение.

    //  проверяем упала ли программа при выполнениии парсинга.
    ($name: ident, $expected: expr) => {
        let mut errors = Vec::new();
        assert!(parser::$name::new().parse(&mut errors, $expected).is_ok());
    };
}

macro_rules! testy_struct {
    /// * name - имя структуры lalrpop, с которой будут парсится данные.
    /// * expected - полученное в результате парсинга значение.
    /// * received - ожидаемое значение.
    ($name: ident, $expected: expr, $received: expr) => {
        let mut errors = Vec::new();
        assert_eq!(
            parser::$name::new().parse(&mut errors, $expected).unwrap(),
            $received
        );
    };
}

#[test]
fn number() {
    testy_struct!(NumParser, "2", 2.0);

    testy_struct!(NumParser, "-22.7", -22.7);

    testy_struct!(NumParser, "+222.222222", 222.222222);

    testy_struct!(NumParser, "222.", 222.);

    testy_struct!(NumParser, "22,7", 22.7);

    testy_struct!(NumParser, "+222,222222", 222.222222);

    testy_struct!(NumParser, "222,", 222.0);

    testy_struct!(NumParser, "2,", 2.0);
}

#[test]
fn term() {
    testy_is_ok!(TermParser, "32");

    testy_is_ok!(TermParser, "(782)");

    testy_is_ok!(TermParser, "((((45342))))");

    testy_is_ok!(TermParser, "((234.)");
}

#[test]
fn expr() {
    testy!["11 * 22 + 33", "((11.0 * 22.0) + 33.0)"];

    testy!["(1 * 2) : 3", "((1.0 * 2.0) / 3.0)"];

    testy!["(1 * 2) / 3", "((1.0 * 2.0) / 3.0)"];

    testy!["(1 * 2) / (3 (43))", "((1.0 * 2.0) / (3.0 * 43.0))"];
}

#[test]
fn func() {
    testy!["sin(2)", "sin(2.0)"];

    testy!["cos(3 - 8)", "cos((3.0 - 8.0))"];

    testy!["sin(3 * 3 - 8)", "sin(((3.0 * 3.0) - 8.0))"];

    testy!["sin(cos(2))", "sin(cos(2.0))"];

    testy!["sin(7) * 7", "(sin(7.0) * 7.0)"];

    testy!["sin(cos(2) * 7)", "sin((cos(2.0) * 7.0))"];
}

#[test]
fn variable() {
    testy!["name", "\"name\""];

    testy!["name - foo", "(\"name\" - \"foo\")"];

    testy!["name * foo", "(\"name\" * \"foo\")"];

    testy!["name * 5.66", "(\"name\" * 5.66)"];

    testy!["8,3 - name", "(8.3 - \"name\")"];

    testy!["name * sin(foo)", "(\"name\" * sin(\"foo\"))"];
}

#[test]
fn init_variable() {
    testy!["name = 2", "name = 2.0"];

    testy!["name = 2 - 8", "name = (2.0 - 8.0)"];

    testy!["name = 2 * 7", "name = (2.0 * 7.0)"];

    testy!["name = (1 * 2) / 3", "name = ((1.0 * 2.0) / 3.0)"];
}
