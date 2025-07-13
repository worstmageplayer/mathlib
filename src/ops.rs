use crate::number::Number;
use crate::error::NumberError;
use std::ops::{Add, Sub, Mul, Div};

// Addition ---------------------------------------------------------------------------------------

/// Returns the sum of two Numbers.
#[inline]
pub fn add(a: &Number, b: &Number) -> Result<Number, NumberError> {
    // a/b + c/d = (ad + cb) / bd

    let (numerator, denominator);
    if a.denominator() == b.denominator() {
        numerator = a.numerator() + b.numerator();
        denominator = a.denominator().clone();
    } else {
        numerator = a.numerator() * b.denominator() + b.numerator() * a.denominator();
        denominator = a.denominator() * b.denominator();
    }

    Number::new(numerator, denominator)
}

impl Add for Number {
    type Output = Result<Number, NumberError>;

    fn add(self, other: Number) -> Self::Output {
        add(&self, &other)
    }
}

// Subtraction ------------------------------------------------------------------------------------

/// Returns the difference of two Numbers.
#[inline]
pub fn sub(a: &Number, b: &Number) -> Result<Number, NumberError> {
    // a/b - c/d = (ad - cb) / bd

    let (numerator, denominator);
    if a.denominator() == b.denominator() {
        numerator = a.numerator() - b.numerator();
        denominator = a.denominator().clone();
    } else {
        numerator = a.numerator() * b.denominator() - b.numerator() * a.denominator();
        denominator = a.denominator() * b.denominator();
    }

    Number::new(numerator, denominator)
}

impl Sub for Number {
    type Output = Result<Number, NumberError>;

    fn sub(self, other: Number) -> Self::Output {
        sub(&self, &other)
    }
}

// Multiplication ---------------------------------------------------------------------------------

/// Returns the product of two Numbers.
#[inline]
pub fn mul(a: &Number, b: &Number) -> Result<Number, NumberError> {
    // a/b * c/d = ac/bd
    if a.is_int() && b.is_int() {
        let result = a.numerator() * b.numerator();
        return Ok(Number::new_int(result));
    }

    let numerator = a.numerator() * b.numerator();
    let denominator = a.denominator() * b.denominator();

    Number::new(numerator, denominator)
}

impl Mul for Number {
    type Output = Result<Number, NumberError>;

    fn mul(self, other: Number) -> Self::Output {
        mul(&self, &other)
    }
}

// Division ---------------------------------------------------------------------------------------

/// Returns the quotient of two Numbers.
#[inline]
pub fn div(a: &Number, b: &Number) -> Result<Number, NumberError> {
    // a/b * c/d = ad/bc

    if b.is_zero() {
        return Err(NumberError::DivisionByZero);
    }

    let (numerator, denominator);
    if a.is_int() && b.is_int() {
        numerator = a.numerator().clone();
        denominator = b.numerator().clone();
    } else {
        numerator = a.numerator() * b.denominator();
        denominator = a.denominator() * b.numerator();
    }

    Number::new(numerator, denominator)
}

impl Div for Number {
    type Output = Result<Number, NumberError>;

    fn div(self, other: Number) -> Self::Output {
        div(&self, &other)
    }
}

