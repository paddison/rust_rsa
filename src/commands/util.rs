use std::{collections::HashMap, fs::File, io::Read};
use num_cpus;
use chrono::{self, Datelike, Timelike};

type Result<T> = std::result::Result<T, ParseCommandError>;

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

// Contains configuration structs for commands
pub trait Configuration {
    #[inline(always)]
    fn is_valid_bit_size(n: u16) -> bool {
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

    #[inline(always)]
    fn get_error_message(invalid_flag: &str, command: &str) -> String {
        format!("Invalid flag: {}. Enter {} -h for help", invalid_flag, command)
    }

    fn get_help_message() -> String;
}


// Small wrapper to indicate that something is a flag
#[derive(Hash, Eq)]
pub struct Flag {
    value: char,
}

impl Flag {
    // new should only be called after testing flag with is_flag()
    fn try_new(flag: &str) -> Option<Self> {
        if is_flag(flag) {
            let value = flag.chars().next_back().unwrap(); // unwrap is safe, since it is valid flag
            Some(Flag { value })
        } else {
            None
        }
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}


// Returns a map containing all flags that where specified, with their index in args
fn get_flags(args: &[String]) -> HashMap<Flag, usize> {
    let mut flags = HashMap::new(); 
    for (i, arg) in args.iter().enumerate() {
        if let Some(f) = Flag::try_new(arg) {
            flags.insert(f, i);
        }
    }
    flags
}

// checks if the argument is a flag or not (flags will always have the form of "-[a-zA-Z]")
pub fn is_flag(flag: &str) -> bool {
    let match_alphabetic = |s: char| -> bool {
        s.is_ascii_alphabetic()
    };
    flag.len() == 2 && flag.starts_with("-") && flag.ends_with(match_alphabetic)
}