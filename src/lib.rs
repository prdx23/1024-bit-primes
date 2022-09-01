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
        // let num = rng::u128() | 0x80000000000000000000000000000001u128;
        let num = (rng::u64() | 0x8000000000000001u64) as u128;
        if algos::miller_rabin_test(num, 30) == PrimeResult::ProbablePrime {
            return num;
        }
    }
}



pub fn run() {
    // println!("Prime found: {}", primes_16bit());
    // println!("Prime found: {}", primes_64bit());
    // println!("Prime found: {}", primes_128bit());
    // primes_128bit()


    // println!("{:?}", BigInt::from(1234u128).digits);
    // println!("{:?}", BigInt::from(random_u128(false).unwrap()).digits);

    // let num = rng::u128() | 0x80000000000000000000000000000001u128;
    // println!("BigInt: {}", BigInt::from(num));
    // println!("u128:   {}", num);

    let num1 = rng::u64() as u128;
    let num2 = rng::u64() as u128;
    // let num1 = 123u128;
    // let num2 = 456u128;
    let expected = num1 * num2;
    let test = BigInt::from(num1) * BigInt::from(num2);

    // let mut b1 = BigInt::from(num1);
    // b1 += BigInt::from(num2);

    println!();
    println!("{} {}", num1, num2);
    println!("{}", expected);
    println!("{}", test);
    // println!("{}", b1);

    // let largest1 = BigInt { digits: [255u8; 78] };
    // let largest2 = BigInt { digits: [99u8; 78] };
    // let test = largest1 + largest2;
}
