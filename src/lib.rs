use std::io;
use std::io::Read;
use std::fs::File;


fn random_u16() -> io::Result<u16> {
    let mut bytes = [0u8; 2];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(u16::from_ne_bytes(bytes) | 0b1000000000000001)
}

#[derive(PartialEq)]
enum PrimeResult {
    Prime,
    Composite,
}

fn trial_division_simple(n: usize) -> PrimeResult {
    let root_n = (n as f64).sqrt() as usize;
    for x in 2..root_n {
        if n % x == 0 {
            return PrimeResult::Composite;
        }
    }
    PrimeResult::Prime
}


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
