use rug::Integer;
use std::env;
use rsa_arbitray_precision::benchmark as bm;
use num_cpus;

// TODO add command line options
fn main() {
    // let msg = Integer::from(10850);
    // let (d, n, e) = rsa_module::generate_key_pair(4096, 2);
    // // let c = rsa_module::encrypt_msg(&msg, &e, &n);
    // // let decyphered = rsa_module::decrypt_cypher(&c, &d, &n);
    // // println!("  msg: {}\n cyph: {}\ndecyp: {}", msg, c.to_string_radix(16), decyphered);
    // bm::benchmark_threads(20, 4, 3);
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

    println!("{:x}", 255);
}


// add flag to specify number of threads and bit sizes
fn do_benchmark(args: &[String]) {

}

// add flag to specify length, and -f if to save to file
fn do_generate(args: &[String]) {

}

// add -f flag for file option
fn do_encrypt(args: &[String]) {

}

// add -f flag for store as file option
fn do_decrypt(args: &[String]) {

}

fn print_help() {

}

fn print_usage() {

}