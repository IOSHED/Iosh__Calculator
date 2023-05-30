
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use super::{
    function::{expression, single_block}, 
    parser::Parser
};



// Парсер парсит всю струку на блоки, которые могут иметь внутри себя другие блоки.
// Блоками являются скобки строки.

#[derive(Debug, Clone)]
pub struct Block {
    id: u64,
    pub meaning: String,
    pub blocks: Vec<Block>,
}



impl Block {



    pub fn new(meaning: String) -> Self {

        let mut hasher = DefaultHasher::new();
        meaning.hash(&mut hasher);

        let id = hasher.finish();

        let meaning = expression().parse(&meaning).unwrap().1;

        let blocks: Vec<Block>= Self::add_blocks(&meaning);

        Block {
            id,
            meaning,
            blocks,
        }
    }



    fn add_blocks(meaning: &String, blocks: Vec<Block>) -> Vec<Block> {
        for (i, &item) in meaning.as_bytes().iter().enumerate() {
            if item == b'(' {
                let new_block = single_block().parse(&meaning[i..]).unwrap().1;
                println!("{:#?} - new block", new_block);
                println!("{:#?} - blocks", blocks);
                if blocks.iter().position(|s| s == &new_block) == Some(0) {
                    blocks.push(new_block);
                }
            }
        }
        blocks
    }



}




impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
