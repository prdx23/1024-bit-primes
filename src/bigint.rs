use std::fmt;
use std::ops::{
    Add, Sub, Mul, Div, Rem, Shl, Shr,
    AddAssign, SubAssign, MulAssign,
    DivAssign, RemAssign, ShlAssign, ShrAssign,
};
use std::cmp::Ordering;

use crate::rng::insert_random_bytes;


const N: usize = 2048 / 64;


#[derive(Clone, Copy)]
pub struct BigInt {
    pub chunks: [u64; N],
}


impl BigInt {

    pub fn zero() -> Self {
        Self { chunks: [0; N] }
    }

    pub fn is_zero(&self) -> bool {
        self.chunks == [0; N]
    }

    pub fn is_even(&self) -> bool {
        self.chunks[0] & 1 == 0
    }

    pub fn random() -> Self {
        let mut bytes = [0; 1024 / 8];
        insert_random_bytes(&mut bytes).expect("Cannot access/dev/urandom");
        Self::from(bytes.as_slice())
    }

    pub fn modify(&mut self) {
        self.chunks[(N / 2) - 1] |= 0x8000000000000000u64;
        self.chunks[0] |= 1;
    }
}


impl From<u128> for BigInt {
    fn from(num: u128) -> Self {
        let mut chunks = [0; N];
        chunks[0] = num as u64;
        chunks[1] = (num >> 64) as u64;
        Self { chunks }
    }
}


impl From<&[u8]> for BigInt {
    fn from(bytes: &[u8]) -> Self {
        let mut chunks = [0; N];
        for (i, slice) in bytes.chunks(8).enumerate() {
            chunks[i] = u64::from_le_bytes(slice.try_into().unwrap());
        }
        Self { chunks }
    }
}


impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut start = false;

        for chunk in self.chunks.iter().rev() {
            if !start && *chunk == 0 { continue; }

            if !start {
                start = true;
                output.push_str(format!("{:b}", chunk).as_str());
            } else {
                output.push_str(format!("{:064b}", chunk).as_str());
            }
        }

        if !start { output.push('0'); }
        write!(f,"{}", output)
    }
}


impl BigInt {
    pub fn print_decimal(&self) {
        let mut bnum = self.clone();
        let mut scratch = BigInt::zero();


        let mut chunks_amt = N;
        while bnum.chunks[N - 1] == 0 {
            bnum <<= 64;
            chunks_amt -= 1;
        }

        let mut bits;
        let mut new_chunk;
        for x in 0..(chunks_amt * 64) {
            scratch <<= 1;

            if bnum.chunks[N - 1] & 0x8000000000000000u64 > 0 {
                scratch.chunks[0] |= 1;
            }

            if x == (chunks_amt * 64) - 1 { break; }

            for chunk in scratch.chunks.iter_mut() {
                if *chunk == 0 { continue; }
                new_chunk = 0;

                for i in (0..64).step_by(4).rev() {
                    bits = (*chunk & (0b1111u64 << i)) >> i;

                    if bits >= 5 { bits += 3; }
                    new_chunk <<= 4;
                    new_chunk += bits;
                }
                *chunk = new_chunk
            }
            bnum <<= 1;
        }

        println!();
        let mut start = false;
        for chunk in scratch.chunks.iter().rev() {
            if *chunk == 0 { continue; }

            for i in (0..64).step_by(4).rev() {
                bits = (*chunk & (0b1111u64 << i)) >> i;

                if !start && bits == 0 { continue; }
                if !start { start = true; }

                print!("{}", bits);
            }
        }
        println!();
    }
}


fn bigint_add(own: BigInt, other: BigInt) -> BigInt {
    let mut sum;
    let mut carry = 0;
    let mut sum_overflow;
    let mut carry_overflow;
    let mut result = BigInt::zero();

    let own_iter = own.chunks.iter();
    let other_iter = other.chunks.iter();

    for (i, (chunk1, chunk2)) in own_iter.zip(other_iter).enumerate() {
        (sum, sum_overflow) = chunk1.overflowing_add(*chunk2);
        (sum, carry_overflow) = sum.overflowing_add(carry);
        result.chunks[i] = sum;
        carry = sum_overflow as u64 + carry_overflow as u64;
    }

    if carry != 0 { panic!("Attempt to add with overflow"); }
    result
}


impl Add for BigInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        bigint_add(self, other)
    }
}


impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        *self = bigint_add(*self, other);
    }
}



fn bigint_sub(own: BigInt, other: BigInt) -> BigInt {
    let mut difference;
    let mut borrow = 0;
    let mut diff_overflow;
    let mut borrow_overflow;
    let mut result = BigInt::zero();

    let own_iter = own.chunks.iter();
    let other_iter = other.chunks.iter();

    for (i, (chunk1, chunk2)) in own_iter.zip(other_iter).enumerate() {
        (difference, diff_overflow) = chunk1.overflowing_sub(*chunk2);
        (difference, borrow_overflow) = difference.overflowing_sub(borrow);
        result.chunks[i] = difference;
        borrow = diff_overflow as u64 + borrow_overflow as u64;
    }

    if borrow != 0 { panic!("Attempt to subtract with overflow"); }
    result
}


impl Sub for BigInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        bigint_sub(self, other)
    }
}


impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        *self = bigint_sub(*self, other);
    }
}

impl BigInt {

    pub fn add_u64(mut self, inc:u64) -> BigInt {
        let mut sum;
        let mut carry = 0;
        (sum, carry) = self.chunks[0].overflowing_add(inc);
        self.chunks[0] = sum;
        let mut i = 1;
        while carry != 0{
            (sum, carry) = self.chunks[i].overflowing_add(1);
            self.chunks[i] = sum;
            i += 1;
            if i == N { panic!("Attempt to add with overflow"); }
        }

        self
    }

    pub fn sub_u64(mut self, inc:u64) -> BigInt {
        let mut diff;
        let mut borrow = 0;
        (diff, borrow) = self.chunks[0].overflowing_sub(dec);
        self.chunks[0] = diff;
        let mut i = 1;
        while borrow != 0{
            (diff, borrow) = self.chunks[i].overflowing_sub(1);
            self.chunks[i] = diff;
            i += 1;
            if i == N { panic!("Attempt to sub with overflow"); }
        }

        self
    }

    pub fn increase(mut self) -> BigInt {
        self.add_u64(1);
        self
    }

    pub fn decrease(mut self) -> BigInt {
        self.sub_u64(1);
        self
    }

    pub fn increase_by_2(mut self) -> BigInt {
        self.add_u64(2);
        self
    }

}


fn bigint_mul(own: BigInt, other: BigInt) -> BigInt {
    let mut result = BigInt::zero();
    let mut intermediate;
    let mut carry;

    let t = own.size();
    let n = other.size();
    if t + n + 1 >= N { panic!("Attempt to multiply with overflow"); }

    for (j, chunk2) in other.chunks.iter().take(n + 1).enumerate() {
        if *chunk2 == 0 { continue; }
        carry = 0;

        for (i, chunk1) in own.chunks.iter().take(t + 1).enumerate() {
            if *chunk1 == 0 && carry == 0 { continue; }

            intermediate = ((*chunk1 as u128) * (*chunk2 as u128)) + carry;
            intermediate += result.chunks[i + j] as u128;
            result.chunks[i + j] = intermediate as u64;
            carry = intermediate >> 64;
        }
        result.chunks[t + j + 1] += carry as u64;
    }
    result
}


impl Mul for BigInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        bigint_mul(self, other)
    }
}


impl MulAssign for BigInt {
    fn mul_assign(&mut self, other: Self) {
        *self = bigint_mul(*self, other);
    }
}


fn bigint_shl(own: BigInt, amount: usize) -> BigInt {
    debug_assert!(amount <= 128);
    let mut result = BigInt::zero();
    let mut overflow = 0;
    let mut shifted: u128;

    for (i, chunk) in own.chunks.iter().enumerate() {
        shifted = ((*chunk as u128) << amount) + overflow;
        result.chunks[i] = shifted as u64;
        overflow = shifted >> 64;
    }

    result
}


impl Shl<usize> for BigInt {
    type Output = Self;
    fn shl(self, amount: usize) -> Self {
        bigint_shl(self, amount)
    }
}


impl ShlAssign<usize> for BigInt {
    fn shl_assign(&mut self, amount: usize) {
        *self = bigint_shl(*self, amount);
    }
}


fn bigint_shr(own: BigInt, amount: usize) -> BigInt {
    debug_assert!(amount <= 128);
    let mut result = BigInt::zero();
    let mut overflow = 0;
    let mut shifted: u128;

    for (i, chunk) in own.chunks.iter().enumerate().rev() {
        shifted = ((*chunk as u128) << (64 - amount)) + (overflow << 64);
        overflow = (shifted as u64) as u128;
        result.chunks[i] = (shifted >> 64) as u64;
    }

    result
}


impl Shr<usize> for BigInt {
    type Output = Self;
    fn shr(self, amount: usize) -> Self {
        bigint_shr(self, amount)
    }
}


impl ShrAssign<usize> for BigInt {
    fn shr_assign(&mut self, amount: usize) {
        *self = bigint_shr(*self, amount);
    }
}


impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.chunks == other.chunks
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (c1, c2) in self.chunks.iter().zip(other.chunks.iter()).rev() {
            if *c1 != *c2 { return c1.partial_cmp(c2); }
        }
        Some(Ordering::Equal)
    }
}


impl BigInt {

    fn size(&self) -> usize {
        let mut n = N - 1;
        for chunk in self.chunks.iter().rev() {
            if *chunk != 0 { break; }
            n -= 1;
        }
        n
    }

}


fn bigint_div(mut dividend: BigInt, mut divisor: BigInt) -> (BigInt, BigInt) {
    if divisor.is_zero() { panic!("Attempt to divide by zero"); }
    if dividend < divisor { return (BigInt::zero(), dividend) }

    // x = dividend
    // y = divisor

    let mut quotient = BigInt::zero();
    let mut lambda = 0;

    let t = divisor.size();

    if divisor.chunks[t] < u64::MAX / 2 {
        while divisor.chunks[t] << lambda < u64::MAX / 2 {
            lambda += 1;
        }
        divisor <<= lambda;
        dividend <<= lambda;
    }

    let n = dividend.size();

    // if y has only 1 "digit", then do long division directly
    if t == 0 {
        let divisor_digit = divisor.chunks[0] as u128;
        let mut remainder = 0;
        let mut current = 0;

        for (i, chunk) in dividend.chunks.iter().enumerate().rev().skip(N - n - 1) {
            current = (remainder << 64) + *chunk as u128;
            quotient.chunks[i] = (current / divisor_digit) as u64;
            remainder = current % divisor_digit;
        }
        return (quotient, BigInt::from(remainder >> lambda));
    }

    // step 2, align and then subtract y from x until x >= aligned
    let mut aligned = divisor.clone();
    for _ in 0..(n - t) {
        aligned <<= 64;
    }

    while dividend >= aligned {
        quotient.chunks[n - t] += 1;
        dividend -= aligned;
    }

    let one = BigInt::from(1);
    let mut x_3digit;
    let mut y_2digit;
    let mut q_u128;
    let mut q_digit;

    // step 3
    for i in ((t + 1)..=n).rev() {

        q_digit = BigInt::zero();

        // step 3.1
        if dividend.chunks[i] == divisor.chunks[t] {
            q_digit.chunks[0] = u64::MAX - 1;
        } else {
            q_u128 = (dividend.chunks[i] as u128) << 64;
            q_u128 += dividend.chunks[i - 1] as u128;
            q_digit = BigInt::from(q_u128 / divisor.chunks[t] as u128);
        }

        // precalc 3digit x and 2digit y
        x_3digit = BigInt::zero();
        x_3digit.chunks[2] = dividend.chunks[i];
        x_3digit.chunks[1] = dividend.chunks[i - 1];
        x_3digit.chunks[0] = dividend.chunks[i - 2];

        y_2digit = BigInt::zero();
        y_2digit.chunks[1] = divisor.chunks[t];
        y_2digit.chunks[0] = divisor.chunks[t - 1];

        // step 3.2
        while q_digit * y_2digit > x_3digit {
            q_digit -= one;
        }

        // move quotient "digit" from temp bigint to its place in quotient
        quotient.chunks[i - t - 1] = q_digit.chunks[0];

        // precalc shifted y
        let mut y_shifted = divisor.clone();
        for _ in 0..(i - t - 1) {
            y_shifted <<= 64;
        }

        // step 3.3 and 3.4
        if dividend >= q_digit * y_shifted {
            dividend -= q_digit * y_shifted;
        } else {
            dividend += y_shifted;
            dividend -= q_digit * y_shifted;
            quotient.chunks[i - t - 1] -= 1;
        }
    }

    // rewind shifts by lambda to get actual remainder
    dividend >>= lambda;

    (quotient, dividend)
}


impl Div for BigInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        bigint_div(self, other).0
    }
}


impl DivAssign for BigInt {
    fn div_assign(&mut self, other: Self) {
        *self = bigint_div(*self, other).0;
    }
}


impl Rem for BigInt {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        bigint_div(self, other).1
    }
}


impl RemAssign for BigInt {
    fn rem_assign(&mut self, other: Self) {
        *self = bigint_div(*self, other).1;
    }
}
