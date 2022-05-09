use rug::Integer;
use rug::ops::Pow;
use rug::rand::RandState;
use std::sync::mpsc;
use rand;
use std::thread;
use crate::helpers::{gcd, find_inverse, pow_mod};
use crate::prime_module::is_prime;

struct SendInteger {
    n: Integer,
}

unsafe impl Sync for SendInteger {}
unsafe impl Send for SendInteger {}


pub fn generate_p_q(bits: u32, n_threads: u8) -> (Integer, Integer) {

    assert!(n_threads >= 2);

    let (tx, rx) = mpsc::sync_channel(n_threads as usize);
    for _ in 0..n_threads {
        let t = tx.clone();
        thread::spawn(move || {
            let mut rng = RandState::new();
            let seed = rand::random::<i16>();
            rng.seed(&Integer::from(seed));

            let lower_bound = Integer::from(Integer::from(2_i16).pow(bits - 1));

            loop {
                let candidate = Integer::from(Integer::random_bits(bits - 1, &mut rng)) + &lower_bound;
                if is_prime(&candidate) {
                    let _ = t.send(SendInteger { n: candidate });
                    break;
                }      
            }
        });
    }

    let mut count = 0;
    let (mut p, mut q) = (Integer::from(0_i16), Integer::from(0_i16));
    for received in rx {
        if count == 0 {
            p = Integer::from(received.n);
        } else if count == 1 {
            q = Integer::from(received.n);
        }

        count += 1;

        if count == 2 { 
            return (p, q) };
    }

    // println!("{}, {}", p, q);
    (p, q)
}

pub fn calculate_n_phi(p: &Integer, q: &Integer) -> Integer {
    Integer::from(Integer::from(p - 1) * Integer::from(q - 1))
}

pub fn generate_e(n_phi: &Integer) -> Integer {
    let mut rng = RandState::new();
    let mut e;
    loop {
        e = Integer::random_below(Integer::from(n_phi), &mut rng); 
        if gcd(n_phi, &e) == 1 {
            return e;
        }
    }
}

pub fn generate_d(e: &Integer, n_phi: &Integer) -> Integer {
    find_inverse(e, n_phi)
}

pub fn generate_key_pair(bits: u32, n_threads: u8) -> (Integer, Integer, Integer)  {
    let (p, q) = generate_p_q(bits, n_threads);
    let n = Integer::from(&p * &q);
    let n_phi = calculate_n_phi(&p, &q);

    let e = generate_e(&n_phi);
    let d = generate_d(&e, &n_phi);
    (d, n, e)
}

pub fn encrypt_msg(msg: &Integer, e: &Integer, n: &Integer) -> Integer {
    pow_mod(msg, e, n)
}

pub fn decrypt_cypher(c: &Integer, d: &Integer, n: &Integer) -> Integer {
    pow_mod(c, d, n)
}

#[test]
fn test_generate_p_q_threads() {
    generate_p_q(4096, 8);
}

