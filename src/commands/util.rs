use chrono::{self, Datelike, Timelike};

// TODO group error types with trait maybe?
type Result<T> = std::result::Result<T, InitConfigError>;

#[derive(Debug)]
pub struct InitConfigError {
    pub msg: String,
}

impl InitConfigError {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug)]
pub struct ParseCommandError {
    err_type: ErrorType,
}

impl From<ErrorType> for ParseCommandError {
    fn from(err_type: ErrorType) -> Self {
        ParseCommandError { err_type }
    }
}

impl ParseCommandError {
    pub fn get_message(&self) -> &str {
        match &self.err_type {
            ErrorType::InvalidFlag(msg) => msg,
            ErrorType::HelpFlag(msg) => msg,
            ErrorType::InvalidBitSize(msg) => msg,
            ErrorType::InvalidArgs(msg) => msg,
            ErrorType::Other(msg) => msg,
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    InvalidFlag(String),
    HelpFlag(String),
    InvalidBitSize(String),
    InvalidArgs(String),
    Other(String),
}

// check if file already exists and return error if so
#[inline(always)]
pub fn verify_file_name(file_name: &str) -> Result<String> {
    if let Ok(_) = std::fs::File::open(&file_name) {
        return Err(InitConfigError { msg: format!("File {} already exists.", file_name)});
    }
    Ok(file_name.to_string())
}

#[inline(always)]
pub fn is_valid_bit_size(n: u32) -> bool {
    // power of two will have one bit set
    // check if n & n - 1 == 0, (100 & 011 == 000)
    n <= 8196 && n >= 128 && (n & (n - 1) == 0) 
}

fn parse_file_name(args: &[String]) -> String {
    let mut args_iter = args.iter();
    args_iter.next();
    if let Some(name) = args_iter.next() {
        String::from(name)
    } else {
        let now = chrono::Utc::now();
        format!("{}-{}-{}T{}:{}", now.day(), now.month(), now.year(), now.hour(), now.minute())
    }
}