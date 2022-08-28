
mod rngs;
mod algos;
mod utils;

use rngs::random_u16;
use rngs::random_u64;

use algos::trial_division_simple;
use algos::trial_division;
use algos::PrimeResult;

use utils::generate_small_primes;


fn primes_16bit() {
    loop {
        let num = random_u16().unwrap();
        if trial_division_simple(num.into()) == PrimeResult::Prime {
            println!("Prime found: {}", num);
            break;
        }
    }
}



fn primes_64bit() {
    const N: usize = 10000;
    let primes = generate_small_primes::<N>();

    loop {
        let num = random_u64().unwrap() as usize;
        let mut result = PrimeResult::Unknown;

        for i in 0..N {
            if num % primes[i] == 0 {
                result = PrimeResult::Composite;
                break;
            }
        }

        if result == PrimeResult::Unknown {
            result = trial_division(num, N + 1)
        }

        if result == PrimeResult::Prime {
            println!("Prime found: {}", num);
            break;
        }
    }
}


pub fn run() {
    // primes_16bit();
    primes_64bit();
}
