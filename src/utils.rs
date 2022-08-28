
use crate::PrimeResult;
use crate::algos::trial_division;

pub fn generate_small_primes<const N: usize>() -> [usize; N] {

    let mut primes: [usize; N] = [0; N];
    primes[0] = 2;
    primes[1] = 3;
    let mut n: usize = 3;
    let mut nth: usize = 2;
    let mut i: usize = 2;

    loop {
        n += 2;
        if trial_division(n, 5) == PrimeResult::Prime {
            primes[i] = n;
            i += 1;
            nth += 1;
            if nth == N {
                return primes
            }
        }
    }
}
