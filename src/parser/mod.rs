

use self::{block::Block};


mod parser;
mod block;
mod function_standart;
mod function;
mod indentifier;




pub fn parser_for_calculator0<'a>(input: &'a str) -> Block {
    let main_block = Block::new(input.to_owned());
    
    main_block
}