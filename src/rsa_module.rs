use rug::Integer;
use rug::ops::Pow;
use rug::rand::RandState;
use crate::helpers::{gcd, find_inverse, pow_mod};
use crate::prime_module::is_prime;

pub fn generate_p_q(bits: u32) -> (Integer, Integer) {
    let mut rng = RandState::new();
    let lower_bound = &Integer::from(Integer::from(2).pow(bits - 1));

    let mut p;
    loop {
        p = Integer::from(Integer::random_bits(bits - 1, &mut rng)) + lower_bound;
        if is_prime(&p) {
            break;
        }       
    }

    let mut q;
    loop {
        q = Integer::from(Integer::random_bits(bits - 1, &mut rng)) + lower_bound;
        if q.is_odd() && p != q {
            if is_prime(&q) {
                break;
            }
        }
    }
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

pub fn generate_key_pair(bits: u32) -> (Integer, Integer, Integer)  {
    let (p, q) = generate_p_q(bits);
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



