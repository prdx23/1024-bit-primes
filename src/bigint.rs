use std::fmt;
use std::ops::{
    Add, Sub,
    AddAssign, SubAssign
};

use crate::rng;


// const B: usize = 4096;
// const N: usize = 1234;

// const N: usize = 256;
const N: usize = 78;


pub struct BigInt {
    pub digits: [u8; N]
}



impl Add for BigInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut digits = [0u8; N];
        let mut carry = 0;

        let mut i = 0;
        for (d1, d2) in self.digits.iter().zip(other.digits.iter()) {
            digits[i] = (d1 + d2 + carry) % 10;
            carry = (d1 + d2 + carry) / 10;
            i += 1;
        }

        if carry != 0 { panic!("Attempt to add with overflow"); }
        Self { digits }
    }
}



impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        let mut digits = [0u8; N];
        let mut carry = 0;
        let mut i = 0;
        for (d1, d2) in self.digits.iter().zip(other.digits.iter()) {
            digits[i] = (d1 + d2 + carry) % 10;
            carry = (d1 + d2 + carry) / 10;
            i += 1;
        }

        if carry != 0 { panic!("Attempt to add with overflow"); }
        self.digits = digits;
    }
}



impl Sub for BigInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut digits = [0u8; N];
        let mut borrow = 0;
        let mut i = 0;
        for (d1, d2) in self.digits.iter().zip(other.digits.iter()) {
            if *d1 < *d2 + borrow {
                digits[i] = 10 + d1 - borrow - d2;
                borrow = 1;
            } else {
                digits[i] = d1 - borrow - d2;
                borrow = 0;
            }
            i += 1;
        }

        if borrow != 0 { panic!("Attempt to subtract with overflow"); }
        Self { digits }
    }
}



impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        let mut digits = [0u8; N];
        let mut borrow = 0;
        let mut i = 0;
        for (d1, d2) in self.digits.iter().zip(other.digits.iter()) {
            if *d1 < *d2 + borrow {
                digits[i] = 10 + d1 - borrow - d2;
                borrow = 1;
            } else {
                digits[i] = d1 - borrow - d2;
                borrow = 0;
            }
            i += 1;
        }

        if borrow != 0 { panic!("Attempt to subtract with overflow"); }
        self.digits = digits;
    }
}



impl From<u128> for BigInt {
    fn from(mut n: u128) -> Self {
        let mut digits = [0; N];
        let mut i = 0;

        while n > 0 {
            digits[i] = (n % 10) as u8;
            n = n / 10;
            i += 1;
        }
        Self { digits }
    }
}



impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut start = false;

        for digit in self.digits.iter().rev() {
            if !start && *digit == 0 { continue; }
            if !start { start = true; }
            output.push(char::from_digit(*digit as u32, 10).unwrap());
        }

        if !start { output.push('0'); }
        write!(f,"{}", output)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u128() {
        let num = rng::u128();
        assert_eq!(format!("{}", num), format!("{}", BigInt::from(num)));
    }

    #[test]
    fn add() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u64() as u128;
        let expected = num1 + num2;
        let test = BigInt::from(num1) + BigInt::from(num2);
        assert_eq!(format!("{}", expected), format!("{}", test));
    }

    #[test]
    fn add_assign() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u64() as u128;
        let expected = num1 + num2;
        let mut n = BigInt::from(num1);
        n += BigInt::from(num2);
        assert_eq!(format!("{}", expected), format!("{}", n));
    }

    #[test]
    fn sub() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u128_range(1, num1) as u128;
        let expected = num1 - num2;
        let test = BigInt::from(num1) - BigInt::from(num2);
        assert_eq!(format!("{}", expected), format!("{}", test));
    }

    #[test]
    fn sub_assign() {
        let num1 = rng::u64() as u128;
        let num2 = rng::u128_range(1, num1) as u128;
        let expected = num1 - num2;
        let mut n = BigInt::from(num1);
        n -= BigInt::from(num2);
        assert_eq!(format!("{}", expected), format!("{}", n));
    }

    // #[test]
    // #[should_panic(expected = "Attempt to add with overflow")]
    // fn add_overflow() {
    //     let large1 = BigInt { digits: [9u8; N] };
    //     let large2 = BigInt { digits: [9u8; N] };
    //     let _ = large1 + large2;
    // }
}
