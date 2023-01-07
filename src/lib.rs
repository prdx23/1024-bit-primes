mod rng;
mod algos;
mod utils;
mod bigint;

use algos::PrimeResult;
use bigint::BigInt;


fn primes_16bit() -> u16 {
    loop {
        let num = rng::u16() | 0b1000000000000001;
        if algos::trial_division_simple(num) == PrimeResult::Prime {
            return num;
        }
    }
}



fn primes_64bit() -> u64 {
    const N: usize = 10000;
    let start = (N + 1) as u64;
    let primes = utils::generate_small_primes::<N>();

    loop {
        let num = rng::u64() | 0x8000000000000001u64;
        let mut result = PrimeResult::Unknown;

        for i in 0..N {
            if num % primes[i] == 0 {
                result = PrimeResult::Composite;
                break;
            }
        }

        if result == PrimeResult::Unknown {
            result = algos::trial_division(num, start)
        }

        if result == PrimeResult::Prime {
            return num;
        }
    }
}



fn primes_128bit() -> u128 {
    loop {
        let num = (rng::u64() | 0x8000000000000001u64) as u128;
        // if algos::fermat_test_u128(num, 1) == PrimeResult::ProbablePrime {
        //     return num;
        // }
        if algos::miller_rabin_test_u128(num, 1) == PrimeResult::ProbablePrime {
            return num;
        }
    }

}


fn primes_1024bit() -> BigInt {
    const P: usize = 5000;
    let primes = utils::generate_small_primes::<P>();

    let zero = BigInt::zero();
    let mut small_prime = BigInt::zero();

    let mut num = BigInt::random();
    num.modify();

    'prime_loop: loop {
        num = num.increase_by_2();

        for i in 0..P {
            small_prime.chunks[0] = primes[i];
            if num % small_prime == zero {
                continue 'prime_loop;
            }
        }

        if algos::miller_rabin_test(num, 10) == PrimeResult::ProbablePrime {
            return num;
        }
    }
}


pub fn run() {
    // println!("Prime found: {}", primes_16bit());
    // println!("Prime found: {}", primes_64bit());
    // println!("Prime found: {}", primes_128bit());

    let (tx, rx) = std::sync::mpsc::channel();

    for _ in 0..16 {
        let thread_tx = tx.clone();
        std::thread::spawn(move || {
            thread_tx.send(primes_1024bit()).unwrap();
        });
    }

    let prime = rx.recv().unwrap();
    prime.print_decimal();
    return
}
