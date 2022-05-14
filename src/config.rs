use std::collections::HashMap;
use num_cpus;
use chrono::{self, Datelike, Timelike};

use crate::key_gen::RsaKey;

// Contains configuration structs for commands
trait Configuration {
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

    fn print_error(invalid_flag: &str, command: &str) {
        println!("Invalid flag: {}. Enter {} -h for help", invalid_flag, command);
    }

    fn print_help();
}
// benchmark commands:
// benchmark [OPTIONS]
// if no flags are specified, it will go from 1k to 4k
// with n_thread = num_cpus 
// flags:
// -b:
// bitsizes can be specified with -b [1024,2048,...]
// bitsizes can be entered as a comma separated list of numbers
// bitsizes have to be a power of two in range of 128 to 8192
// invalid bitsizes will be ignored, but user should be notified
// -t:
// num of threads with -t [t1,t2,t3,...,tn]
// threads can be entered as a comma separated list of numbers
// if none specified run with threads equal to amount of cpu cores 
// add flag to specify number of threads and bit sizes
// -f [file_name]
// save results to a file 
// if [file_name] is empty, a default name with the date and time is created
// -h
// show help for this command
#[derive(Debug)]
pub struct BenchmarkConfig {
    bit_sizes: Vec<u16>,
    n_threads: Vec<u8>,
    file: Option<String>,
}

impl BenchmarkConfig {
    pub fn new(args: &[String]) -> Option<Self> {
        let mut bit_sizes = vec![2048];
        let mut n_threads = vec![num_cpus::get_physical() as u8];
        let mut file = None;

        for (i, arg) in args.iter().enumerate() {
            if is_flag(arg) {
                match arg.as_str() {
                    "-b" => bit_sizes = Self::parse_bit_sizes(&args[(i + 1)..]),
                    "-t" => n_threads = Self::parse_n_threads(&args[(i + 1)..]),
                    "-f" => file = Some(Self::parse_file_name(&args[i..])),
                    "-h" => { 
                        Self::print_help(); 
                        return None;
                    },
                    invalid_flag => { 
                        Self::print_error(invalid_flag, "benchmark"); 
                        return None;
                    },
                }
            }
        }
        
        Some(BenchmarkConfig { bit_sizes, n_threads, file })
    }

    fn parse_bit_sizes(args: &[String]) -> Vec<u16> {
        let mut bit_sizes = Vec::new();
        // loop through args until next flag is found
        for arg in args {
            if is_flag(arg) {
                break;
            }
            // check if n is a valid bitsize
            if let Ok(n) = arg.parse::<u16>() {
                if Self::is_valid_bit_size(n) {
                    bit_sizes.push(n)
                } else {
                    println!("Not in range or not power of 2: {}. Will be ignored", n);
                }
            }
        }
        // if input was empty or numbers where invalid bitsizes, push default values
        if bit_sizes.len() == 0 {
            println!("Entered bit sizes where invalid, defaulting to 2048");
            bit_sizes.push(2048)
        }
        bit_sizes
    }

    fn parse_n_threads(args: &[String]) -> Vec<u8> {
        let mut n_threads = vec![];
        for arg in args {
            if is_flag(arg) {
                break;
            }
            if let Ok(n) = arg.parse::<u8>() {
                n_threads.push(n);
            } else {
                println!("Invalid number for amount of threads: {}, will be ignored", arg)
            }
        }
        if n_threads.len() == 0 {
            n_threads.push(num_cpus::get_physical() as u8);
        }
        n_threads
    }
}

impl Configuration for BenchmarkConfig {
    fn print_help() {
        println!("TODO")
    }
}

// TODO implement many tests
#[cfg(test)]
mod benchmark_tests {
    use chrono::{Datelike, Timelike};

    use super::BenchmarkConfig;

    #[test]
    fn test_new_invalid() {
        let args = vec!["-b".to_string(), "1024".to_string(), 
                        "-t".to_string(), "5".to_string(), 
                        "-g".to_string(), "bla".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_none());
    }

    #[test]
    fn test_new_b_valid() {
        let args = vec!["-b".to_string(), "1024".to_string(), "2048".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![1024, 2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());

    }

    #[test]
    fn test_new_b_invalid() {
        let args = vec!["-b".to_string(), "bla".to_string(), "blub".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());
    }

    #[test]
    fn test_new_b_one_invalid() {
        let args = vec!["-b".to_string(), "bla".to_string(), "1024".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![1024]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());
    }

    #[test]
    fn test_new_t_valid() {
        let args = vec!["-t".to_string(), "5".to_string(), "10".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![5, 10]);
        assert!(cfg.file.is_none());
    }

    #[test]
    fn test_new_t_invalid() {
        let args = vec!["-t".to_string(), "bla".to_string(), "blub".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());
    }
    #[test]
    fn test_new_f_with_name() {
        let args = vec!["-f".to_string(), "my_file".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_some());
        assert_eq!(cfg.file.unwrap(), "my_file".to_string());
    }

    #[test]
    fn test_new_f_without_name() {
        let args = vec!["-f".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_some());
        let now = chrono::Utc::now();
        let expected = format!("{}-{}-{}T{}:{}", now.day(), now.month(), now.year(), now.hour(), now.minute());
        assert_eq!(cfg.file.unwrap(), expected);
    }
}

// Generate a key pair
// generate [OPTIONS]
// flags:
// -b [n]
// specify length n of key in bits, only powers of 2 permitted
// if empty, generate 2k key
// -f [file_name]
// save results to file,
// if [file_name] is empty, a default name with the date and time is created
// -h
// show help for this command
pub struct GenerateConfig {
    length: u16,
    file: Option<String>,
}

impl GenerateConfig {
    pub fn new(args: &[String]) -> Option<Self> {
        let mut length = 2048;
        let mut file = None;

        for (i, arg) in args.iter().enumerate() {
            if is_flag(arg) {
                match arg.as_str() {
                    "-b" => length = Self::parse_key_length(&args[i..]),
                    "-f" => file = Some(Self::parse_file_name(&args[i..])),
                    "-h" => {
                        Self::print_help();
                        return None;
                    }
                    invalid_flag => { 
                        Self::print_error(invalid_flag, "generate");
                        return None;
                    }
                }
            }
        } 
        Some(GenerateConfig { length, file })
    }

    #[inline(always)]
    fn parse_key_length(args: &[String]) -> u16 {
        let mut key_length = 2048;
        if let None = args.get(1) {
            println!("No key length specified, defaulting to 2048");
            return key_length;
        }
        if let Ok(n) = args[1].parse::<u16>() {
            if Self::is_valid_bit_size(n) {
                key_length = n;
            } else {
                println!("Not a number or invalid key lenght: {}, defaulting to 2048", args[1])
            }
        }
        
        key_length
    }
}

impl Configuration for GenerateConfig {
    fn print_help() {
        println!("None");
    }
}

// encrypt a message 
// encrypt [OPTIONS] key_file message
// 
// flags:
// -f [file_name] 
// specify if message should be saved to file
// -p
// use public key to encrypt (standard option)
// -s
// use private key to encrypt
// -F
// message comes from file, otherwise will be string
// -h
struct EncryptConfig<Key> 
where Key: RsaKey
{
    key: Key,
    is_private: bool,
    file: Option<String>,
    message: String,
}

impl<Key> Configuration for EncryptConfig<Key> where Key: RsaKey {
    fn print_help() {
        println!("TODO");
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
struct DecryptConfig<Key> 
where Key: RsaKey
{
    key: Key,
    file: Option<String>,
    message: String,
}

impl<Key> Configuration for DecryptConfig<Key> where Key: RsaKey {
    fn print_help() {
        println!("TODO");
    }
}

// Small wrapper to indicate that something is a flag
#[derive(Hash, Eq)]
struct Flag {
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
fn is_flag(flag: &str) -> bool {
    let match_alphabetic = |s: char| -> bool {
        s.is_ascii_alphabetic()
    } ;
    flag.len() == 2 && flag.starts_with("-") && flag.ends_with(match_alphabetic)
}