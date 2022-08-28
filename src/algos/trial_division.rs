use crate::PrimeResult;


pub fn trial_division_simple(n: u128) -> PrimeResult {
    let root_n = (n as f64).sqrt() as u128;
    for x in 3..root_n {
        if n % x == 0 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::Prime
}


pub fn trial_division(n: usize, start: usize) -> PrimeResult {
    if n % 3 == 0 {
        return PrimeResult::Composite;
    }
    let root_n = (n as f64).sqrt() as usize;
    for x in (start..(root_n + 1)).step_by(6) {
        if n % x == 0 || n % (x + 2) == 0 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::Prime
}
