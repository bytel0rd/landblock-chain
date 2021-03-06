use chrono::prelude::*;
use std::boxed::Box;
use serde::{Serialize, Deserialize};

use super::super::utils::Error;

#[derive(Debug, Copy, Clone, Serialize, Deserialize,)]
pub enum BlockType {
    CONTRACT,
    TRANSACTION
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {

    block_type: BlockType,

    data: Option<i32>,

    transaction_date: DateTime<Utc>,

    from: String,

    to: Option<String>,

    previous_block_hash: Option<String>,

}

impl  Block {
    pub fn new_transaction(from: String, to: String) -> Self {
        return Block {
            from, 
            to:Some(to),
            data: None,
            block_type: BlockType::TRANSACTION,
            transaction_date: Utc::now(),
            previous_block_hash: None,
        };
    }
    
    pub fn new_contract(from: String) -> Self {
        return Block {
            from, 
            to: None,
            data: None,
            block_type: BlockType::CONTRACT,
            transaction_date: Utc::now(),
            previous_block_hash: None,
        };
    }

    pub fn set_data(&mut self, data: i32) -> () {
        self.data = Some(data);
    }
    pub fn set_transaction_date(mut self, date: DateTime<Utc>) -> () {
        self.transaction_date = date;
    }

    pub fn set_previous_hash(&mut self, hash: Option<String>) -> () {
        self.previous_block_hash = hash;
    }

    pub fn get_data(&self) -> &Option<i32> {
        return &self.data;
    }
    pub fn get_previous_block_hash(&self) -> &Option<String> {
        return &self.previous_block_hash;
    }

    pub fn get_transaction_date(&self) -> &DateTime<Utc> {
        return &self.transaction_date;
    }
    
    pub fn to_json(&self) -> Result<String, Error> {

        let serialized = serde_json::to_string(self);

        if serialized.is_err() {
            
            println!("{:#?}", serialized.unwrap_err());

            let error_message = String::from("Error occurred while serlizaling block");

            return Err(Error::from_message(
                error_message
            ));
        }

        return Ok(serialized.unwrap());
    }

    pub fn get_hash(&self) ->Result<String, Error> {
        return Block::calculate_hash(self);
    }

    
    pub fn calculate_hash(block:&Block) -> Result<String, Error> {
        
        let json = block.to_json()?;
        let hash = blake3::hash(json.as_bytes());

        return Ok(hash.to_string());
    }
}
