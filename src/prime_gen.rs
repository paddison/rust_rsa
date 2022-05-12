use rug::{Integer, ops::Pow, rand::* };
use crate::helpers::pow_mod;

use self::sieve_of_eratosthenes::Sieve;

pub fn is_prime(n: &Integer, sieve: &Sieve) -> bool {

    if !n.get_bit(0) || !sieve.is_prime_candidate(&n) { return false };

    let (s, d) = get_factors(n);

    for _ in 0..23 {
        if !rabin_miller_test(n, &d, &s) {
            return false;
        }
    }
    true
}

// rewrite n as 2^s * d + 1 where d is an odd number
fn get_factors(n: &Integer) -> (Integer, Integer) {
    let mut d = Integer::from(n - 1);
    let mut s: u32 = 0;

    while Integer::from(&d % &2) == 0 {
        s += 1;
        d >>= 1;
    }
    let two = Integer::from(2);
    assert_eq!(*n, Integer::from(&two.pow(s) * &d) + 1);
    (Integer::from(s), d)
}

// if one of these relations hold the number is a strong probable prime
fn rabin_miller_test(n: &Integer, d: &Integer, s: &Integer) -> bool {

    let mut rng = RandState::new();

    // create a random number a and test if n is a strong probable prime to base a
    let a = &Integer::from(n).random_below(&mut rng);

    // first condition: a^d mod n == 1 mod n
    if pow_mod(a, d, n) == 1 {
        return true;
    }

    let mut r = 0;

    while &r < s  {
    // second condition: a^(2^r * d) mod n == -1 mod n
        let two = Integer::from(2);
        let exp = &Integer::from(two.pow(r) * d);
        if pow_mod(a, exp, n) == Integer::from(n - &1) {
            return true;
        }
        r += 1;
    }

    false
}

pub mod sieve_of_eratosthenes {
    use rug::Integer;

    pub struct Sieve {
        primes: Vec<u32>,
    }

    impl Sieve {

        pub fn new(size: u32) -> Self {
            Sieve { primes: get_primes(size) }
        }

        pub fn is_prime_candidate(&self, n: &Integer) -> bool {
            for prime in self.primes.iter() {
                if Integer::from(n % prime) == 0 && n != prime {
                    return false;
                }
            }
            true
        }
    }

    fn get_primes(n: u32) -> Vec<u32> {
        get_numbers_from_sieve(create_sieve(n))
    }
    
    fn create_sieve(n: u32) -> Vec<bool> {
        let sqrt_n = (n as f64).sqrt() as usize;
        let mut sieve: Vec<bool> = vec![true; (n - 1) as usize];
    
        // loop up until the square root of n
        for i in 2..=sqrt_n {
            // determine if the current number is a prime
            if sieve[i - 2] { 
                // we know that starting from the square all multiples of this number aren't primes, so we filter them
                let no_prime = i * i;
                // scalar value to get multiples of our current number
                let mut a = 0;
                
                // set at the correct index (we start at 2 so we need to subtract 2, to index correctly)
                while let Some(n) = sieve.get_mut(no_prime + i * a - 2) {
                    *n = false;
                    a += 1;
                }
            } 
        }
        
        sieve
    }

    #[test]
    fn test_create_sieve_10() {
        let sieve = create_sieve(20);
        assert_eq!(vec![true, true, false, true, false, 
                        true, false, false, false, true,
                        false, true, false, false, false,
                        true, false, true, false], 
                    sieve);
    }
    
    fn get_numbers_from_sieve(sieve: Vec<bool>) -> Vec<u32> {
        let mut primes: Vec<u32> = Vec::new();
        // add all the prime numbers (add + 2 because we start from 2)
        for (n, is_prime) in sieve.into_iter().enumerate() {
            if is_prime {
                primes.push((n + 2) as u32);
            } 
        }
        primes
    }
}