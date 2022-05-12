use std::{collections::HashMap, time};

use crate::key_gen;

pub fn benchmark_threads(repeats: u16, thread_range: u8, bit_range: u8) {
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
                let n_threads = 2_usize.pow(j as u32);
                let entry = map.entry(n_threads).or_insert(0);
                *entry += benchmark_generate_key_pair(bits, n_threads);
            }     
        }
    }

    print_results(maps, repeats, thread_range);
}

fn benchmark_generate_key_pair(bits: u32, n_threads: usize) -> u128 {
    let start = time::Instant::now();
    let (_, _) = key_gen::generate_key_pair(bits, n_threads);
    println!("Created {} bit key pair in {}, with {} threads", bits, start.elapsed().as_millis(), n_threads);
    start.elapsed().as_millis()
}

fn print_results(maps: Vec<HashMap<usize, u128>>, repeats: u16, thread_range: u8) {
    println!("===Results===");
    for (i,map) in maps.into_iter().enumerate() {
        println!("");
        let bits = 1024 * 2_u32.pow(i as u32);
        println!("{} bits:", bits);
        for j in 1..(thread_range + 1) {
            let n_threads = 2_usize.pow(j as u32);
            println!("\t{} Threads: {}", n_threads, map.get(&n_threads).unwrap() / repeats as u128);
        } 
    }
}
