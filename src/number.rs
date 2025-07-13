use num_bigint::{BigInt, Sign};
use Sign::Minus;
use num_traits::{Zero, One};
use num_integer::Integer;
use crate::error::NumberError;

/// Represents a rational number as a fraction.
///
/// Number uses BigInt to store a numerator and denominator.
#[derive(Clone)]
pub struct Number {
    numerator: BigInt,
    denominator: BigInt,
}

impl Number {
    /// Creates a new Number given a numerator and denominator.
    ///
    /// The fraction will be reduced to its lowest form
    /// and its denominator will always be positive.
    ///
    /// Returns NumberError if the denominator is zero.
    #[inline]
    pub fn new<N, D>(numerator: N, denominator: D) -> Result<Number, NumberError>
    where
        N: Into<BigInt>,
        D: Into<BigInt>,
    {
        let mut numerator = numerator.into();
        let mut denominator = denominator.into();

        if denominator.is_zero() {
            return Err(NumberError::ZeroDenominator);
        }

        if numerator.is_zero() {
            return Ok(Number::zero());
        }

        let gcd = numerator.gcd(&denominator);
        if !gcd.is_one() {
            numerator /= &gcd;
            denominator /= &gcd;
        }

        if denominator.sign() == Minus {
            numerator = -numerator;
            denominator = -denominator;
        }

        Ok(Number {
            numerator,
            denominator,
        })
    }

    /// Creates a new Number where its denominator is one.
    #[inline]
    pub fn new_int<A>(a: A) -> Number
    where A: Into<BigInt>
    {
        Number {
            numerator: a.into(),
            denominator: BigInt::one(),
        }
    }

    /// Creates a new Number without validating or reducing the fraction.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new_unsafe<A, B>(a: A, b: B) -> Number
    where
        A: Into<BigInt>,
        B: Into<BigInt>,
    {
        Number {
            numerator: a.into(),
            denominator: b.into(),
        }
    }

    #[inline]
    pub fn zero() -> Number {
        Number {
            numerator: BigInt::zero(),
            denominator: BigInt::one(),
        }
    }

    #[inline]
    pub fn one() -> Number {
        Number {
            numerator: BigInt::one(),
            denominator: BigInt::one(),
        }
    }

    #[inline]
    pub fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    #[inline]
    pub fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    #[inline]
    pub fn sign(&self) -> Sign {
        self.numerator.sign()
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        self.denominator.is_one()
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
}

use std::convert::From;

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::new_int(value)
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::new_int(value)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::new_int(value)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::new_int(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::new_int(value)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::new_int(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::new_int(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::new_int(value)
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::new_int(value)
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Number::new_int(value)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Number::new_int(value)
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number::new_int(value)
    }
}

use std::fmt;

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == BigInt::one() {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == BigInt::one() {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}
