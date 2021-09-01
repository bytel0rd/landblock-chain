use std::convert::TryInto;

use super::block::Block;
use super::super::utils::{Error, ErrorLevel};

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

    pub fn append_block(&mut self, mut block: Block) -> Result<(), Error> {

        let possible_last_block = self.get_last_block();

        if possible_last_block.is_none() {
            self.link.push(block);
        } else {

            let last_block = possible_last_block.unwrap();
        
            let hash = last_block.get_hash()?;

            block.set_previous_hash(Some(hash));   
            self.link.push(block);
        }

        return Ok(());
    }

    pub fn get_block_by_index(&self, index: usize) -> Option<&Block> {
        return self.link.get(index);
    }

    pub fn validate_chain(&self) -> Result<bool, Error> {

        let is_valid = true;

        let mut last_index = self.len() - 1;

        if last_index  <  0 {
            return Ok(is_valid);
        }

        for _ in 0..last_index + 1 {

            let current_block_hash_previous_hash = self.get_block_by_index(1)
                .map(|block| block.get_previous_block_hash().as_deref())
                .flatten()
                .map(|hash| String::from(hash));

            let previous_block_hash = self.get_block_by_index(0)
                .map(|block| block.get_hash()).unwrap()?;

                if current_block_hash_previous_hash.is_some() {

                    if current_block_hash_previous_hash.unwrap() != previous_block_hash {
                        return Ok(false);
                    }

                }

            
            last_index -= 1;
        }

        return Ok(is_valid);
    }
}
