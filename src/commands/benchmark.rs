use crate::commands::util::*;

type Result<T> = std::result::Result<T, ParseCommandError>;

// benchmark commands:
// benchmark [OPTIONS]
// OPTIONS:
// -s, --size=<1024,2048,...> size of keys to be used, from 128 to 8192, 1204 and 2048 if empty
// -t, --threads=<1,2,3,...> number of threads to be used, num cpus if empty
// -f, --file <file_name> save results to a file, bm.txt if empty
// -h, --help print help for this command
#[derive(Debug)]
pub struct BenchmarkConfig {
    bit_sizes: Vec<u16>,
    n_threads: Vec<u8>,
    file: Option<String>,
}

impl BenchmarkConfig {
    pub fn new(args: &[String]) -> Result<Self> {
        let mut bit_sizes = vec![2048];
        let mut n_threads = vec![num_cpus::get_physical() as u8];
        let mut file = None;

        for (i, arg) in args.iter().enumerate() {
            if is_flag(arg) {
                match arg.as_str() {
                    "-b" => bit_sizes = Self::parse_bit_sizes(&args[(i + 1)..])?,
                    "-t" => n_threads = Self::parse_n_threads(&args[(i + 1)..]),
                    "-f" => file = Some(Self::parse_file_name(&args[i..])),
                    "-h" => { 
                        let err_type = ErrorType::HelpFlag(Self::get_help_message());
                        return Err(ParseCommandError::from(err_type));
                    },
                    invalid_flag => { 
                        let err_type = ErrorType::InvalidFlag(Self::get_error_message(invalid_flag, "benchmark"));
                        return Err(ParseCommandError::from(err_type));
                    },
                }
            }
        }
        
        Ok(BenchmarkConfig { bit_sizes, n_threads, file })
    }

    fn parse_bit_sizes(args: &[String]) -> Result<Vec<u16>> {
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
                    let err_msg = format!("Not in range or not power of 2: {}.", n);
                    let err_type = ErrorType::InvalidBitSize(err_msg);
                    return Err(ParseCommandError::from(err_type));
                }
            }
        }
        // if input was empty 
        if bit_sizes.len() == 0 {
            let err_msg = format!("No key length parameters found or non numeric inputs.");
            let err_type = ErrorType::InvalidBitSize(err_msg);
            return Err(ParseCommandError::from(err_type));
        }
        Ok(bit_sizes)
    }

    // TODO update to errortype maybe
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
    fn get_help_message() -> String {
        "TODO".to_string()
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
        assert!(cfg.is_err());
    }

    #[test]
    fn test_new_b_valid() {
        let args = vec!["-b".to_string(), "1024".to_string(), "2048".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![1024, 2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());

    }

    #[test]
    fn test_new_b_invalid() {
        let args = vec!["-b".to_string(), "bla".to_string(), "blub".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());
    }

    #[test]
    fn test_new_b_one_invalid() {
        let args = vec!["-b".to_string(), "bla".to_string(), "1024".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![1024]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());
    }

    #[test]
    fn test_new_t_valid() {
        let args = vec!["-t".to_string(), "5".to_string(), "10".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![5, 10]);
        assert!(cfg.file.is_none());
    }

    #[test]
    fn test_new_t_invalid() {
        let args = vec!["-t".to_string(), "bla".to_string(), "blub".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_none());
    }
    #[test]
    fn test_new_f_with_name() {
        let args = vec!["-f".to_string(), "my_file".to_string()];
        let cfg = BenchmarkConfig::new(&args);
        assert!(cfg.is_ok());
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
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.bit_sizes, vec![2048]);
        assert_eq!(cfg.n_threads, vec![num_cpus::get_physical() as u8]);
        assert!(cfg.file.is_some());
        let now = chrono::Utc::now();
        let expected = format!("{}-{}-{}T{}:{}", now.day(), now.month(), now.year(), now.hour(), now.minute());
        assert_eq!(cfg.file.unwrap(), expected);
    }
}