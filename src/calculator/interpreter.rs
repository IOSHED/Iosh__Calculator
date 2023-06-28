use interpreter::interpreter::Interpreter;

use crate::{config::Config, in_out::load_interpreter};


pub fn get_interpreter() -> Interpreter {
    let instans = Config::get();
    let config = instans.lock().unwrap();

    match load_interpreter() {
        Ok(mut i) => {
            i.config = config.get_config_for_interpreter();
            i
        },
        Err(_) => Interpreter::new(config.get_config_for_interpreter())
    }
}