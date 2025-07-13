pub mod number;
pub mod compare;
pub mod ops;
pub mod error;

#[cfg(test)]
mod tests {
    use super::number::Number;
    use super::ops::{add, sub, mul, div};
    use super::error::NumberError;
    use num_bigint::BigInt;

    fn n(a: i64, b: i64) -> Number {
        Number::new(BigInt::from(a), BigInt::from(b)).unwrap()
    }

#[test]
    fn test_add() {
        assert_eq!(add(&n(1, 2), &n(1, 2)).unwrap(), n(1, 1)); // 1/2 + 1/2 = 1
        assert_eq!(add(&n(1, 3), &n(1, 6)).unwrap(), n(1, 2)); // 1/3 + 1/6 = 1/2
        assert_eq!(add(&n(-1, 2), &n(1, 2)).unwrap(), n(0, 1)); // -1/2 + 1/2 = 0
    }

#[test]
    fn test_sub() {
        assert_eq!(sub(&n(1, 1), &n(1, 2)).unwrap(), n(1, 2)); // 1 - 1/2 = 1/2
        assert_eq!(sub(&n(2, 3), &n(1, 6)).unwrap(), n(1, 2)); // 2/3 - 1/6 = 1/2
        assert_eq!(sub(&n(1, 2), &n(1, 2)).unwrap(), n(0, 1)); // 1/2 - 1/2 = 0
    }

#[test]
    fn test_mul() {
        assert_eq!(mul(&n(2, 3), &n(3, 4)).unwrap(), n(1, 2)); // 2/3 * 3/4 = 6/12 = 1/2
        assert_eq!(mul(&n(1, 2), &n(0, 1)).unwrap(), n(0, 1)); // x * 0 = 0
        assert_eq!(mul(&n(-1, 2), &n(2, 1)).unwrap(), n(-1, 1)); // -1/2 * 2 = -1
    }

#[test]
    fn test_div() {
        assert_eq!(div(&n(1, 2), &n(1, 4)).unwrap(), n(2, 1)); // 1/2 ÷ 1/4 = 2
        assert_eq!(div(&n(2, 3), &n(2, 3)).unwrap(), n(1, 1)); // x ÷ x = 1
        assert_eq!(div(&n(-1, 2), &n(1, 2)).unwrap(), n(-1, 1)); // -1/2 ÷ 1/2 = -1
    }

#[test]
    fn test_div_by_zero() {
        let result = div(&n(1, 2), &n(0, 1));
        assert!(matches!(result, Err(NumberError::ZeroDenominator)));
    }
}
