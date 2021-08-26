use std::time::Duration;
use std::marker::Copy;

#[derive(Debug, Copy, Clone)]
pub struct Block<'h> {

    data: Option<i32>,

    transaction_date: std::time::Duration,

    previous_block_hash: Option<&'h String>,

    next_block: Option<&'h Block<'h>>

}

impl<'h> Block<'h> {

    pub fn new () -> Self {
        return Block {
            data: None,
            transaction_date: std::time::Instant::now().elapsed(),
            previous_block_hash: None,
            next_block: None
        }
    }

    pub fn set_data(mut self, data: i32) -> () {
        self.data = Some(data)
    }
    
    pub fn set_transaction_date(mut self, date: std::time::Duration) -> () {
        self.transaction_date = date;
    }

    pub fn set_next_block(mut self, block: &'h Block) -> () {
        self.next_block = Some(block);
    }

    pub fn get_data(&self) -> &Option<i32> {
        return &self.data
    }
    
    pub fn get_previous_block_hash(&self) -> &Option<&String> {
        return &self.previous_block_hash;
    }

    pub fn get_transaction_date(&self) -> &Duration {
        return &self.transaction_date;
    }

}

