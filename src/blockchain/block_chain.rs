// use std::marker::Copy;
use std::convert::TryInto;

use super::block::Block;

#[derive(Debug, Clone)]
pub struct BlockChain {
    link: Vec<Block>,
}

impl BlockChain {

    pub fn new() -> Self {

         return BlockChain {
            link: vec![],
        };
    }

    pub fn get_first_block(&self) -> Option<&Block> {
        return self.link.get(0);
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        return self.link.last()
    }

    pub fn len(&self) -> i64 {
        return self.link.len().try_into().unwrap();
    }

    pub fn append_block(&mut self, mut block: Block) -> () {

        let last_block = self.get_last_block();

        if last_block.is_none() {
            self.link.push(block);
        } else {
            let hash = last_block.unwrap().get_hash();
            block.set_previous_hash(hash.clone());   
            self.link.push(block);
        }
    }

    pub fn get_block_by_index(&self, index: usize) -> Option<&Block> {
        return self.link.get(index);
    }

    pub fn validate_chain(&self) -> bool {

        let is_valid = true;

        let mut last_index = self.len() - 1;

        if last_index  <  0 {
            return is_valid;
        }

        for _ in 0..last_index + 1 {

            let current_block_previous_block_hash = self.get_block_by_index(0)
                .map(|block| block.get_hash()).flatten();

            let previous_block_hash = self.get_block_by_index(0)
                .map(|block| block.get_previous_block_hash().as_deref())
                .flatten()
                .map(|hash| String::from(hash));

                if current_block_previous_block_hash != previous_block_hash {
                    return false;
                }
            
            last_index -= 1;
        }

        return is_valid;
    }
}
