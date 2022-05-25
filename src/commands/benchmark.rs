use crate::parser::parser::{OptParser, ParseFlagError};
use crate::{opt, parser::opt::FlagType};
use crate::parser::opt::OptDescriptor;
use std::{collections::HashMap, time};
use crate::key_gen;

use super::util::Configuration;

type Result<T> = std::result::Result<T, InitBenchmarkError>;

const L_SIZE: &str = "size";
const L_THREADS: &str = "threads";
const L_FILE: &str = "file";
const L_HELP: &str = "help";
const L_REPEATS: &str = "repeats";

const S_SIZE: &str = "s";
const S_THREADS: &str = "t";
const S_FILE: &str = "f";
const S_HELP: &str = "h";
const S_REPEATS: &str = "r";

#[derive(Debug)]
pub struct InitBenchmarkError {
    msg: String,
}

impl InitBenchmarkError {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}

// // benchmark commands:
// // benchmark [OPTIONS]
// // OPTIONS:
// // -s, --size <1024 2048 ...> size of keys to be used, from 128 to 8192, 1204 and 2048 if empty
// // -t, --threads <1 2 3 ...> number of threads to be used, num cpus if empty
// // -f, --file <file_name> save results to a file, bm.txt if empty
// // -h, --help print help for this command
// -r, --repeats number of repeats, defaults to 5
#[derive(Debug)]
pub struct BenchmarkConfig {
    pub bit_sizes: Vec<u32>,
    pub n_threads: Vec<usize>,
    pub file: Option<String>,
    pub repeats: u16,
    pub print_help: bool,
}

impl BenchmarkConfig {
    pub fn init(args: &[String]) -> Result<Self> {
        let expected = vec![
            opt!(S_SIZE, L_SIZE, FlagType::MultiArg(true)),
            opt!(S_THREADS, L_THREADS, FlagType::MultiArg(true)),
            opt!(S_FILE, L_FILE, FlagType::SingleArg(true)),
            opt!(S_HELP, L_HELP, FlagType::NoArg),
            opt!(S_REPEATS, L_REPEATS, FlagType::SingleArg(true)),
        ];

        let mut parser = OptParser::new(args, expected);
        let mut found_opts = vec![];

        // get all the options from the provided arguments
        while let Some(result) = parser.next() {
            match result {
                Ok(found_opt) => found_opts.push(found_opt),
                Err(e) => {
                    let msg = match e {
                        ParseFlagError::ArgRequired(flag) => format!("No arguments provided for: {}", flag),
                        ParseFlagError::InvalidOpt(flag) => format!("Invalid option: {}", flag),
                    };
                    return Err(InitBenchmarkError { msg });
                }
            }
        }

        // Go through found options and create config accordingly
        let mut bit_sizes = vec![2048];
        let mut n_threads = vec![num_cpus::get_physical()];
        let mut file = None;
        let mut print_help = false;
        let mut repeats = 5;
        for opt in found_opts {
            match opt.get_name() {
                L_SIZE => if let Some(size_strings) = opt.consume() {
                    bit_sizes = Self::parse_bit_sizes(size_strings)?;
                }
                L_THREADS => if let Some(thread_strings) = opt.consume() {
                    n_threads = Self::parse_n_threads(thread_strings)?;
                },
                L_FILE => match opt.consume() {
                    // indexing directly is safe, since file refers to SingleArg and parser returns None
                    // if no arg was provided, otherwise file_name will hold exactly one value
                    Some(file_name) => file = Some(file_name[0].clone()),
                    None => file = Some("bm.txt".to_string()),
                    
                },
                L_HELP => print_help = true,
                L_REPEATS => {
                    if let Some(repeats_strings) = opt.consume() {
                        if let Ok(n) = repeats_strings[0].parse::<u16>() {
                            repeats = n;
                        } else {
                            return Err(InitBenchmarkError { msg: format!("Unable to parse number of repeats: {}", repeats_strings[0])});
                        }
                        
                    }
                }
                invalid => return Err(InitBenchmarkError { msg: format!("Parser returned invalid opt: {}", invalid) }),
            }
        }

        Ok(BenchmarkConfig { bit_sizes, n_threads, file, repeats, print_help })
    }

    #[inline(always)]
    fn parse_bit_sizes(size_strings: Vec<String>) -> Result<Vec<u32>> {
        let mut parsed_sizes = vec![];

        for size in size_strings {
            if let Err(_) = size.parse::<u32>() {
                return Err(InitBenchmarkError { msg: format!("Unable to parse bit size: {}", size )});
            }
            let n = size.parse::<u32>().unwrap();
            if !Self::is_valid_bit_size(n) {
                return Err(InitBenchmarkError { msg: format!("Invalid bit size: {}, needs to be in range of 128 to 8192 and power of 2.", n)});
            }
            parsed_sizes.push(n);
        }

        Ok(parsed_sizes)
    }

    #[inline(always)]
    fn parse_n_threads(thread_strings: Vec<String>) -> Result<Vec<usize>> {
        let mut parsed_threads = vec![];

        for n_threads in thread_strings {
            if let Err(_) = n_threads.parse::<usize>() {
                return Err(InitBenchmarkError { msg: format!("Unable to parse number of threads: {}", n_threads )});
            }
            parsed_threads.push(n_threads.parse::<usize>().unwrap());
        }
        Ok(parsed_threads)
    }
}

impl Configuration for BenchmarkConfig {
    fn get_help_message() -> String {
        "Usage:\n\n\
        benchmark [OPTIONS]\n\n\
        OPTIONS:\n\
        -s, --size <1024 2048 ...> size of keys to be used, from 128 to 8192, 1204 and 2048 if empty\n\
        -t, --threads <1 2 3 ...> number of threads to be used, num cpus if empty\n\
        -f, --file <file_name> save results to a file, bm.txt if empty\n\
        -h, --help print help for this command\n\
        -r, --repeats number of repeats, defaults to 5".to_string()
    }
}

fn benchmark_threads(repeats: u16, n_threads: &Vec<usize>, bit_sizes: &Vec<u32>)  -> HashMap<u32, Vec<(usize, u128)>>{
    let mut benchmark_results = HashMap::new();

    for bit_size in bit_sizes {
        let mut entry = vec![];
        for t in n_threads {
            let mut single_result = (*t, 0);
            for _ in 0..repeats {
                single_result.1 += benchmark_generate_key_pair(*bit_size, *t)
            }
            single_result.1 /= repeats as u128;
            entry.push(single_result);
        }
        benchmark_results.insert(*bit_size, entry);
    }
    benchmark_results
}

fn print_results(benchmark_results: HashMap<u32, Vec<(usize, u128)>>) {
    for (_, _) in benchmark_results {

    }
}

pub fn run(config: BenchmarkConfig) {
    if config.print_help {
        println!("{}", BenchmarkConfig::get_help_message());
        return;
    }
    let benchmark_results = benchmark_threads(config.repeats, &config.n_threads, &config.bit_sizes);
    if let Some(_) = config.file {
        // store results
    }
    print_results(benchmark_results);
}

fn benchmark_generate_key_pair(bits: u32, n_threads: usize) -> u128 {
    let start = time::Instant::now();
    let (_, _) = key_gen::generate_key_pair(bits, n_threads);
    println!("Created {} bit key pair in {}, with {} threads", bits, start.elapsed().as_millis(), n_threads);
    start.elapsed().as_millis()
}

#[cfg(test)]
mod tests {
    use super::BenchmarkConfig;

    #[test]
    fn test_init_valid() {
        let args = (vec!["benchmark", "-f", "blub", "--threads", "2", "4", "6", "--size", "512", "1024", "2048", "-h"])
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let config = BenchmarkConfig::init(&args[1..]);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.bit_sizes, vec![512, 1024, 2048]);
        assert!(config.print_help);
        assert_eq!(config.n_threads, vec![2, 4, 6]);
        assert_eq!(config.file.unwrap(), "blub");
    }

    #[test]
    fn test_init_defaults() {
        let args = vec!["-f".to_string(), "-t".to_string(), "-s".to_string()];

        let config = BenchmarkConfig::init(&args);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.bit_sizes, vec![2048]);
        assert_eq!(config.n_threads, vec![num_cpus::get_physical()]);
        assert_eq!(config.file.unwrap(), "bm.txt");
    }

    #[test]
    fn test_init_invalid_flag() {
        let args = vec!["-f".to_string(), "file.txt".to_string(), "--henlo".to_string()];
        let config = BenchmarkConfig::init(&args);
        assert!(config.is_err());
        assert_eq!(config.unwrap_err().msg, "Invalid option: --henlo");
    }

    #[test]
    fn test_init_invalid_bit_size() {
        let args = vec!["-s".to_string(), "512".to_string(), "1023".to_string()];
        let config = BenchmarkConfig::init(&args);
        assert!(config.is_err());
        assert_eq!(config.unwrap_err().msg, "Invalid bit size: 1023, needs to be in range of 128 to 8192 and power of 2.");
    }

    #[test]
    fn test_init_invalid_n_threads() {
        let args = vec!["-t".to_string(), "5".to_string(), "bla".to_string()];
        let config = BenchmarkConfig::init(&args);
        assert!(config.is_err());
        assert_eq!(config.unwrap_err().msg, "Unable to parse number of threads: bla" )
    }
}