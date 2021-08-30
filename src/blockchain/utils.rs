#[derive(Debug, Clone, Copy)]
pub enum ErrorLevel {
    Internal,
    External,
}

#[derive(Debug, Clone)]
pub struct BlockError  {

    message: String,

    error_level: ErrorLevel
}

impl BlockError {

    pub fn from_message(message: String) -> Self {
        BlockError {
            message,
            error_level: ErrorLevel::Internal
        }
    } 

    pub fn from_message_and_error(message: String, error_level: ErrorLevel) -> Self {
        BlockError {
            message,
            error_level
        }
    } 

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn set_message(mut self, message: String) -> () {
        self.message = message;
    }

    pub fn get_error_level(&self) -> ErrorLevel {
        self.error_level.clone()
    }

    pub fn set_error_level(mut self, level: ErrorLevel) -> () {
        self.error_level = level;
    }
    
}