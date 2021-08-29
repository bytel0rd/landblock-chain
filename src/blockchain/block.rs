use std::time::Duration;
use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct Block {
    data: Option<i32>,

    transaction_date: std::time::Duration,

    previous_block_hash: Option<String>,

}

impl  Block {
    pub fn new() -> Self {
        return Block {
            data: None,
            transaction_date: std::time::Instant::now().elapsed(),
            previous_block_hash: None,
        };
    }

    pub fn set_data(&mut self, data: i32) -> () {
        self.data = Some(data);
    }
    pub fn set_transaction_date(mut self, date: std::time::Duration) -> () {
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

    pub fn get_transaction_date(&self) -> &Duration {
        return &self.transaction_date;
    }

    pub fn get_hash(&self) ->Option<String> {
        return Some(String::from("hash"));
    }
}
