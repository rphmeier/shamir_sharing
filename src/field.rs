use std::fmt;
use std::ops::{Add, Mul, Sub, Div};

use num::{One, Zero};
use num::bigint::{BigInt, Sign};

// we just do secret sharing over this big prime (same order as secp256k1 curve).
// 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
thread_local! {
    pub static PRIME: BigInt = {
        let mut x = BigInt::one() << 256;

        x = x - (1u64 << 32);
        x = x - (1u64 << 9);
        x = x - (1u64 << 8);
        x = x - (1u64 << 7);
        x = x - (1u64 << 6);
        x = x - (1u64 << 4);
        x = x - 1u64;

        x
    };
}

// Integer on the prime field.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(BigInt);

impl Value {
    pub fn mod_inverse(&self) -> Self {
        PRIME.with(|prime| {
            assert!(!self.is_zero(), "Attempted division by zero");

            let zero_val = BigInt::zero();
            let (mut t, mut new_t) = (zero_val.clone(), BigInt::one());
            let (mut r, mut new_r) = (prime.clone(), self.0.clone());

            while new_r != zero_val {
                let quotient = &r / &new_r;

                let temp_t = new_t.clone();
                new_t = t - quotient.clone() * new_t;
                t = temp_t;

                let temp_r = new_r.clone();
                new_r = r - quotient * new_r; 
                r = temp_r;
            }

            if r > BigInt::one() { panic!("unable to invert {}", self.0) }
            if t < BigInt::zero() { t = t + prime }

            Value(t)
        })
    }
}

impl One for Value {
    fn one() -> Self {
        Value(BigInt::one())
    }
}

impl Zero for Value {
    fn zero() -> Self {
        Value(BigInt::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl From<u64> for Value {
    fn from(x: u64) -> Self {
        Value(x.into())
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Value {
        PRIME.with(move |prime| Value((self.0 + other.0) % prime))
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Value {
        PRIME.with(move |prime| Value((self.0 * other.0) % prime))
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Self) -> Value {
        if other <= self {
            Value(self.0 - other.0)
        } else {
            PRIME.with(move |prime| Value(prime - other.0 + self.0))
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Self) -> Value {
        let inv = other.mod_inverse();
        self * inv
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::rand::Rand for Value {
    fn rand<R: ::rand::Rng>(r: &mut R) -> Value {
        // the prime is less than 2^256, so create a random value in that range.
        let bytes: [u8; 32] = ::rand::Rand::rand(r);

        PRIME.with(move |prime| Value(BigInt::from_bytes_be(Sign::Plus, &bytes[..]) % prime))
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, PRIME};
    use num::{Zero, One};

    #[test]
    fn prime_order() {
        // assuming this proof is OK
        // https://safecurves.cr.yp.to/proof/115792089237316195423570985008687907853269984665640564039457584007908834671663.html

        PRIME.with(|prime| {
            let x: ::num::BigInt = "115792089237316195423570985008687907853269984665640564039457584007908834671663".parse().unwrap();
            assert_eq!(prime, &x);
        })
    }

    #[test]
    fn mod_inv() {
        for i in 1u64..1000 {
            let val = Value::from(i);
            let inv = val.mod_inverse();

            assert!(val * inv == Value::from(1u64));
        }
    }

    #[test]
    fn division() {
        for i in (0..100).map(|i| i + 236_662).map(Value::from) {
            for j in (0..100).map(|k| k + 4423).map(Value::from) {
                let div = i.clone() / j.clone();
                
                assert_eq!(div * j, i);
            }
        }
    }

    #[test]
    fn sub_wraparound() {
        let x = Value::zero() - Value::one();
        PRIME.with(|prime| assert_eq!(x.0, prime.clone() - 1));

        assert_eq!(x + Value::one(), Value::zero());
    }
}