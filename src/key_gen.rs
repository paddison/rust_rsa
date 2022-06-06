use rug::{Integer, Complete};
use rug::integer::ParseIntegerError;
use rug::ops::Pow;
use rug::rand::RandState;
use std::fs::File;
use std::io::{Write, Read, ErrorKind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use rand;
use std::{thread};
use crate::helpers::{gcd, find_inverse, pow_mod};
use crate::prime_gen::is_prime;
use crate::prime_gen::sieve_of_eratosthenes::Sieve;

const SEPARATOR: &str = "\n=======\n";

pub trait RsaKey {
    fn from_file(file_name: &str) -> std::io::Result<Self> where Self: Sized {
        let mut buffer = String::new();
        let _ = File::open(file_name)?.read_to_string(&mut buffer)?;
        let key = Self::deserialize(buffer);
        match key {
            Ok(k) => Ok(k),
            Err(e) => Err(std::io::Error::new(ErrorKind::Other, e.to_string())),
        }
    }
    fn get_parts(&self) -> Vec<&Integer>;
    fn deserialize(key: String) -> Result<Self, ParseIntegerError> where Self: Sized;
    fn write_to_file(&self, file_name: String) -> std::io::Result<()> {
        let s = self.serialize();
        let mut file = File::create(file_name)?;
        file.write_all(&s.as_bytes())?;
        Ok(())
    }
    fn serialize(&self) -> String;
    fn into_hex(n: &Integer ) -> String {
        let n_ptr = n.as_raw();
        let mut raw_string = String::new();
    
        // SAFETY: Accessing the pointer is safe, since n will be a valid integer,
        // and the pointer only accesses memory in mpz.size, which must be valid
        unsafe {
            let mpz = *n_ptr;
            // we need to go backwarts, since the limbs(64 bit) of the number are stored in reverse order apparently
            for i in (0..mpz.size).rev() {
                let part = *mpz.d.as_ptr().add(i as usize);
                for byte in part.to_be_bytes(){
                    raw_string += &format!("{:0>2x}", byte);
                }
            }
        }

        raw_string
    }
}

#[derive(Debug)]
pub struct RsaPrivateKey {
    d: Integer,
    n: Integer,
    public_key_part: Integer,
}

impl RsaPrivateKey {
    pub fn new(bits: u32) -> Self {
        let cores = num_cpus::get_physical();
        let (p, q) = generate_p_q(bits, cores);
        let n_phi = calculate_n_phi(&p, &q);
        let e = generate_e(&n_phi);
        let d = generate_d(&e, &n_phi);
        let n = Integer::from(&p * &q);
        RsaPrivateKey { d, n , public_key_part: e }
    }
}

impl RsaKey for RsaPrivateKey {

    #[inline(always)]
    fn get_parts(&self) -> Vec<&Integer> {
        let mut parts = vec![];
        parts.push(&self.d);
        parts.push(&self.n);
        parts.push(&self.public_key_part);
        parts
    }

    fn deserialize(key: String) -> Result<Self, ParseIntegerError> {
        let parts: Vec<&str> = key.split(SEPARATOR).collect();
        let d = Integer::parse_radix(parts[0], 16)?.complete();
        let n = Integer::parse_radix(parts[1], 16)?.complete();
        let public_key_part = Integer::parse_radix(parts[2], 16)?.complete();
        Ok(RsaPrivateKey { d, n, public_key_part })
    }

    fn serialize(&self) -> String {
        let parts = self.get_parts();
        assert_eq!(parts.len(), 3);
        let mut buffer = String::new();

        buffer += &RsaPrivateKey::into_hex(parts[0]); 
        buffer += SEPARATOR;
        buffer += &RsaPrivateKey::into_hex(parts[1]); 
        buffer += SEPARATOR;
        buffer += &RsaPrivateKey::into_hex(parts[2]);

        buffer
    }
}

pub struct RsaPublicKey {
    e: Integer,
    n: Integer,
}

impl RsaKey for RsaPublicKey {

    fn get_parts(&self) -> Vec<&Integer> {
        let mut parts = vec![];
        parts.push(&self.e);
        parts.push(&self.n);

        parts
    }

    fn deserialize(key: String) -> Result<Self, ParseIntegerError> where Self: Sized {
        let parts: Vec<&str> = key.split(SEPARATOR).collect();
        let e = Integer::from_str_radix(parts[0], 16)?;
        let n = Integer::from_str_radix(parts[1], 16)?;

        Ok(RsaPublicKey { e, n })
    }

    fn serialize(&self) -> String {
        let parts = self.get_parts();
        assert_eq!(parts.len(), 2);
        let mut buffer = String::new();
        buffer += &RsaPublicKey::into_hex(parts[0]);
        buffer += SEPARATOR;
        buffer += &RsaPublicKey::into_hex(parts[1]);

        buffer
    }
}

/// Wrapper for Integer, to share it between threads.
/// These will always be immutable, so it is safe to share them
struct SendInteger {
    n: Integer,
}

unsafe impl Sync for SendInteger {}
unsafe impl Send for SendInteger {}

// TODO: Create struct for public and private keys, which can be serialized into a file

pub fn generate_p_q(bits: u32, n_threads: usize) -> (Integer, Integer) {

    assert!(n_threads >= 2);
    let found_primes = Arc::new(AtomicBool::new(false));  
    let sieve = Arc::new(Sieve::new(10000));
    let (tx, rx) = mpsc::sync_channel(n_threads as usize);
    
    for _ in 0..n_threads {
        let t = tx.clone();
        let sieve = Arc::clone(&sieve);
        let found = Arc::clone(&found_primes);

        thread::spawn(move || {
            let mut rng = RandState::new();
            let lower_bound = Integer::from(Integer::from(2_i16).pow(bits - 1));
            let seed = rand::random::<i16>();
            rng.seed(&Integer::from(seed));

            while !found.load(Ordering::Relaxed) {
                let candidate = Integer::from(Integer::random_bits(bits - 1, &mut rng)) + &lower_bound;
                if is_prime(&candidate, &sieve) {
                    let _ = t.send(SendInteger { n: candidate });
                }      
            }
        });
    }
    let p = Integer::from(rx.recv().unwrap().n);
    let q = Integer::from(rx.recv().unwrap().n);
    found_primes.swap(true, Ordering::SeqCst); // signal other threads to stop searching
    (p, q)
}

#[inline(always)]
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

pub fn generate_key_pair(bits: u32, n_threads: usize) -> (RsaPrivateKey, RsaPublicKey)  {
    let (p, q) = generate_p_q(bits, n_threads);
    let n = Integer::from(&p * &q);
    let n_phi = calculate_n_phi(&p, &q);
    let e = generate_e(&n_phi);
    let d = generate_d(&e, &n_phi);
    (RsaPrivateKey { d, n: Integer::from(&n), public_key_part: Integer::from(&e) }, RsaPublicKey { e, n })
}

pub fn encrypt_msg(msg: &Integer, RsaPublicKey { e, n, ..}: RsaPublicKey) -> Integer {
    pow_mod(msg, &e, &n)
}

pub fn decrypt_cypher(c: &Integer, RsaPrivateKey { d, n, .. }: RsaPrivateKey) -> Integer {
    pow_mod(c, &d, &n)
}

#[test]
fn test_generate_p_q_threads() {
    use std::time;

    for _ in 0..10 {
        let start = time::Instant::now();
        let (_, _) = generate_p_q(2048, 6);
        println!("Created 4k bit key pair in {}, with 6 threads", start.elapsed().as_millis());
    }
    for _ in 0..10 {
        let start = time::Instant::now();
        let (_, _) = generate_p_q(2048, 8);
        println!("Created 4k bit key pair in {}, with 8 threads", start.elapsed().as_millis());
    
    }
}

#[test]
fn test_serialize() {
    let key = RsaPrivateKey::new(2048);
    let serialized_key = key.serialize();
    let deserialized_key = RsaPrivateKey::deserialize(serialized_key);
    assert!(deserialized_key.is_ok());
    let deserialized_key =  deserialized_key.unwrap();
    assert_eq!(key.d, deserialized_key.d);
    assert_eq!(key.n, deserialized_key.n);
}

#[test]
fn test_bit() {
    let n = Integer::from(0b0011);
    assert!(n.get_bit(0));
    assert!(!n.get_bit(3));
}

#[test]
fn generate_key_pair_and_serialize() {
    let (_, pk) = generate_key_pair(1024, num_cpus::get_physical());
    pk.serialize();
}