use crate::rng;
use crate::utils;



#[derive(PartialEq)]
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



pub fn fermat_test(num: u128, k: usize) -> PrimeResult {
    let mut result = PrimeResult::ProbablePrime;
    for _ in 0..k {
        let base = rng::u128_range(2, num - 1);
        if utils::mod_exp(base, num - 1, num) != 1 {
            result = PrimeResult::Composite;
            break;
        }
    }
    result
}



pub fn miller_rabin_test(n: u128, k: usize) -> PrimeResult {

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
