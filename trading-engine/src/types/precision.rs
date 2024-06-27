use crate::utils::numeric_converter::{
    IntegerConverter,
    FloatConverter,
};
use once_cell::sync::Lazy;
use anyhow::{Result, anyhow};

pub const MAX_IO_PRECISION: u8 = 9;
pub const MAX_IO_MULTIPLIER: f64 = 1_000_000_000.0; // 10.0**MAX_IO_PRECISION
pub const PRICE_MAX: f64 = 9_223_372_036.0; // i64::MAX / 1000_000_000
pub const PRICE_MIN: f64 = -9_223_372_036.0; // i64::MIN / 1000_000_000
pub const QUANTITY_MAX: f64 = 18_446_744_073.0; // u64::MAX / 1000_000_000
pub const QUANTITY_MIN: f64 = 0.0;

pub trait Precision {
    fn precision() -> u8;

    fn check_precision_bound() -> bool { Self::precision() <= MAX_IO_PRECISION }

    fn check_f64price_bound(price: f64) -> bool { 
        (PRICE_MIN <= price) && ( price <= PRICE_MAX )
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

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec0;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec1;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec2;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec3;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec4;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec5;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec6;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec7;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec8;
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)] pub struct Prec9;

impl Precision for Prec0 {
    fn precision() -> u8 { 0 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec0::check_f64price_bound(value) {
            Ok((value.round() as i64) * 1_000_000_000_i64)
        } else {
            let error = || anyhow!("price: {price} out of bound (called from Prec0)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value: f64) -> Result<u64> {
        if Prec0::check_f64quantity_bound(value) {
            Ok((value.round() as u64) * 1_000_000_000_u64)
        } else {
            let error = || anyhow!("quantity: {qnt} out of bound (called from Prec0)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec1 {
    fn precision() -> u8 { 1 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec1::check_f64price_bound(value) {
            let rounded = (value * 10.0).round() as i64;
            Ok(rounded * 100_000_000_i64)
        } else {
            let error = || anyhow!("price: {price} out of bound (called from Prec1)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec1::check_f64quantity_bound(value) {
            let rounded = (value * 10.0).round() as u64;
            Ok(rounded * 100_000_000_u64)
        } else {
            let error = || anyhow!("quantity: {qnt} out of bound (called from Prec1)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec2 {
    fn precision() -> u8 { 2 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec2::check_f64price_bound(value) {
            let rounded = (value * 100.0).round() as i64;
            Ok(rounded * 10_000_000_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound (called from Prec2)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec2::check_f64quantity_bound(value) {
            let rounded = (value * 100.0).round() as u64;
            Ok(rounded * 10_000_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec2)", qnt = value);
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
            let error = || anyhow!("price:{price} out of bound (called from Prec3)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec3::check_f64quantity_bound(value) {
            let rounded = (value * 1_000.0).round() as u64;
            Ok(rounded * 1_000_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec3)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec4 {
    fn precision() -> u8 { 4 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec4::check_f64price_bound(value) {
            let rounded = (value * 10_000.0).round() as i64;
            Ok(rounded * 100_000_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound (called from Prec4)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec4::check_f64quantity_bound(value) {
            let rounded = (value * 10_000.0).round() as u64;
            Ok(rounded * 100_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec4)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec5 {
    fn precision() -> u8 { 5 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec5::check_f64price_bound(value) {
            let rounded = (value * 100_000.0).round() as i64;
            Ok(rounded * 10_000_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound (called from Prec5)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec5::check_f64quantity_bound(value) {
            let rounded = (value * 100_000.0).round() as u64;
            Ok(rounded * 10_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec5)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec6 {
    fn precision() -> u8 { 6 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec6::check_f64price_bound(value) {
            let rounded = (value * 1_000_000.0).round() as i64;
            Ok(rounded * 1_000_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound (called from Prec6)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec6::check_f64quantity_bound(value) {
            let rounded = (value * 1_000_000.0).round() as u64;
            Ok(rounded * 1_000_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec6)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec7 {
    fn precision() -> u8 { 7 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec7::check_f64price_bound(value) {
            let rounded = (value * 10_000_000.0).round() as i64;
            Ok(rounded * 100_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound (called from Prec7)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec7::check_f64quantity_bound(value) {
            let rounded = (value * 10_000_000.0).round() as u64;
            Ok(rounded * 100_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec7)", qnt = value);
            Err(error())
        }
    }
}

impl Precision for Prec8 {
    fn precision() -> u8 { 8 }

    fn price_f64_to_i64(value: f64) -> Result<i64> {
        if Prec8::check_f64price_bound(value) {
            let rounded = (value * 100_000_000.0).round() as i64;
            Ok(rounded * 10_i64)
        } else {
            let error = || anyhow!("price:{price} out of bound (called from Prec8)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec8::check_f64quantity_bound(value) {
            let rounded = (value * 100_000_000.0).round() as u64;
            Ok(rounded * 10_u64)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec8)", qnt = value);
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
            let error = || anyhow!("price:{price} out of bound (called from Prec9)", price = value);
            Err(error())
        }
    }

    fn quantity_f64_to_u64(value:f64) -> Result<u64> {
        if Prec9::check_f64quantity_bound(value) {
            let rounded = (value * 1_000_000_000.0).round() as u64;
            Ok(rounded)
        } else {
            let error = || anyhow!("quantity:{qnt} out of bound (called from Prec9)", qnt = value);
            Err(error())
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision() {
        assert_eq!(Prec0::precision(), 0);
        assert_eq!(Prec1::precision(), 1);
        assert_eq!(Prec2::precision(), 2);
        assert_eq!(Prec3::precision(), 3);
        assert_eq!(Prec4::precision(), 4);
        assert_eq!(Prec5::precision(), 5);
        assert_eq!(Prec6::precision(), 6);
        assert_eq!(Prec7::precision(), 7);
        assert_eq!(Prec8::precision(), 8);
        assert_eq!(Prec9::precision(), 9);
    }

    #[test]
    fn test_price_conversion_and_reversion() -> Result<()> {
        let price = 1.2345678912345;

        let ioi64 = Prec0::price_f64_to_i64(price)?;
        let reversed_price = Prec0::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_000_000_000);
        assert_eq!(reversed_price, 1.0);

        let ioi64 = Prec1::price_f64_to_i64(price)?;
        let reversed_price = Prec1::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_200_000_000);
        assert_eq!(reversed_price, 1.2);

        let ioi64 = Prec2::price_f64_to_i64(price)?;
        let reversed_price = Prec2::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_230_000_000);
        assert_eq!(reversed_price, 1.23);

        let ioi64 = Prec3::price_f64_to_i64(price)?;
        let reversed_price = Prec3::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_235_000_000);
        assert_eq!(reversed_price, 1.235);

        let ioi64 = Prec4::price_f64_to_i64(price)?;
        let reversed_price = Prec4::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_600_000);
        assert_eq!(reversed_price, 1.2346);

        let ioi64 = Prec5::price_f64_to_i64(price)?;
        let reversed_price = Prec5::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_570_000);
        assert_eq!(reversed_price, 1.23457);

        let ioi64 = Prec6::price_f64_to_i64(price)?;
        let reversed_price = Prec6::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_568_000);
        assert_eq!(reversed_price, 1.234568);

        let ioi64 = Prec7::price_f64_to_i64(price)?;
        let reversed_price = Prec7::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_567_900);
        assert_eq!(reversed_price, 1.2345679);

        let ioi64 = Prec8::price_f64_to_i64(price)?;
        let reversed_price = Prec8::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_567_890);
        assert_eq!(reversed_price, 1.23456789);

        let ioi64 = Prec9::price_f64_to_i64(price)?;
        let reversed_price = Prec9::price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 1_234_567_891);
        assert_eq!(reversed_price, 1.234567891);

        Ok(())
    }

    #[test]
    fn test_quantity_conversion_and_reversion() -> Result<()> {
        let quantity = 1.2345678912345;

        let iou64 = Prec0::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec0::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_000_000_000);
        assert_eq!(reversed_quantity, 1.0);

        let iou64 = Prec1::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec1::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_200_000_000);
        assert_eq!(reversed_quantity, 1.2);

        let iou64 = Prec2::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec2::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_230_000_000);
        assert_eq!(reversed_quantity, 1.23);

        let iou64 = Prec3::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec3::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_235_000_000);
        assert_eq!(reversed_quantity, 1.235);

        let iou64 = Prec4::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec4::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_234_600_000);
        assert_eq!(reversed_quantity, 1.2346);

        let iou64 = Prec5::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec5::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_234_570_000);
        assert_eq!(reversed_quantity, 1.23457);

        let iou64 = Prec6::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec6::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_234_568_000);
        assert_eq!(reversed_quantity, 1.234568);

        let iou64 = Prec7::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec7::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_234_567_900);
        assert_eq!(reversed_quantity, 1.2345679);

        let iou64 = Prec8::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec8::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_234_567_890);
        assert_eq!(reversed_quantity, 1.23456789);

        let iou64 = Prec9::quantity_f64_to_u64(quantity)?;
        let reversed_quantity = Prec9::quantity_u64_to_f64(iou64);
        assert_eq!(iou64, 1_234_567_891);
        assert_eq!(reversed_quantity, 1.234567891);

        Ok(())

    }
}

