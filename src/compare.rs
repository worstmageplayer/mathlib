#![allow(dead_code)]

use num_bigint::Sign;
use Sign::{Plus, NoSign, Minus};
use std::cmp::Ordering;
use Ordering::{Less, Equal, Greater};
use crate::number::Number;

/// Compares two Number values
///
/// Returns their relative ordering
pub fn compare(a: &Number, b: &Number) -> Ordering {
    match (a.sign(), b.sign()) {
        (Plus, Plus) => {
            let a_new = a.numerator() * b.denominator();
            let b_new = b.numerator() * a.denominator();
            a_new.cmp(&b_new)
        },
        (Minus, Minus) => {
            let a_new = a.numerator() * b.denominator();
            let b_new = b.numerator() * a.denominator();
            a_new.cmp(&b_new).reverse()
        },
        (NoSign, Plus) => Less,
        (NoSign, NoSign) => Equal,
        (NoSign, Minus) => Greater,
        (Plus, NoSign) => Greater,
        (Plus, Minus) => Greater,
        (Minus, NoSign) => Less,
        (Minus, Plus) => Less,
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        compare(self, other) == Equal
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}
