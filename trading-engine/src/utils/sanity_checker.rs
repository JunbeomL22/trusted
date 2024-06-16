use anyhow::{bail, Result};



pub fn check_positive_f64(value: f64, param: &str) -> Result<()> {
    if value <= 0.0 {
        bail!("invalid value f64 for {param}: {value}");
    }
    Ok(())
}

pub fn check_non_negative_f64(value: f64, param: &str) -> Result<()> {
    if value < 0.0 {
        bail!("invalid value f64 for {param}: {value}");
    }
    Ok(())
}

pub fn check_positive_i64(value: i64, param: &str) -> Result<()> {
    if value <= 0 {
        bail!("invalid value i64 for {param}: {value}");
    }
    Ok(())
}

pub fn check_in_range_inclusive_u8(value: u8, left: u8, right: u8, param: &str) -> Result<()> {
    if value < left || value > right {
        bail!("invalid value u8 for {param}: {value}");
    }
    Ok(())
}

pub fn check_in_range_inclusive_u64(value: u64, left: u64, right: u64, param: &str) -> Result<()> {
    if value < left || value > right {
        bail!("invalid value u64 for {param}: {value}");
    }
    Ok(())
}

pub fn check_in_range_inclusive_usize(value: usize, left: usize, right: usize, param: &str) -> Result<()> {
    if value < left || value > right {
        bail!("invalid value usize for {param}: {value}");
    }
    Ok(())
}

pub fn check_in_range_inclusive_f64(value: f64, left: f64, right: f64, param: &str) -> Result<()> {
    let eps = f64::EPSILON;
    if value.is_nan() || value.is_infinite() {
        bail!("invalid value f64 for {param}: {value}");
    }

    if value < left - eps || value > right + eps {
        bail!("invalid value f64 for {param}: {value}");
    }
    Ok(())
}