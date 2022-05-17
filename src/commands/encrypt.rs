use std::{fs::File, io::Read};

use crate::commands::util::*;

type Result<T> = std::result::Result<T, ParseCommandError>;

// encrypt a message 
// encrypt [OPTIONS] key_file message
// 
// flags:
// -f [file_name] specify if message should be saved to file
// -s use private key to encrypt
// -F message comes from file, otherwise will be string
// -h display help for this command
pub struct EncryptConfig
{
    key_file: String,
    is_private: bool,
    file: Option<String>,
    message: String,
}

impl EncryptConfig{
    pub fn new(args: &[String]) -> Result<Self> {
        if args.len() < 2 {
            return Err(ParseCommandError::from(ErrorType::InvalidArgs("Key_file and message required.".to_string())));
        }
        let mut is_private = false;
        let mut file: Option<String> = None;
        let mut message = String::new();
        for (i, arg) in args[0..args.len() - 2].iter().enumerate() {
            if is_flag(arg) {
                match arg.as_str() {
                    "-s" => is_private = true,
                    "-F" => message = Self::parse_message(&args[i + 1..])?,
                    "-f" => file = Some(Self::parse_file_name(&args[i..])),
                    "-h" => {
                        let err_type = ErrorType::HelpFlag(Self::get_help_message());
                        return Err(ParseCommandError::from(err_type));
                    },
                    invalid_flag => {
                        let err_type = ErrorType::InvalidFlag(Self::get_error_message(invalid_flag, "encrypt"));
                        return Err(ParseCommandError::from(err_type));
                    }
                }
            }   
        }

        if message.len() == 0 {
            message = args[args.len() - 1].clone();
        }

        let key_file = args[args.len() - 2].clone();
        Ok(EncryptConfig { key_file, is_private, file, message })
    }

    fn parse_message(args: &[String]) -> Result<String> {
        let file_name = match args.get(1) {
            Some(s) => s,
            None => {
                let err_type = ErrorType::InvalidArgs("No file name given.".to_string());
                return Err(ParseCommandError::from(err_type));
            },
        };
        let mut f = match File::open(file_name)  {
            Ok(f) => f,
            Err(e) => {
                let err_type = ErrorType::Other(e.to_string());
                return Err(ParseCommandError::from(err_type));
            },
        };
        let mut message = String::new();
        match f.read_to_string(&mut message) {
            Ok(_) => Ok(message),
            Err(e) => {
                let err_type = ErrorType::Other(e.to_string());
                Err(ParseCommandError::from(err_type))
            }
        }
    }
}

impl Configuration for EncryptConfig {
    fn get_help_message() -> String {
        todo!()
    }
}