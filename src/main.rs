use rsa_arbitray_precision::rsa_module;
use rug::Integer;
use std::env;
use std::{time, collections::HashMap};

// TODO add command line options
fn main() {
    // let msg = Integer::from(10850);
    // let (d, n, e) = rsa_module::generate_key_pair(4096, 2);
    // // let c = rsa_module::encrypt_msg(&msg, &e, &n);
    // // let decyphered = rsa_module::decrypt_cypher(&c, &d, &n);
    // // println!("  msg: {}\n cyph: {}\ndecyp: {}", msg, c.to_string_radix(16), decyphered);
    benchmark_threads(20, 4, 3);
    let _args: Vec<String> = env::args().collect();
}

fn benchmark_threads(repeats: u16, thread_range: u8, bit_range: u8) {
    assert!(bit_range > 0);
    assert!(thread_range > 0);
    let mut maps = vec![];
    for _ in 0..bit_range {
        maps.push(HashMap::new())
    }

    for i in 0..repeats {
        println!("{} repeats left", repeats - i);
        for (i,map) in maps.iter_mut().enumerate() {
            let bits = 1024 * 2_u32.pow(i as u32);
            for j in 1..(thread_range + 1) {
                let n_threads = 2_u8.pow(j as u32);
                let entry = map.entry(n_threads).or_insert(0);
                *entry += benchmark_generate_key_pair(bits, n_threads);
            }
            
        }
    }

    print_results(maps, repeats, thread_range);
}

fn benchmark_generate_key_pair(bits: u32, n_threads: u8) -> u128 {
    let start = time::Instant::now();
    let (_, _, _) = rsa_module::generate_key_pair(bits, n_threads);
    println!("Created {} bit key pair in {}, with {} threads", bits, start.elapsed().as_millis(), n_threads);
    start.elapsed().as_millis()
}

fn print_results(maps: Vec<HashMap<u8, u128>>, repeats: u16, thread_range: u8) {
    println!("===Results===");
    for (i,map) in maps.into_iter().enumerate() {
        println!("");
        let bits = 1024 * 2_u32.pow(i as u32);
        println!("{} bits:", bits);
        for j in 1..(thread_range + 1) {
            let n_threads = 2_u8.pow(j as u32);
            println!("\t{} Threads: {}", n_threads, map.get(&n_threads).unwrap() / repeats as u128);
        } 
    }
}
