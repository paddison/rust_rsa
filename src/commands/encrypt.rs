use std::{io::{Read, Write}, fs::File};

use crate::{opt, parser::{opt::FlagType, parser::OptParser}, key_gen::{RsaKey, RsaPublicKey, self}, input_module};

use super::util::{InitConfigError, Configuration};
use crate::parser::opt::OptDescriptor;

type Result<T> = std::result::Result<T, InitConfigError>;

const L_FILE: &str = "file";
const L_KEY: &str = "key";
const L_FROM: &str = "from";
const L_HELP: &str = "help";

const S_FILE: &str = "f";
const S_KEY: &str = "k";
const S_FROM: &str = "F";
const S_HELP: &str = "h";

// Encrypt a message 
// encrypt [OPTIONS] key_file message
// key_file: file containing rsa key
// message: message to be encrypted (can be from file, too)
// 
// OPTIONS:
// -f, --file [file_name] specify if message should be saved to file, will be 'out' if 'file_name' is empty
// -k, --key [private | public] if key for decryption is private or public (default is private) // NOT IMPLEMENTED
// -F, --from message comes from file, otherwise will be string
// -h, --help display help message for this command
pub struct EncryptConfig
{
    key_file: String,
    message: String,
    from_file: bool,
    use_private: bool,
    file: Option<String>,
    print_help: bool,
}

// message needs to be smaller than key size
impl EncryptConfig {
    pub fn init(args: &[String]) -> Result<Self> {
        let expected = vec![
            opt!(S_FILE, L_FILE, FlagType::SingleArg(true)),
            opt!(S_KEY, L_KEY, FlagType::SingleArg(false)),
            opt!(S_FROM, L_FROM, FlagType::NoArg),
            opt!(S_HELP, L_HELP, FlagType::NoArg),
        ];

        let message = match args.get(args.len() - 1) {
            Some(s) => s.trim().to_string(),
            None => return Err(InitConfigError { msg: "Error, no argument for message provided".to_string()}),
        };

        let key_file = match args.get(args.len() - 2) {
            Some(s) => s.clone(),
            None => return Err(InitConfigError { msg: "Error, no argument for key file provided".to_string()}),
        };

        let parser = OptParser::new(&args[..args.len() - 2], expected);
        let found_opts = Self::consume_parser(parser)?;
        let mut file = None;
        let mut use_private = false;
        let mut from_file = false;
        let mut print_help = false;

        for opt in found_opts {
            match opt.get_name() {
                L_FILE => match opt.consume() {
                    Some(file_name) => file = Some(file_name[0].clone()),
                    None => file = Some("out".to_string()),
                },
                L_KEY => match opt.consume().unwrap()[0].as_str() {
                    "public" => use_private = false,
                    "private" => use_private = true,
                    invalid => return Err(InitConfigError { msg: format!("Invalid parameter for -k/--key: {}, has to be 'public' or 'private'.", invalid)}),
                },
                L_FROM => from_file = true,
                L_HELP => print_help = true,
                invalid => return Err(InitConfigError { msg: format!("Parser returned invalid opt: {}", invalid) }),
            }
        }

        return Ok(EncryptConfig { key_file, message, from_file, use_private, file, print_help });
    }
}

impl Configuration for EncryptConfig {
    fn get_help_message() -> String {
        "Usage:\n\n\
        encrypt [OPTIONS] key_file message \n\
        key_file: file containing rsa key\n\
        message: message to be encrypted (can be from file, too)\n\n\
        OPTIONS:\n\
        -f, --file [file_name] specify if message should be saved to file, will be 'out' if 'file_name' is empty\n\
        -k, --key [private | public] if key for decryption is private or public (default is private) // NOT IMPLEMENTED\n\
        -F, --from message comes from file, otherwise will be string\n\
        -h, --help display help message for this command\n".to_string()
    }
}

pub fn run(config: EncryptConfig) {
    if config.print_help {
        println!("{}", EncryptConfig::get_help_message());
        return;
    }
    let key = if config.use_private {
        eprintln!("Error, encryption via private key is not yet implemented!");
        return;
    } else {
        match RsaPublicKey::from_file(&config.key_file) {
            Ok(key) => key,
            Err(_) => { 
                eprintln!("Unable to open key file {}, or file contains invalid public key", config.key_file);
                return;
            }
        }
    };
    
    // get message either from file or from config
    let message = if config.from_file {
        match std::fs::File::open(&config.message) {
            Ok(mut f) => {
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => buf,
                    Err(e) => {
                        eprintln!("Unable to read file content: {}", e);
                        return;
                    }
                } 
            },
            Err(_) => {
                eprintln!("Unable to open file: {}", config.message);
                return;
            }
        }
    } else {
        config.message
    };
    
    // parse message to Integer, so it can be encrypted
    let integer_message = input_module::string_to_number(message);
    let cipher = key_gen::encrypt_msg(&integer_message, key);
    let string_cipher = RsaPublicKey::into_hex(&cipher);

    match config.file {
        Some(file_name) => {
            match File::create(&file_name) {
                Ok(mut f) => { 
                    match f.write_all(string_cipher.as_bytes()) {
                        Ok(_) => println!("Stored cipher to {}", file_name),
                        Err(e) => eprintln!("Couldn't write cipher to file {}: {}", file_name, e)
                    }
                },
                Err(_) => eprintln!("Unable to create file: {}", file_name),
            }
        },
        None => {
            println!("Cipher is:\n{}", string_cipher)
        }
    }
}