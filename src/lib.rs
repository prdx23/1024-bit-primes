
mod rngs;
mod algos;

use rngs::random_u16;
use algos::trial_division_simple;
use algos::PrimeResult;


pub fn run() {
    loop {
        let num: usize = random_u16().unwrap().into();
        if trial_division_simple(num) == PrimeResult::Prime {
            println!("Prime found:");
            println!("{} {:b}", num, num);
            break;
        }
    }
}
