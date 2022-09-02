use std::io;
use std::io::Read;
use std::fs::File;



pub fn insert_random_bytes(mut bytes: &mut[u8]) -> io::Result<()> {
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(())
}



pub fn u16() -> u16 {
    let mut bytes = [0u8; 2];
    insert_random_bytes(&mut bytes).expect("Cannot access /dev/urandom");
    u16::from_ne_bytes(bytes)
}



pub fn u64() -> u64 {
    let mut bytes = [0u8; 8];
    insert_random_bytes(&mut bytes).expect("Cannot access /dev/urandom");
    u64::from_ne_bytes(bytes)
}



pub fn u128() -> u128 {
    let mut bytes = [0u8; 16];
    insert_random_bytes(&mut bytes).expect("Cannot access /dev/urandom");
    u128::from_ne_bytes(bytes)
}



pub fn u128_range(min: u128, max: u128) -> u128 {
    loop {
        // let x = self::u128();
        let x = self::u64() as u128;
        if x > min && x < max { return x; }
    }
}

