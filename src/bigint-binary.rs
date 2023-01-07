use std::fmt;
use std::cmp::Ordering;
use std::ops::{
    Add, Sub, Shl, Shr, Mul, Div, Rem,
    AddAssign, SubAssign, ShlAssign, ShrAssign,
    MulAssign, DivAssign, RemAssign,
};

use crate::rng::insert_random_bytes;


const N: usize = 2048;


#[derive(Copy, Clone)]
pub struct BigInt {
    pub bits: [bool; N]
}

impl BigInt {
    pub fn zero() -> Self {
        Self { bits: [false; N] }
    }

    pub fn random() -> Self {
        let mut bytes = [0u8; N / 16];
        insert_random_bytes(&mut bytes).expect("Cannot access /dev/urandom");

        let mut bits = [false; N];
        let mut i = 0;
        for byte in bytes {
            for position in 0..8 {
                let mask = 1 << position;
                bits[i] = byte & mask > 0;
                i += 1;
            }
        }

        Self { bits }
    }

    pub fn modify(&mut self) {
        self.bits[0] = true;
        self.bits[N - (N / 2) - 1] = true;
    }

    pub fn random_range(min: BigInt, max: BigInt) -> BigInt {
        let mut num = BigInt::random();
        if num > max {
            num -= num - max - BigInt::from(rng::u128());
        }
        num
    }
}


impl From<u128> for BigInt {
    fn from(n: u128) -> Self {
        let mut bits = [false; N];
        let mut i = 0;

        for byte in n.to_le_bytes() {
            for position in 0..8 {
                let mask = 1 << position;
                bits[i] = byte & mask > 0;
                i += 1;
            }
        }

        Self { bits }
    }
}


impl From<&[bool]> for BigInt {
    fn from(other: &[bool]) -> Self {
        let mut bits = [false; N];
        for (i, bit) in other.iter().enumerate() {
            bits[i] = *bit;
        }
        Self { bits }
    }
}


impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut start = false;

        for bit in self.bits.iter().rev() {
            if !start && *bit == false { continue; }
            if !start { start = true; }

            output.push(match *bit {
                true  => '1',
                false => '0',
            })
        }

        if !start { output.push('0'); }
        write!(f,"{}", output)
    }
}


impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        for (b1, b2) in self.bits.iter().zip(other.bits.iter()).rev() {
            if *b1 && !*b2 { return Some(Ordering::Greater); }
            else if !*b1 && *b2 { return Some(Ordering::Less); }
        }

        Some(Ordering::Equal)
    }
}


fn bigint_add(own: &[bool], other: &[bool]) -> [bool; N] {
    let mut bits = [false; N];
    let mut carry = false;

    for (i, (d1, d2)) in own.iter().zip(other.iter()).enumerate() {
        bits[i] = d1 ^ d2 ^ carry;
        carry = (d1 & d2) | (carry & (d1 ^ d2));
    }

    if carry { panic!("Attempt to add with overflow"); }
    bits
}


impl Add for BigInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { bits: bigint_add(&self.bits, &other.bits) }
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        self.bits = bigint_add(&self.bits, &other.bits);
    }
}



fn bigint_sub(own: &[bool], other: &[bool]) -> [bool; N] {
    let mut bits = [false; N];
    let mut borrow = false;

    for (i, (d1, d2)) in own.iter().zip(other.iter()).enumerate() {
        bits[i] = d1 ^ d2 ^ borrow;
        borrow = (!d1 & d2) | (borrow & !(d1 ^ d2));
    }

    if borrow { panic!("Attempt to subtract with overflow"); }
    bits
}

impl Sub for BigInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { bits: bigint_sub(&self.bits, &other.bits) }
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        self.bits = bigint_sub(&self.bits, &other.bits);
    }
}



fn bigint_shl(own: &[bool], amount: usize) -> [bool; N] {
    let mut bits = [false; N];
    if amount > N { return bits; }

    let mut i = amount;
    for bit in own.iter().take(N - amount) {
        bits[i] = *bit;
        i += 1;
    }

    bits
}


impl Shl<usize> for BigInt {
    type Output = Self;
    fn shl(self, amount: usize) -> Self {
        Self { bits: bigint_shl(&self.bits, amount) }
    }
}

impl ShlAssign<usize> for BigInt {
    fn shl_assign(&mut self, amount: usize) {
        self.bits = bigint_shl(&self.bits, amount);
    }
}


fn bigint_shr(own: &[bool], amount: usize) -> [bool; N] {
    let mut bits = [false; N];
    if amount > N { return bits; }

    let mut i = 0;
    for bit in own.iter().skip(amount) {
        bits[i] = *bit;
        i += 1;
    }

    bits
}


impl Shr<usize> for BigInt {
    type Output = Self;
    fn shr(self, amount: usize) -> Self {
        Self { bits: bigint_shr(&self.bits, amount) }
    }
}

impl ShrAssign<usize> for BigInt {
    fn shr_assign(&mut self, amount: usize) {
        self.bits = bigint_shr(&self.bits, amount);
    }
}



fn bigint_mul(own: &[bool], other: &[bool]) -> [bool; N] {
    let mut result = BigInt::zero();
    let n1 = BigInt::from(own);
    let mut current;

    for (shift, d2) in other.iter().enumerate() {
        if !(*d2) { continue; }

        for i in (N - shift)..N {
            if own[i] { panic!("Attempt to multiply with overflow"); }
        }

        current = n1 << shift;
        result += current;
    }

    result.bits
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self { bits: bigint_mul(&self.bits, &other.bits) }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, other: Self) {
        self.bits = bigint_mul(&self.bits, &other.bits);
    }
}



fn bigint_div(own_bits: &[bool], other_bits: &[bool]) -> ([bool; N], [bool; N]) {
    let mut quotient = BigInt::zero();
    let mut dividend = BigInt::from(own_bits);
    let mut remainder = BigInt::zero();
    let divisor = BigInt::from(other_bits);

    if divisor == BigInt::zero() {
        panic!("Attempt to divide by zero");
    }

    let mut no_of_bits = N;
    while !dividend.bits[N - 1] {
        dividend <<= 1;
        no_of_bits -= 1;
    }

    for i in 0..no_of_bits {
        remainder <<= 1;
        remainder.bits[0] = dividend.bits[N - 1 - i];

        quotient <<= 1;
        if remainder >= divisor {
            remainder -= divisor;
            quotient.bits[0] = true;
        }
    }

    (quotient.bits, remainder.bits)
}

impl Div for BigInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self { bits: bigint_div(&self.bits, &other.bits).0 }
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, other: Self) {
        self.bits = bigint_div(&self.bits, &other.bits).0;
    }
}

impl Rem for BigInt {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Self { bits: bigint_div(&self.bits, &other.bits).1 }
    }
}

impl RemAssign for BigInt {
    fn rem_assign(&mut self, other: Self) {
        self.bits = bigint_div(&self.bits, &other.bits).1;
    }
}

#[cfg(test)]
mod tests {
    use crate::rng;
    use super::*;

    #[test]
    fn from_u128() {
        let num = rng::u128();
        assert_eq!(format!("{:b}", num), format!("{}", BigInt::from(num)));
    }

    #[test]
    fn add() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u64() as u128;
        let expected = num1 + num2;
        let test = BigInt::from(num1) + BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", test));
    }

    #[test]
    fn add_assign() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u64() as u128;
        let expected = num1 + num2;
        let mut n = BigInt::from(num1);
        n += BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", n));
    }

    #[test]
    fn sub() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u128_range(1, num1) as u128;
        let expected = num1 - num2;
        let test = BigInt::from(num1) - BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", test));
    }

    #[test]
    fn sub_assign() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u128_range(1, num1) as u128;
        let expected = num1 - num2;
        let mut n = BigInt::from(num1);
        n -= BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", n));
    }

    #[test]
    fn shl() {
        let num1 = rng::u64() as u128;
        let amount = 10;
        let expected = num1 << amount;
        let test = BigInt::from(num1) << amount;
        assert_eq!(format!("{:b}", expected), format!("{}", test));
    }

    #[test]
    fn shl_assign() {
        let num1 = rng::u64() as u128;
        let amount = 10;
        let expected = num1 << amount;
        let mut n = BigInt::from(num1);
        n <<= amount;
        assert_eq!(format!("{:b}", expected), format!("{}", n));
    }

    #[test]
    fn shr() {
        let num1 = rng::u64() as u128;
        let amount = 10;
        let expected = num1 >> amount;
        let test = BigInt::from(num1) >> amount;
        assert_eq!(format!("{:b}", expected), format!("{}", test));
    }

    #[test]
    fn shr_assign() {
        let num1 = rng::u64() as u128;
        let amount = 10;
        let expected = num1 >> amount;
        let mut n = BigInt::from(num1);
        n >>= amount;
        assert_eq!(format!("{:b}", expected), format!("{}", n));
    }

    #[test]
    fn mul() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u64() as u128;
        let expected = num1 * num2;
        let test = BigInt::from(num1) * BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", test));
    }

    #[test]
    fn mul_assign() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u64() as u128;
        let expected = num1 * num2;
        let mut n = BigInt::from(num1);
        n *= BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", n));
    }

    #[test]
    fn div() {
        let num1 = rng::u64() as u128;
        let num2 = (rng::u64() as u16) as u128;
        let expected = num1 / num2;
        let test = BigInt::from(num1) / BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", test));
    }

    #[test]
    fn div_assign() {
        let num1 = rng::u64() as u128;
        let num2 = (rng::u64() as u16) as u128;
        let expected = num1 / num2;
        let mut n = BigInt::from(num1);
        n /= BigInt::from(num2);
        assert_eq!(format!("{:b}", expected), format!("{}", n));
    }
}
