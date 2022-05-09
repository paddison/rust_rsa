pub mod helpers;
pub mod prime_module;
pub mod rsa_module;
pub mod input_module;

#[cfg(test)]
pub mod tests {
    pub mod helpers_tests {
        use rug::Integer;
        use crate::helpers::*;

        #[test]
        fn test_4_13_497() {
            let result = pow_mod(&Integer::from(u128::MAX), &Integer::from(u128::MAX), &Integer::from(19));
            println!("{}", result);
            assert_ne!(445, result);
        } 
    }

    pub mod rsa_module_tests {
        use rug::Integer;
        use crate::rsa_module::generate_e;
        use crate::helpers::gcd;

        #[test]
        fn test_23400() {
            let n_phi = Integer::from(7);
            let result = generate_e(&n_phi);
            println!("{}", result);
            println!("{}", gcd(&n_phi, &result));
        }
    }

}