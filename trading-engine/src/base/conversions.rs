/// Conversion functions for fixed-point arithmetic.
/// reference: https://github.com/nautechsystems/nautilus_trader
/// 
use anyhow::{bail, Result};

pub const FIXED_PRECISION: u8 = 9;
pub const FIXED_SCALAR: f64 = 1_000_000_000.0; // 10.0**FIXED_PRECISION

pub fn check_fixed_precision(precision: u8) -> Result<()> {
    if precision > FIXED_PRECISION {
        bail!("Condition failed: `precision` was greater than the maximum `FIXED_PRECISION` (9), was {precision}")
    }
    Ok(())
}

#[must_use]
pub fn f64_to_fixed_i64(value: f64, precision: u8) -> i64 {
    assert!(precision <= FIXED_PRECISION, "precision exceeded maximum 9");
    let pow1 = 10_i64.pow(u32::from(precision));
    let pow2 = 10_i64.pow(u32::from(FIXED_PRECISION - precision));
    let rounded = (value * pow1 as f64).round() as i64;
    rounded * pow2
}

pub fn f64_to_fixed_u64(value: f64, precision: u8) -> u64 {
    assert!(precision <= FIXED_PRECISION, "precision exceeded maximum 9");
    let pow1 = 10_u64.pow(u32::from(precision));
    let pow2 = 10_u64.pow(u32::from(FIXED_PRECISION - precision));
    let rounded = (value * pow1 as f64).round() as u64;
    rounded * pow2
}

#[must_use]
pub fn fixed_i64_to_f64(value: i64) -> f64 {
    (value as f64) / FIXED_SCALAR
}

#[must_use]
pub fn fixed_u64_to_f64(value: u64) -> f64 {
    (value as f64) / FIXED_SCALAR
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_fixed_precision() {
        assert!(check_fixed_precision(8).is_ok());
        assert!(check_fixed_precision(10).is_err());
    }

    #[test]
    fn test_f64_to_fixed_i64() {
        assert_eq!(f64_to_fixed_i64(1.234, 3), 1234000000);
        assert_eq!(f64_to_fixed_i64(1.2345, 9), 1234500000);
        assert_eq!(f64_to_fixed_i64(1_000_000_000.5, 1), 1_000_000_000_500_000_000);
    }
}

