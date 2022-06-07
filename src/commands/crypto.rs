use std::io::Read;

use crate::{opt, parser::{opt::FlagType, parser::OptParser}};

use super::util::InitConfigError;
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
pub struct CryptoConfig
{
    key_file: String,
    message: String,
    from_file: bool,
    use_private: bool,
    file: Option<String>,
    print_help: bool,
}

// message needs to be smaller than key size
impl CryptoConfig {
    pub fn init(args: &[String], do_encrypt: bool) -> Result<Self> {

        // manually check if only help flag was given (not optimal, but watcha gonna do)
        if args.len() < 3 {
            for arg in args {
                if arg == "-h" || arg == "--help" {
                    return Err(InitConfigError { msg: if do_encrypt { encrypt::get_help_message()} else { decrypt::get_help_message() }});
                }
            }
        }

        let expected = vec![
            opt!(S_FILE, L_FILE, FlagType::SingleArg(true)),
            opt!(S_KEY, L_KEY, FlagType::SingleArg(false)),
            opt!(S_FROM, L_FROM, FlagType::NoArg),
            opt!(S_HELP, L_HELP, FlagType::NoArg),
        ];



        let message = match args.get(args.len().wrapping_sub(1)) {
            Some(s) => s.trim().to_string(),
            None => return Err(InitConfigError { msg: "Error, no argument for message provided".to_string()}),
        };

        let key_file = match args.get(args.len().wrapping_sub(2)) {
            Some(s) => s.clone(),
            None => return Err(InitConfigError { msg: "Error, no argument for key file provided".to_string()}),
        };

        let parser = OptParser::new(&args[..args.len() - 2], expected);
        let found_opts = parser.consume()?;
        let mut file = None;
        let mut use_private = if do_encrypt { false } else { true };
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

        return Ok(CryptoConfig { key_file, message, from_file, use_private, file, print_help });
    }

    fn get_message(&self) -> std::io::Result<String> {
        if self.from_file {
            let mut f = std::fs::File::open(&self.message)?;
            let mut buf = String::new();
            let _ = f.read_to_string(&mut buf)?;
            Ok(buf)
        } else {
            Ok(self.message.clone())
        }
    }
}

pub mod encrypt {
    use std::{io::Write, fs::File};

    use crate::{key_gen::{RsaPublicKey, RsaKey, self}, input_module};

    use super::CryptoConfig;

    #[inline(always)]
    pub fn get_help_message() -> String {
        "Usage:\n\n\
        encrypt [OPTIONS] key_file message \n\
        key_file: file containing rsa key\n\
        message: message to be encrypted (can be from file, too)\n\n\
        OPTIONS:\n\
        -f, --file [file_name] specify if message should be saved to file, will be 'out' if 'file_name' is empty\n\
        -k, --key [private | public] if key for encryption is private or public (default is private) // NOT IMPLEMENTED\n\
        -F, --from message comes from file, otherwise will be string\n\
        -h, --help display help message for this command\n".to_string()
    }
    
    pub fn run(config: CryptoConfig) {
        if config.print_help {
            println!("{}", get_help_message());
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
        let message = match config.get_message() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error while getting message from file: {}", e); 
                return;
            }
        };
        
        // parse message to Integer, so it can be encrypted
        let integer_message = input_module::string_to_number(message);
        println!("{}", integer_message);
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
}

// TODO still too much copy & paste, need to find better way to store all information 
// maybe add boolean to config to indicate if decrypt/encrypt command was called
// then in run, branch according to decrypt/ encrypt. For help messages, consider adding boolean to switch words decrypt/encrypt
// also implement both decryption/encryption with both keys. can be done by just calling different functions
pub mod decrypt {
    use std::{fs::File, io::Write};

    use rug::{Integer, Complete};

    use crate::{key_gen::{RsaKey, self, RsaPrivateKey}, input_module};

    use super::CryptoConfig;

    #[inline(always)]
    pub fn get_help_message() -> String {
        "Usage:\n\n\
        decrypt [OPTIONS] key_file message \n\
        key_file: file containing rsa key\n\
        message: message to be decrypt (can be from file, too)\n\n\
        OPTIONS:\n\
        -f, --file [file_name] specify if message should be saved to file, will be 'out' if 'file_name' is empty\n\
        -k, --key [private | public] if key for decryption is private or public (default is private) // NOT IMPLEMENTED\n\
        -F, --from message comes from file, otherwise will be string\n\
        -h, --help display help message for this command\n".to_string()
    }

    pub fn run(config: CryptoConfig) {
        if config.print_help {
            println!("{}", get_help_message());
            return;
        }
        
        let key = if config.use_private {
            match RsaPrivateKey::from_file(&config.key_file) {
                Ok(key) => key,
                Err(_) => { 
                    eprintln!("Unable to open key file {}, or file contains invalid public key", config.key_file);
                    return;
                }
            }
        } else {
            eprintln!("Error, decryption via public key is not yet implemented!");
            return;
        };

        let cipher = match config.get_message() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error while getting cipher from file: {}", e); 
                return;
            }
        };

        let integer_cipher = match Integer::parse_radix(cipher, 16) {
            Ok(incomplete) => incomplete.complete(),
            Err(_) => {
                eprintln!("Unable to parse cipher to integer.");
                return;
            }
        };

        let integer_message = key_gen::decrypt_cypher(&integer_cipher, key);
        println!("{}", integer_message);
        let message = match input_module::number_to_string(integer_message) {
            Ok(m) => m,
            Err(_) => {
                eprintln!("Couldn't convert message to string. Original message may have contained invalid utf8");
                return;
            }
        };

        match config.file {
            Some(file_name) => {
                match File::create(&file_name) {
                    Ok(mut f) => { 
                        match f.write_all(message.as_bytes()) {
                            Ok(_) => println!("Stored message to {}", file_name),
                            Err(e) => eprintln!("Couldn't write message to file {}: {}", file_name, e)
                        }
                    },
                    Err(_) => eprintln!("Unable to create file: {}", file_name),
                }
            },
            None => {
                println!("Message is:\n{}", message)
            }
        }
    }
}

// TODO testitests