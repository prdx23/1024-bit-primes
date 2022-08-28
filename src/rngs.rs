use std::io;
use std::io::Read;
use std::fs::File;


pub fn random_u16() -> io::Result<u16> {
    let mut bytes = [0u8; 2];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(u16::from_ne_bytes(bytes) | 0b1000000000000001)
}


pub fn random_u64() -> io::Result<u64> {
    let mut bytes = [0u8; 8];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(u64::from_ne_bytes(bytes) | 0x8000000000000001u64)
}
