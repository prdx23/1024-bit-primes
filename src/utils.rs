
use crate::PrimeResult;
use crate::algos::trial_division;

pub fn generate_small_primes<const N: usize>() -> [u64; N] {

    let mut primes: [u64; N] = [0; N];
    primes[0] = 2;
    primes[1] = 3;
    let mut n: u64 = 3;
    let mut nth: u64 = 2;
    let mut i: usize = 2;
    let limit = N as u64;

    loop {
        n += 2;
        if trial_division(n, 5) == PrimeResult::Prime {
            primes[i] = n;
            i += 1;
            nth += 1;
            if nth == limit {
                return primes
            }
        }
    }
}


pub fn mod_exp(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0 }

    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    result
}
// pub fn mod_exp(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
//     if modulus == 1 { return 0 }

//     let mut result = 1;

//     for _ in 0..(exponent - 1) {
//         result = ((result % modulus) * (base % modulus)) % modulus;
//     }

//     result
// }
