#[macro_use] extern crate lalrpop_util;

use i_calc::interpreter::Interpreter;


lalrpop_mod!(pub parser);


macro_rules! testy {

    // expected - полученный ввод данных от пользвателя.
    // received - ожидаемое значение.
    ($expected: expr, $received: expr) => {

        let mut errors = Vec::new();
        
        match parser::CalcParser::new().parse(&mut errors, $expected) {
            Ok(ast) => {
                let mut interpreter = Interpreter::new();
                match interpreter.eval(ast, $expected) {
                    Ok(n) => {
                        assert_eq!(
                            $received,
                            format!("{:.9}", n)
                        )
                    },
                    Err(err) => {
                        assert_eq!(
                            format!("Error: {err:?}"),
                            $received
                        )
                    }
                }
            },
            Err(err) => {
                assert_eq!(
                    format!("Error: {:?}", err),
                    $received
                )
            }
        }
    };
}


#[test]
fn add() {
    testy!("3 + 4", "7");
    testy!("2 + 2", "4");
    testy!("3.8 + 4.7", "8.5");
}


#[test]
fn sub() {
    testy!("3 - 4", "-1");
    testy!("2 + 2 - 2", "2");
    testy!("3.8 - 4.7", "-0.9");
}