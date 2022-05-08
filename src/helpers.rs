use rug::Integer;

pub fn pow_mod(base: &Integer, exp: &Integer, modulo: &Integer) -> Integer {
    if &0 == exp {
        return Integer::from(1);
    }

    let mut cur_base = Integer::from(base % modulo);
    let mut result = Integer::from(1);
    let mut exponent = Integer::from(exp);

    while &0 < &exponent {
        if Integer::from(&exponent % &2) == 1 {
            result = Integer::from(Integer::from(result * &cur_base) % modulo);
        }
        exponent >>= 1;
        cur_base = Integer::from(Integer::from(&cur_base * &cur_base) % modulo);     
    }

    result
}

pub fn gcd(a: &Integer, b: &Integer) -> Integer {

    let mut a = Integer::from(a);
    let mut b = Integer::from(b);

    let mut tmp;

    loop {
        tmp = Integer::from(&b);
        b = Integer::from(&a % &b);
        a = Integer::from(tmp);
        if &b == &0 {
            return Integer::from(a);
        }
    }
}

pub fn find_inverse(e: &Integer, n_phi: &Integer) -> Integer {
    let mut old_r = Integer::from(n_phi);
    let mut r = Integer::from(e);
    let mut old_s = Integer::from(0);
    let mut s = Integer::from(1);

    let mut q;

    let mut r_tmp: Integer;
    let mut s_tmp: Integer;

    while &r != &0 {
        q = Integer::from(&old_r / &r);
        r_tmp = Integer::from(&old_r - Integer::from(&q * &r));
        old_r = Integer::from(&r);
        r = Integer::from(&r_tmp);

        s_tmp = Integer::from(&old_s - Integer::from(&q * &s));
        old_s = Integer::from(&s);
        s = Integer::from(&s_tmp);

        
    }
    if &old_s < &0 {
        return Integer::from(&old_s + n_phi);
    }
    old_s
}

#[cfg(test)]
pub mod test {

    use super::*;
    use rug::Integer;

    #[test]
    pub fn gcd_16_4() {
        let result = gcd(&Integer::from(16), &Integer::from(4));
        assert_eq!(4, result);
    }

    #[test]
    pub fn find_inverse_7_23400() {
        let e = Integer::from(7);
        let n_phi = Integer::from(23400);
        let result = find_inverse(&e, &n_phi);
        assert_eq!(3343, result);
    }
}


