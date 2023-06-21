use std::{fs::File, io::{BufReader, BufRead}};

use i_calc::errors::CalcErrors;

const PATH_FILE_HELP: &str = "README.md";

pub fn read_file_help() -> Result<String, CalcErrors> {
    let mut text = String::new();

    let file = match File::open(PATH_FILE_HELP) {
        Ok(f) => f,
        Err(_) => return Err(CalcErrors::CanNotOpenFileWithText)
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        text = format!("{text}{line}\n");
    }

    Ok(text)
}