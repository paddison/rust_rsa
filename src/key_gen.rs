use rug::Integer;
use rug::ops::Pow;
use rug::rand::RandState;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use rand;
use std::{thread, time};
use crate::helpers::{gcd, find_inverse, pow_mod};
use crate::prime_gen::is_prime;
use crate::prime_gen::sieve_of_eratosthenes::Sieve;


trait RsaKey {
    fn from_file(file_name: String) -> Self;
    fn write_to_file(&self, file_name: String) -> std::io::Result<()> {
        let s = self.serialize();
        let mut file = File::create(file_name)?;
        file.write_all(&s.as_bytes())?;
        Ok(())
    }
    fn get_parts(&self) -> (&Integer, &Integer);
    fn serialize(&self) -> String {
        let (part_0, part_1) = self.get_parts();
        let mut s0 = Self::into_hex(part_0);
        let s1 = Self::into_hex(part_1);
       
        s0 += "\n=======\n";
        s0 += s1.as_ref();
        s0
    }
    fn into_hex(n: &Integer ) -> String {
        let n_ptr = n.as_raw();
        let mut raw_string = String::new();
    
        // SAFETY: Accessing the pointer is safe, since n will be a valid integer,
        // and the pointer only accesses memory in mpz.size, which must be valid
        unsafe {
            let mpz = *n_ptr;
            for i in 0..mpz.size {
                let part = *mpz.d.as_ptr().add(i as usize);
                for byte in part.to_be_bytes(){
                    raw_string += &format!("{:x}", byte);
                }
            }
        }

        raw_string
    }
}

pub struct RsaPrivateKey {
    d: Integer,
    n: Integer,
}

pub struct RsaPublicKey {
    e: Integer,
    n: Integer,
}

impl RsaKey for RsaPrivateKey {

    fn from_file(file_name: String) -> Self {
        RsaPrivateKey { d: Integer::from(1), n: Integer::from(2) }
    }

    #[inline(always)]
    fn get_parts(&self) -> (&Integer, &Integer) {
        (&self.d, &self.n)
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

pub fn generate_p_q(bits: u32, n_threads: u8) -> (Integer, Integer) {

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

pub fn generate_key_pair(bits: u32, n_threads: u8) -> (RsaPrivateKey, RsaPublicKey)  {
    let (p, q) = generate_p_q(bits, n_threads);
    let n = Integer::from(&p * &q);
    let n_phi = calculate_n_phi(&p, &q);
    let e = generate_e(&n_phi);
    let d = generate_d(&e, &n_phi);
    (RsaPrivateKey { d, n: Integer::from(&n) }, RsaPublicKey { e, n })
}

pub fn encrypt_msg(msg: &Integer, RsaPublicKey { e, n }: RsaPublicKey) -> Integer {
    pow_mod(msg, &e, &n)
}

pub fn decrypt_cypher(c: &Integer, RsaPrivateKey { d, n }: RsaPrivateKey) -> Integer {
    pow_mod(c, &d, &n)
}

#[test]
fn test_generate_p_q_threads() {
    for _ in 0..10 {
        let start = time::Instant::now();
        let (_, _) = generate_p_q(4096, 6);
        println!("Created 4k bit key pair in {}, with 6 threads", start.elapsed().as_millis());
    }
    for _ in 0..10 {
        let start = time::Instant::now();
        let (_, _) = generate_p_q(4096, 8);
        println!("Created 4k bit key pair in {}, with 8 threads", start.elapsed().as_millis());
    }
}