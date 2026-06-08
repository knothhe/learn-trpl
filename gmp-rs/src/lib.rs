use std::fmt;

#[derive(Clone)]
pub struct Bigint {
    digits: Vec<u64>,
}

const BIGINT_BASE: u64 = 1_000_000_000_000_000_000;

impl Bigint {
    pub fn new(n: u64) -> Self {
        if n < BIGINT_BASE {
            Self { digits: vec![n] }
        } else {
            Self {
                digits: vec![n % BIGINT_BASE, n / BIGINT_BASE],
            }
        }
    }

    pub fn add(&mut self, other: &Self) {
        let common_len = self.digits.len().min(other.digits.len());
        let mut carry = 0_u64;

        for index in 0..common_len {
            let sum = self.digits[index] + other.digits[index] + carry;
            if sum >= BIGINT_BASE {
                self.digits[index] = sum - BIGINT_BASE;
                carry = 1;
            } else {
                self.digits[index] = sum;
                carry = 0;
            }
        }

        if self.digits.len() < other.digits.len() {
            self.digits.reserve(other.digits.len() - common_len + 1);

            let mut index = common_len;
            while index < other.digits.len() && carry != 0 {
                let sum = other.digits[index] + carry;
                if sum >= BIGINT_BASE {
                    self.digits.push(sum - BIGINT_BASE);
                } else {
                    self.digits.push(sum);
                    carry = 0;
                }
                index += 1;
            }
            self.digits.extend_from_slice(&other.digits[index..]);
        } else {
            let mut index = common_len;
            while index < self.digits.len() && carry != 0 {
                let sum = self.digits[index] + carry;
                if sum >= BIGINT_BASE {
                    self.digits[index] = sum - BIGINT_BASE;
                } else {
                    self.digits[index] = sum;
                    carry = 0;
                }
                index += 1;
            }
        }

        if carry != 0 {
            self.digits.push(carry);
        }
    }
}

impl fmt::Display for Bigint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut digits = self.digits.iter().rev();
        if let Some(digit) = digits.next() {
            write!(f, "{digit}")?;
        }
        for digit in digits {
            write!(f, "{digit:018}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_all_u64_values() {
        assert_eq!(Bigint::new(u64::MAX).to_string(), u64::MAX.to_string());
    }

    #[test]
    fn adds_with_carry() {
        let mut value = Bigint::new(BIGINT_BASE - 1);
        value.add(&Bigint::new(1));
        assert_eq!(value.to_string(), BIGINT_BASE.to_string());
    }

    #[test]
    fn adds_different_lengths() {
        let mut short = Bigint::new(1);
        short.add(&Bigint::new(u64::MAX));
        assert_eq!(short.to_string(), (u64::MAX as u128 + 1).to_string());

        let mut long = Bigint::new(u64::MAX);
        long.add(&Bigint::new(1));
        assert_eq!(long.to_string(), (u64::MAX as u128 + 1).to_string());
    }
}
