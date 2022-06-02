use crate::commands::util::*;

type Result<T> = std::result::Result<T, InitDecryptError>;

#[derive(Debug)]
pub struct InitDecryptError {
    msg: String,
}

impl InitDecryptError {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}

// decrypt a message
// decrypt [options] key_file message
// flags:
// -f [file_name] 
// specify if message should be saved to file
// -F
// message comes from file, otherwise will be string
// -h
// show help for this command
//
// Note: file_header should contain information about the key that was used to encrypt
struct DecryptConfig {
    key_file: String,
    file: Option<String>,
    message: String,
}

impl Configuration for DecryptConfig  {
    fn get_help_message() -> String {
        todo!()
    }
}