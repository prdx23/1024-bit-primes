use crate::rng;
use crate::utils;
use crate::BigInt;



#[derive(PartialEq, Debug)]
pub enum PrimeResult {
    Prime,
    Composite,
    Unknown,
    ProbablePrime,
}



pub fn trial_division_simple(n: u16) -> PrimeResult {
    let root_n = (n as f64).sqrt() as u16;
    for x in 3..root_n {
        if n % x == 0 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::Prime
}



pub fn trial_division(n: u64, start: u64) -> PrimeResult {
    // assumption: n is odd, n > 3 and start > 3

    if n % 3 == 0 {
        return PrimeResult::Composite;
    }

    let root_n = (n as f64).sqrt() as u64;
    for x in (start..(root_n + 1)).step_by(6) {
        if n % x == 0 || n % (x + 2) == 0 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::Prime
}



pub fn fermat_test_u128(num: u128, k: usize) -> PrimeResult {
    for _ in 0..k {
        let base = rng::u128_range(2, num - 1);
        if utils::mod_exp(base, num - 1, num) != 1 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::ProbablePrime
}



pub fn miller_rabin_test_u128(n: u128, k: usize) -> PrimeResult {

    let mut s = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        d = d / 2;
        s += 1;
    }

    'main_loop: for _ in 0..k {
        let base = rng::u128_range(2, n - 2);

        let mut x = utils::mod_exp(base, d, n);
        if x == 1 || x == n - 1 { continue 'main_loop; }

        for _ in 0..(s - 1) {
            x = utils::mod_exp(x, 2, n);
            if x == n - 1 { continue 'main_loop; }
        }

        return PrimeResult::Composite;
    }

    PrimeResult::ProbablePrime
}


pub fn miller_rabin_test(n: BigInt, k: usize) -> PrimeResult {

    let zero = BigInt::zero();
    let one = BigInt::from(1);

    let mut s = zero;
    let n_minus_1 = n.decrease();

    let mut d = n_minus_1;
    while d.is_even() {
        d >>= 1;
        s = s.increase();
    }

    let mut bytes = [0; (1024 / 16)];
    let mut x;
    let mut base;

    'main_loop: for _ in 0..k {

        rng::insert_random_bytes(&mut bytes).unwrap();
        base = BigInt::from(bytes.as_slice());

        x = one;
        while !d.is_zero() {
            if !d.is_even() {
                x = (x * base) % n;
            }
            d = d >> 1;
            base = (base * base) % n;
        }

        if x == one || x == n_minus_1 { continue 'main_loop; }

        while !s.is_zero() {
            x = (x * x) % n;
            if x == n_minus_1 { continue 'main_loop; }
            s = s.decrease();
        }

        return PrimeResult::Composite;
    }

    PrimeResult::ProbablePrime
}
