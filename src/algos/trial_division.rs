use crate::PrimeResult;


pub fn trial_division_simple(n: usize) -> PrimeResult {
    let root_n = (n as f64).sqrt() as usize;
    for x in 2..root_n {
        if n % x == 0 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::Prime
}
