
use std::env;
use rsa_arbitray_precision::commands::*;


// TODO maybe implement factory pattern for config 
// and them handle running the program
fn main() {
    // let msg = Integer::from(10850);
    // let (d, n, e) = rsa_module::generate_key_pair(4096, 2);
    // // let c = rsa_module::encrypt_msg(&msg, &e, &n);
    // // let decyphered = rsa_module::decrypt_cypher(&c, &d, &n);
    // // println!("  msg: {}\n cyph: {}\ndecyp: {}", msg, c.to_string_radix(16), decyphered);
    // bm::benchmark_threads(10, 4, 3);
    let args: Vec<String> = env::args().collect();
    if let Some(cmd) = args.get(1) {
        match cmd.as_str() {
            "benchmark" => do_benchmark(&args[2..]),
            "generate" => do_generate(&args[2..]),
            "encrypt" => do_encrypt(&args[2..]),
            "decrypt" => do_decrypt(&args[2..]),
            "help" => print_help(),
            _ => print_usage(),
        };
    }
}

// benchmark commands:
// benchmark [OPTIONS]
// if no flags are specified, it will go from 1k to 4k
// with n_thread = num_cpus 
// flags:
// -b:
// bitsizes can be specified with -b [1024,2048,...]
// bitsizes can be entered as a comma separated list of numbers
// -t:
// num of threads with -t [t1,t2,t3,...,tn]
// threads can be entered as a comma separated list of numbers
// add flag to specify number of threads and bit sizes
// -f [file_name]
// save results to a file 
// if [file_name] is empty, a default name with the date and time is created
// -h
// show help for this command
fn do_benchmark(args: &[String]) {
    let config = benchmark::BenchmarkConfig::new(args);
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
fn do_generate(args: &[String]) {
    let config = generate::GenerateConfig::new(args);
}

// encrypt a message 
// encrypt [OPTIONS] key_file message
// 
// flags:
// -f [file_name] 
// specify if message should be saved to file
// -s
// use private key to encrypt
// -F
// message comes from file, otherwise will be string
// -h
// show help for this command
fn do_encrypt(args: &[String]) {
    let config = encrypt::EncryptConfig::new(args);
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
fn do_decrypt(args: &[String]) {

}

// Print possible commands
fn print_help() {

}

// Print short string to show benchmark, help etc. commands
fn print_usage() {

}