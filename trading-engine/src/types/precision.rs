use anyhow::{Result, anyhow};
use enum_dispatch::enum_dispatch;

// zero system constant
const MAX_IO_MULTIPLIER_ZERO_SYSTEM: f64 = 1.0; // 10.0 ** ZERO
const PRICE_MAX_ZERO_SYSTEM: f64 = 9_223_372_036_000_000_000.0; // i64::MAX
const PRICE_MIN_ZERO_SYSTEM: f64 = -9_223_372_036_000_000_000.0; // i64::MIN
const QUANTITY_MAX_ZERO_SYSTEM: f64 = 18_446_744_072_000_000_000.0; // u64::MAX
const QUANTITY_MIN_ZERO_SYSTEM: f64 = 0.0;
// three system constant
const MAX_IO_MULTIPLIER_THREE_SYSTEM: f64 = 1_000.0; // 10.0 ** THREE
const PRICE_MAX_THREE_SYSTEM: f64 = 9_223_372_036_000_000.0; // i64::MAX / 1_000
const PRICE_MIN_THREE_SYSTEM: f64 = -9_223_372_036_000_000.0; // i64::MIN / 1_000
const QUANTITY_MAX_THREE_SYSTEM: f64 = 18_446_744_072_000_000.0; // u64::MAX / 1_000
const QUANTITY_MIN_THREE_SYSTEM: f64 = 0.0;
// six system constant
const MAX_IO_MULTIPLIER_SIX_SYSTEM: f64 = 1_000_000.0; // 10.0 ** SIX
const PRICE_MAX_SIX_SYSTEM: f64 = 9_223_372_036_000.0; // i64::MAX / 1_000_000
const PRICE_MIN_SIX_SYSTEM: f64 = -9_223_372_036_000.0; // i64::MIN / 1_000_000
const QUANTITY_MAX_SIX_SYSTEM: f64 = 18_446_744_072_000.0; // u64::MAX / 1_000_000
const QUANTITY_MIN_SIX_SYSTEM: f64 = 0.0;
// nine system constant
const MAX_IO_MULTIPLIER_NINE_SYSTEM: f64 = 1_000_000_000.0; // 10.0**NINE
const PRICE_MAX_NINE_SYSTEM: f64 = 9_223_372_036.0; // i64::MAX / 1_000_000_000
const PRICE_MIN_NINE_SYSTEM: f64 = -9_223_372_036.0; // i64::MIN / 1_000_000_000
const QUANTITY_MAX_NINE_SYSTEM: f64 = 18_446_744_073.0; // u64::MAX / 1_000_000_000
const QUANTITY_MIN_NINE_SYSTEM: f64 = 0.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// book price creation hendler
/// in the order of frequency of use
/// Preci_j means for number having i precisions where nominated by 9 precision in mind
/// ex1) Prec2_9: this stores a number with 2 precisions. So, it removes 7 precisions from 9 precision after str -> float
/// then, it multiply 10^7 to make it integer, and store it as i64
/// when it handles the actual feature calculation the book price and quantity, it will be converted to float again and must be nominated
pub enum PrecisionHandler {
    #[default]
    // the following precisions are based on 9 max precision
    Prec0_3, // all quantity in KRX, stock price
    Prec2_3, // fx futures, index futures, bond in KRX
    Prec3_3, // repo, 3M risk-free futures
    Prec6_6, // bond yield in KRX
    Prec9_9, // maybe coin? 
}

impl PrecisionHandler {
    #[inline]
    #[must_use]
    pub fn precision(&self) -> u8 {
        match self {
            Precision::Prec0_3 => 0,
            Precision::Prec2_3 => 2,
            Precision::Prec3_3 => 3,
            Precision::Prec6_6 => 6,
            Precision::Prec9_9 => 9,
        }
    }

    #[inline]
    #[must_use]
    pub fn max_precision(&self) -> u8 {
        match self {
            Precision::Prec0_3 => 3,
            Precision::Prec2_3 => 3,
            Precision::Prec3_3 => 3,
            Precision::Prec6_6 => 6,
            Precision::Prec9_9 => 9,
        }
    }

    #[inline]
    #[must_use]
    pub fn check_f64price_bound(&self, price: f64) -> bool {
        match self {
            Precision::Prec0_3 | Precision::Prec2_3 | Precision::Prec3_3 => (PRICE_MIN_THREE_SYSTEM..=PRICE_MAX_THREE_SYSTEM).contains(&price),
            Precision::Prec6_6 => (PRICE_MIN_SIX_SYSTEM..=PRICE_MAX_SIX_SYSTEM).contains(&price),
            Precision::Prec9_9 => (PRICE_MIN_NINE_SYSTEM..=PRICE_MAX_NINE_SYSTEM).contains(&price),
        }
    }

    #[inline]
    #[must_use]
    pub fn check_f64quantity_bound(&self, quantity: f64) -> bool {
        match self {
            Precision::Prec0_3 | Precision::Prec2_3 | Precision::Prec3_3 => (QUANTITY_MIN_THREE_SYSTEM..=QUANTITY_MAX_THREE_SYSTEM).contains(&quantity),
            Precision::Prec6_6 => (QUANTITY_MIN_SIX_SYSTEM..=QUANTITY_MAX_SIX_SYSTEM).contains(&quantity),
            Precision::Prec9_9 => (QUANTITY_MIN_NINE_SYSTEM..=QUANTITY_MAX_NINE_SYSTEM).contains(&quantity),
        }
    }

    #[inline]
    #[must_use]
    pub fn price_f64_to_i64(&self, value: f64) -> Result<i64> {
        if !self.check_f64price_bound(value) {
            let error = || anyhow!("price: {price} out of bound (called from {precision})", price = value, precision = self.precision());
            return Err(error());
        }
        
        match self {
            Precision::Prec0_3 => Ok((value.round() as i64) * 1_000_i64),
            Precision::Prec2_3 => Ok((value * 100.0).round() as i64 * 10_i64),
            Precision::Prec3_3 => Ok((value * 1_000.0).round() as i64),
            Precision::Prec6_6 => Ok((value * 1_000_000.0).round() as i64),
            Precision::Prec9_9 => Ok((value * 1_000_000_000.0).round() as i64),
        }
    }

    #[inline]
    #[must_use]
    pub fn price_i64_to_f64(&self, value: i64) -> f64 {
        match self {
            Precision::Prec0_3 | Precision::Prec2_3 | Precision::Prec3_3 => value as f64 / MAX_IO_MULTIPLIER_THREE_SYSTEM,
            Precision::Prec6_6 => value as f64 / MAX_IO_MULTIPLIER_SIX_SYSTEM,
            Precision::Prec9_9 => value as f64 / MAX_IO_MULTIPLIER_NINE_SYSTEM,
        }
    }

    #[inline]
    #[must_use]
    pub fn quantity_f64_to_u64(&self, value: f64) -> Result<u64> {
        if !self.check_f64quantity_bound(value) {
            let error = || anyhow!("quantity: {qnt} out of bound (called from {precision})", qnt = value, precision = self.precision());
            return Err(error());
        }

        match self {
            Precision::Prec0_3 => Ok((value.round() as u64) * 1_000_u64),
            Precision::Prec2_3 => Ok((value * 100.0).round() as u64 * 10_u64),
            Precision::Prec3_3 => Ok((value * 1_000.0).round() as u64),
            Precision::Prec6_6 => Ok((value * 1_000_000.0).round() as u64),
            Precision::Prec9_9 => Ok((value * 1_000_000_000.0).round() as u64),
        }
    }

    #[inline]
    #[must_use]
    pub fn quantity_u64_to_f64(&self, value: u64) -> f64 {
        match self {
            Precision::Prec0_3 | Precision::Prec2_3 | Precision::Prec3_3 => value as f64 / MAX_IO_MULTIPLIER_THREE_SYSTEM,
            Precision::Prec6_6 => value as f64 / MAX_IO_MULTIPLIER_SIX_SYSTEM,
            Precision::Prec9_9 => value as f64 / MAX_IO_MULTIPLIER_NINE_SYSTEM,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision() -> Result<()> {
        assert_eq!(Precision::Prec0_3.precision(), 0);
        assert_eq!(Precision::Prec0_3.max_precision(), 3);

        assert_eq!(Precision::Prec2_3.precision(), 2);
        assert_eq!(Precision::Prec2_3.max_precision(), 3);

        assert_eq!(Precision::Prec6_6.precision(), 6);
        assert_eq!(Precision::Prec6_6.max_precision(), 6);

        assert_eq!(Precision::Prec9_9.precision(), 9);
        assert_eq!(Precision::Prec9_9.max_precision(), 9);

        Ok(())
    }

    #[test]
    fn test_price_conversion_and_reversion() -> Result<()> {
        let price = 8_243_456_109_832.235_567_891_234_5;

        let prec0 = Precision::Prec0_3;
        let ioi64 = prec0.price_f64_to_i64(price)?;
        let reversed_price = prec0.price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_000);
        assert_eq!(reversed_price, 8_243_456_109_832.0);

        let prec2 = Precision::Prec2_3;
        let ioi64 = prec2.price_f64_to_i64(price)?;
        let reversed_price = prec2.price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_240);
        assert_eq!(reversed_price, 8_243_456_109_832.24);

        let prec3 = Precision::Prec3_3;
        let ioi64 = prec3.price_f64_to_i64(price)?;
        let reversed_price = prec3.price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_236);
        assert_eq!(reversed_price, 8_243_456_109_832.236);

        let prec6 = Precision::Prec6_6;
        let ioi64 = prec6.price_f64_to_i64(price)?;
        let reversed_price = prec6.price_i64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_235_568);
        assert_eq!(reversed_price, 8_243_456_109_832.235_568);

        Ok(())
    }

    #[test]
    fn test_quantity_conversion_and_reversion() -> Result<()> {
        let quantity = 8_243_456_109_832.235_567_891_234_5;

        let prec0 = Precision::Prec0_3;
        let ioi64 = prec0.quantity_f64_to_u64(quantity)?;
        let reversed_quantity = prec0.quantity_u64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_000);
        assert_eq!(reversed_quantity, 8_243_456_109_832.0);

        let prec2 = Precision::Prec2_3;
        let ioi64 = prec2.quantity_f64_to_u64(quantity)?;
        let reversed_quantity = prec2.quantity_u64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_240);
        assert_eq!(reversed_quantity, 8_243_456_109_832.24);

        let prec3 = Precision::Prec3_3;
        let ioi64 = prec3.quantity_f64_to_u64(quantity)?;
        let reversed_quantity = prec3.quantity_u64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_236);
        assert_eq!(reversed_quantity, 8_243_456_109_832.236);

        let prec6 = Precision::Prec6_6;
        let ioi64 = prec6.quantity_f64_to_u64(quantity)?;
        let reversed_quantity = prec6.quantity_u64_to_f64(ioi64);
        assert_eq!(ioi64, 8_243_456_109_832_235_568);
        assert_eq!(reversed_quantity, 8_243_456_109_832.235_568);
    
        Ok(())
    }
}

