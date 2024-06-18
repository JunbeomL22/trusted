use enum_dispatch::enum_dispatch;
use anyhow::{Result, anyhow};

pub const MAX_IO_PRECISION: u8 = 9;
pub const MAX_IO_MULTIPLIER: f64 = 1_000_000_000.0; // 10.0**MAX_IO_PRECISION
pub const PRICE_MAX: f64 = 9_223_372_036.0; // i64::MAX / 1000_000_000
pub const PRICE_MIN: f64 = -9_223_372_036.0; // i64::MIN / 1000_000_000
pub const QUANTITY_MAX: f64 = 18_446_744_073.0; // u64::MAX / 1000_000_000
pub const QUANTITY_MIN: f64 = 0.0;

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Prec2;

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Prec3;

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Prec8;

#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Prec9;

#[enum_dispatch]
pub trait Precision {
    fn precision() -> u8;
    fn check_precesion_bound() -> bool { Self::precision() <= MAX_IO_PRECISION }
    fn check_f64price_bound(price: f64) -> bool { 
        (- PRICE_MIN <= price) && ( price <= PRICE_MAX )
    }

    fn check_f64quantity_bound(quantity: f64) -> bool {
        (QUANTITY_MIN <= quantity) && (quantity <= QUANTITY_MAX)
    }

    #[must_use]
    fn price_f64_to_i64(value: f64) -> Result<i64>;

    #[must_use]
    fn price_i64_to_f64(value: i64) -> f64 {
        value as f64 / MAX_IO_MULTIPLIER
    }
    #[must_use]
    fn quantity_f64_to_u64(value: f64) -> Result<u64>;

    #[must_use]
    fn quantity_u64_to_f64(value: u64) -> f64 {
        value as f64 / MAX_IO_MULTIPLIER
    }
}

impl Precision for Prec2 {
    fn precision() -> u8 { 2 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec2::check_f64price_bound(value) {
            let rounded = (value * 100.0).round() as i64;
            Ok(rounded * 10_000_000_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value: f64) -> Result<u64> {
        if Prec2::check_f64price_bound(value) {
            let rounded = (value * 100.0).round() as u64;
            Ok(rounded * 10_000_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec8 {
    fn precision() -> u8 { 8 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec8::check_f64price_bound(value) {
            let rounded = (value * 100_000_000.0).round() as i64;
            Ok(rounded)
        } else {
            let error = || anyhow!("price:{price} out of bound", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec8::check_f64quantity_bound(value) {
            let rounded = (value * 100_000_000.0).round() as u64;
            Ok(rounded)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec3 {
    fn precision() -> u8 { 3 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec3::check_f64price_bound(value) {
            let rounded = (value * 1_000.0).round() as i64;
            Ok(rounded * 1_000_000_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec3::check_f64quantity_bound(value) {
            let rounded = (value * 1_000.0).round() as u64;
            Ok(rounded * 1_000_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec9 {
    fn precision() -> u8 { 9 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec9::check_f64price_bound(value) {
            let rounded = (value * 1_000_000_000.0).round() as i64;
            Ok(rounded)
        } else {
            let error = || anyhow!("price:{price} out of bound", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec9::check_f64quantity_bound(value) {
            let rounded = (value * 1_000_000_000.0).round() as u64;
            Ok(rounded)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound", qnt = value);
            Err(error())
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision() {
        assert_eq!(Prec2::precision(), 2);
        assert_eq!(Prec9::precision(), 9);
    }
    #[test]
    fn test_conversion_and_reversion() {

        let price = 1.2345678912345;

        let ioi64 = Prec2::f64_to_ioi64(price);
        let reversed_price = Prec2::ioi64_to_f64(ioi64);
        assert_eq!(ioi64, 1_230_000_000);
        assert_eq!(reversed_price, 1.23);

        let ioi64 = Prec9::f64_to_ioi64(price);
        let reversed_price = Prec9::ioi64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_567_891);
        assert_eq!(reversed_price, 1.234567891);
    }
}

