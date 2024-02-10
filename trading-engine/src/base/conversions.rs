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