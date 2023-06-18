use i_calc::errors::CalcErrors;
use i_calc::interpreter::Interpreter;

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

fn check_len_history(history: &Vec<(String, Result<f64, CalcErrors>)>, mut to: usize) -> usize {
    let len_history = history.len();
    if to > len_history {
        to = len_history
    }
    to
}

pub fn print_request_history(interpreter: &mut Interpreter, to: usize) -> () {

    let history = interpreter.get_request_history(8);
    let to = check_len_history(&history, to);

    let left = "Result";
    let right = "String";

    let width = get_len_big_element_in_history(&history, left.len() + right.len());

    println!("| {:^width$} | {:^width$} |", left, right);
    println!("|-{:-^width$}-|-{:-^width$}-|", "", "");

    for i in 0..to {
        let (req_str, res) = &history[i];
        if let Ok(res) = res {
            println!("| {:^width$} | {:^width$} |", res, req_str.trim_end());
        }
    }
}