use std::fmt;

pub struct Bigint {
    digits: Vec<u64>,
}

const BIGINT_BASE: u64 = 1_000_000_000_000_000_000;

impl Bigint {
    pub fn new(n: u64) -> Self {
        Self { digits: vec![n] }
    }

    pub fn add(&mut self, other: &Self) {
        let mut carry = 0;
        for (a, b) in self.digits.iter_mut().zip(other.digits.iter()) {
            let sum = *a + *b + carry;
            *a = sum % BIGINT_BASE;
            carry = sum / BIGINT_BASE;
        }
        // a.len not equal to other.len, so other.digits may have more digits than self.digits
        // this is b.len > a.len, so we need to iterate over the remaining digits of other
        for b in other.digits.iter().skip(self.digits.len()) {
            let sum = *b + carry;
            self.digits.push(sum % BIGINT_BASE);
            carry = sum / BIGINT_BASE;
        }
        // if a.len > b.len, we need to iterate over the remaining digits of self
        // if there are still digits in self, we need to add them to the result
        for a in self.digits.iter_mut().skip(other.digits.len()) {
            let sum = *a + carry;
            *a = sum % BIGINT_BASE;
            carry = sum / BIGINT_BASE;
        }
        // if there is still a carry, we need to add it to the result
        while carry > 0 {
            self.digits.push(carry % BIGINT_BASE);
            carry /= BIGINT_BASE;
        }
    }
}

impl fmt::Display for Bigint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // add padding zeros
        for (index, digit) in self.digits.iter().rev().enumerate() {
            if index == 0 {
                write!(f, "{}", digit)?;
            } else {
                write!(f, "{:018}", digit)?;
            }
        }
        Ok(())
    }
}

impl Clone for Bigint {
    fn clone(&self) -> Self {
        Self {
            digits: self.digits.clone(),
        }
    }
}
