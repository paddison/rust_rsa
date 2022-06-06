use crate::commands::util::*;

type Result<T> = std::result::Result<T, InitConfigError>;



// Decrypt a message
// decrypt [OPTIONS] key_file message
// key_file: file containing public key
// OPTIONS:
// flags:
// -f [file_name] specify if message should be saved to file
// -F 'message' parameter comes from file
// -k [private | public] if key for decryption is private or public (default is private)
// -h display help message for this command
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