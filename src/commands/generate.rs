use std::io::Write;

use crate::{commands::util::*, opt, parser::{opt::FlagType, parser::{OptParser}}, key_gen::{self, RsaKey}};
use crate::parser::opt::OptDescriptor;
use chrono::Local;

use super::util;

type Result<T> = std::result::Result<T, InitConfigError>;

const L_SIZE: &str = "size";
const L_FILE: &str = "file";
const L_HELP: &str = "help";

const S_SIZE: &str = "s";
const S_FILE: &str = "f";
const S_HELP: &str = "h";

#[derive(Debug)]
pub struct InitGenerateError {
    msg: String,
}

impl InitGenerateError {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}
// Generate a key pair, default size is set to 2048 bit
// generate [OPTIONS]
// OPTIONS:
// -s, --size n specify length n of key in bits, only powers of 2 permitted
// -f, --file file_name save keypair to file, if file_name is empty, create file with date and size.
// -h, --help display help message for this command
// show help for this command
pub struct GenerateConfig {
    size: u32,
    file: Option<String>,
    print_help: bool,
}

impl GenerateConfig {
    pub fn init(args: &[String]) -> Result<Self> {
        let expected = vec![
            opt!(S_SIZE, L_SIZE, FlagType::MultiArg(false)),
            opt!(S_FILE, L_FILE, FlagType::SingleArg(true)),
            opt!(S_HELP, L_HELP, FlagType::NoArg),
        ];

        // get parser and get all opts
        let parser = OptParser::new(args, expected);
        let found_opts = Self::consume_parser(parser)?;

        // set default values
        let mut size = 2048;
        let mut file = None;
        let mut print_help = false;
        // there is a bug, where if file is parsed before size, that size will be invalid 
        for opt in found_opts {
            match opt.get_name() {
                L_SIZE => size = Self::parse_bit_size(opt.consume().unwrap()[0].clone())?,
                L_FILE =>  match opt.consume() {
                    Some(name) => file = Some(util::verify_file_name(&name[0])?),
                    None => {
                        let s = Local::now().format("%y-%m-%dT%H:%M").to_string();
                        file = Some(format!("{}", s)); 
                    }
                },
                L_HELP => print_help = true,
                invalid => {
                    return Err(InitConfigError{ msg: format!("Parser returned invalid argument: {}", invalid) })
                }
            }
        }

        Ok(GenerateConfig { size, file, print_help })
    }

    fn parse_bit_size(size: String) -> Result<u32> {
        match size.parse::<u32>() {
            Ok(n) => {
                if Self::is_valid_bit_size(n) {
                    Ok(n)
                } else {
                    Err(InitConfigError{ msg: format!("Invalid key size: {}", n)})
                }
            }
            Err(_) => Err(InitConfigError{ msg: format!("Unable to parse input to number: {}", size)}),
        }
    }

}

impl Configuration for GenerateConfig {
    fn get_help_message() -> String {
        "Usage:\n\n\
        generate [OPTIONS]\n\n\
        OPTIONS:\n\
        -s, --size n specify length n of key in bits, only powers of 2 permitted\n\
        -f, --file file_name save keypair to file, if file_name is empty, create file with name = creation date\n\
        -h, --help display help message for this command".to_string()
    }
}

pub fn run(config: GenerateConfig) {
    if config.print_help {
        return println!("{}", GenerateConfig::get_help_message());
    }
    println!("Generating {} bit key pair...", config.size);
    let (sk, pk) = key_gen::generate_key_pair(config.size, num_cpus::get_physical());    
    
    match config.file {
        Some(file_name) => {
            // write secret key
            write_to_file(sk, true, &file_name);
            write_to_file(pk, false, &file_name);
        }, 
        None => {
            println!("RSA Keys:");
            println!("{}", sk.serialize())
        },
    }
}

fn write_to_file<K: RsaKey>(key: K, is_private: bool, file_name: &str) {
    let (prefix, key_type) = if is_private { ("sk_".to_string(), "private") } else { ("pk_".to_string(), "public key") }; 
    let file_name = prefix + &file_name;
    let f_pk = std::fs::File::create(&file_name);
            match f_pk {
                Ok(mut f) => {
                    let key_string = key.serialize();
                    let _ = f.write_all(key_string.as_bytes());
                    println!("Wrote {key_type} to file: {file_name}");
                },
                Err(e) => {
                    eprintln!("Error creating {key_type} file: {e}.\nKey has not been saved")
                }
            } 
}

