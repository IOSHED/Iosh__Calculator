
use interpreter::{config::Config, interpreter::Interpreter};
use utils::parser;

macro_rules! testy {
    // expected - полученный ввод данных от пользвателя.
    // received - ожидаемое значение.
    ($expected: expr, $received: expr) => {
        let mut interpreter = Interpreter::new(Config::new(50, 50));

        let mut errors = Vec::new();

        match parser::CalcParser::new().parse(&mut errors, $expected) {
            Ok(ast) => match interpreter.eval(ast, $expected) {
                Ok(result) => match result {
                    Some(result) => assert_eq!(format!("{:?}", result), $received),
                    None => assert_eq!("\n", $received),
                },
                Err(err) => assert_eq!(format!("Error: {err:?}"), $received),
            },
            Err(err) => assert_eq!(format!("Error: {err:?}"), $received),
        }
    };
}

#[test]
fn div() {
    testy!("12 * 3", "36.0");
    testy!("2 * 2.2", "4.4");
    testy!("3.8 * 0", "0.0");
}

#[test]
fn mul() {
    testy!("10 mod 3", "1.0");
    testy!("2 mod 2", "0.0");
    testy!("3.8 mod 4.7", "3.8");
}

#[test]
fn mod_() {
    testy!("10 mod 3", "1.0");
    testy!("2 mod 2", "0.0");
    testy!("3.8 mod 4.7", "3.8");
}

#[test]
fn int_div() {
    testy!("10 div 3", "3.0");
    testy!("2 div 2", "1.0");
    testy!("3.8 div 4.7", "0.0");
}

#[test]
fn add() {
    testy!("3 + 4", "7.0");
    testy!("2 + 2", "4.0");
    testy!("3.8 + 4.7", "8.5");
}

#[test]
fn sub() {
    testy!("3 - 4", "-1.0");
    testy!("2 + 2 - 2", "2.0");
    testy!("3.0 - 4.5", "-1.5");
}
