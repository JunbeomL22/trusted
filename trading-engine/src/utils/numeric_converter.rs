use crate::types::base::{BookPrice, BookQuantity, OrderCount};
use crate::{log_error, log_warn};
use anyhow::{anyhow, bail, Result};
use serde::{de::Deserializer, Deserialize, Serialize};
use std::ptr::read_unaligned;

/// decimal_point_length includes the decimal point itself
/// ex) 123.45 => digit_length: 3, decimal_point_length: 3, total_length: 6
#[derive(
    Default, Debug, Clone, Serialize, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct NumReprCfg {
    pub digit_length: usize,
    pub decimal_point_length: usize,
    pub is_signed: bool,
    pub total_length: usize,
    pub float_normalizer: Option<i32>,
    pub drop_decimal_point: bool,
}

impl NumReprCfg {
    pub fn check_validity(&self) -> Result<()> {
        let mut check_size = self.digit_length + self.decimal_point_length;

        check_size += if self.is_signed { 1 } else { 0 };

        if check_size != self.total_length {
            let error = || {
                anyhow!(
                    "NumReprCfg::new => size check failed: \n\
                digit_length: {:?}\n\
                decimal_point_length: {:?}\n\
                is_signed: {:?}\n\
                total_length: {:?}\n\
                check_size: {:?}",
                    self.digit_length,
                    self.decimal_point_length,
                    self.is_signed,
                    self.total_length,
                    check_size
                )
            };
            return Err(error());
        }
        Ok(())
    }
}

#[inline(always)]
fn div_rem(dividend: i64, divisor: i64) -> (i64, i64) {
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);
    (quotient, remainder)
}

#[inline(always)]
pub fn u8_chunk_to_u64_decimal(mut chunk: u64) -> u64 {
    let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f) * 10;
    chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on 2 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff) * 100;
    chunk = lower_digits + upper_digits;

    // 4-byte mask trick (works on a pair of four digits)
    let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff) * 10000;
    chunk = lower_digits + upper_digits;

    chunk
}

#[inline(always)]
pub fn u8_chunk_to_u128_decimal(mut chunk: u128) -> u128 {
    let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
    chunk = lower_digits + upper_digits;
    // 2-byte mask trick (works on 4 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
    chunk = lower_digits + upper_digits;
    // 4-byte mask trick (works on 2 pairs of four digits)
    let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
    chunk = lower_digits + upper_digits;
    // 8-byte mask trick (works on a pair of eight digits)
    let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
    let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100_000_000;
    chunk = lower_digits + upper_digits;
    //
    chunk
}

#[inline(always)]
pub fn parse_under8_with_floating_point(u: &[u8], length: usize, point_length: usize) -> u64 {
    debug_assert!(
        (1..=8).contains(&length),
        "parse_under8: length must be less than or equal to 8"
    );
    match point_length {
        // ex) u = "12345"
        0 => parse_under8(u, length),
        // ex) u = "12345."
        1 => parse_under8(&u[..(length - 1)], length - 1),
        _ => {
            // ex) u = "123.45", length = 6, point_length = 3,
            // "123.45" => "??123.45"
            let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
            // "??123.45" => "123.4500"
            chunk <<= 64 - (length * 8);
            // "123.4500" => "12345000"

            let point_mask = 0xffff_ffff_ffff_ffff << ((9 - point_length) * 8);
            let decimal_mask = !point_mask;
            chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
            u8_chunk_to_u64_decimal(chunk)
        }
    }
}

#[inline(always)]
pub fn parse_under16_with_floating_point(u: &[u8], length: usize, point_length: usize) -> u64 {
    debug_assert!(
        (1..=16).contains(&length),
        "parse_under16: length must be less than or equal to 16"
    );
    match point_length {
        0 => parse_under16(u, length) as u64,
        1 => parse_under16(&u[..(length - 1)], length - 1) as u64,
        _ => {
            let mut chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
            chunk <<= 128 - (length * 8);
            let point_mask = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << ((17 - point_length) * 8);
            let decimal_mask = !point_mask;
            chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
            u8_chunk_to_u128_decimal(chunk) as u64
        }
    }
}

#[inline(always)]
pub fn parse_under32_with_floating_point(u: &[u8], length: usize, point_length: usize) -> u64 {
    debug_assert!(
        (16..=32).contains(&length),
        "parse_under32: length must be less than or equal to 32"
    );

    match point_length {
        0 => parse_under32(u, length) as u64,
        1 => parse_under32(&u[..(length - 1)], length - 1) as u64,
        _ => {
            let point_location = length - point_length;
            let left_point_length = if point_location > 16 {
                0
            } else {
                16 - point_location
            };
            let right_point_length = if point_location < 16 { 0 } else { point_length };
            //
            let (upper, lower) = u.split_at(16);

            let upper_val = parse_under16_with_floating_point(upper, 16, left_point_length);
            let lower_val =
                parse_under16_with_floating_point(lower, length - 16, right_point_length);

            let power_val = if point_location <= 15 {
                10u128.pow((length - 16) as u32)
            } else {
                10u128.pow((length - 17) as u32)
            };
            /*
            dbg!(
                std::str::from_utf8(upper).unwrap(),
                std::str::from_utf8(lower).unwrap(),
                upper_val,
                lower_val,
                point_location,
                length,
                left_point_length,
                right_point_length,
                power_val,
            );
             */
            upper_val * power_val as u64 + lower_val
        }
    }
}

#[inline(always)]
pub fn parse_under8(u: &[u8], length: usize) -> u64 {
    debug_assert!(
        length <= 8,
        "parse_under8: length must be less than or equal to 8"
    );
    let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
    chunk <<= 64 - (length * 8);
    u8_chunk_to_u64_decimal(chunk)
}

#[inline(always)]
pub fn parse_under16(u: &[u8], length: usize) -> u128 {
    debug_assert!(
        length <= 16,
        "parse_under16: length must be less than or equal to 16"
    );
    let mut chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
    chunk <<= 128 - (length * 8);
    u8_chunk_to_u128_decimal(chunk)
}

#[inline(always)]
pub fn parse_under32(u: &[u8], length: usize) -> u128 {
    debug_assert!(
        (16..=32).contains(&length),
        "parse_under32: length must be less than or equal to 32"
    );
    let (upper, lower) = u.split_at(16);
    parse_under16(upper, 16) * 10u128.pow((length - 16) as u32) + parse_under16(lower, length - 16)
}

#[derive(Debug, Serialize)]
pub struct IntegerConverter {
    numcfg: NumReprCfg,
    is_signed: bool,
    start_index: usize,
    end_index: usize,
    decimal_point_length: usize,
    decimal_point_length_i32: i32,
    parsing_length: usize,
    #[serde(skip)]
    parser: fn(&[u8], usize, usize) -> u64,
}

impl<'de> Deserialize<'de> for IntegerConverter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Step 1: Deserialize NumReprCfg
        let numcfg = NumReprCfg::deserialize(deserializer)?;
        // Step 2: Call IntegerConverter::new with the deserialized NumReprCfg
        IntegerConverter::new(numcfg).map_err(serde::de::Error::custom)
    }
}

impl Default for IntegerConverter {
    fn default() -> IntegerConverter {
        IntegerConverter {
            numcfg: NumReprCfg::default(),
            is_signed: false,
            start_index: 0,
            end_index: 0,
            decimal_point_length: 0,
            decimal_point_length_i32: 0,
            parsing_length: 0,
            parser: parse_under16_with_floating_point,
        }
    }
}

impl Clone for IntegerConverter {
    fn clone(&self) -> Self {
        IntegerConverter::new(self.numcfg).unwrap()
    }
}

impl IntegerConverter {
    pub fn get_config(&self) -> &NumReprCfg {
        &self.numcfg
    }
    pub fn new(numcfg: NumReprCfg) -> Result<IntegerConverter> {
        numcfg.check_validity()?;
        if !cfg!(target_endian = "little") {
            bail!("IntegerConverter parse by bit operations based only on little endian")
        }

        let mut all_digit_size = numcfg.total_length;

        let start_index = if numcfg.is_signed {
            all_digit_size -= 1;
            1
        } else {
            0
        };

        let decimal_point_length = if numcfg.drop_decimal_point {
            all_digit_size -= numcfg.decimal_point_length;
            0
        } else {
            numcfg.decimal_point_length
        };

        let end_index = all_digit_size;

        let parser = match all_digit_size {
            0..=8 => parse_under8_with_floating_point,
            9..=16 => parse_under16_with_floating_point,
            17..=18 => {
                log_warn!(
                    "long digit",
                    number_config = numcfg,
                    all_digit_size = all_digit_size
                );
                parse_under32_with_floating_point
            }
            _ => {
                log_error!(
                    "unsupported digit size", 
                    number_config = numcfg,
                    digit_size = all_digit_size,
                    message = "number show digit is larger than 18 digits is over the capacity of u64 type",
                );
                bail!("unsupported digit size");
            }
        };

        Ok(IntegerConverter {
            numcfg,
            is_signed: numcfg.is_signed,
            start_index,
            end_index,
            decimal_point_length,
            decimal_point_length_i32: decimal_point_length as i32,
            parsing_length: all_digit_size,
            parser,
        })
    }

    #[inline(always)]
    pub fn to_u64(&self, value: &[u8]) -> u64 {
        (self.parser)(
            &value[self.start_index..self.end_index],
            self.parsing_length,
            self.decimal_point_length,
        )
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline(always)]
    pub unsafe fn to_u64_unchecked(&self, value: &[u8]) -> u64 {
        parse_under16(value, self.parsing_length) as u64
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline(always)]
    pub unsafe fn to_u32_unchecked(&self, value: &[u8]) -> u32 {
        parse_under8(value, self.parsing_length) as u32
    }

    #[inline(always)]
    pub fn to_i64(&self, value: &[u8]) -> i64 {
        if self.is_signed {
            match value[0] == b'0' {
                false => (!self.to_u64(value)).wrapping_add(1) as i64,
                _ => self.to_u64(value) as i64,
            }
        } else {
            self.to_u64(value) as i64
        }
    }

    #[inline(always)]
    pub fn to_f64_from_i64(&self, value: i64) -> f64 {
        value as f64 / 10_f64.powi(self.decimal_point_length_i32)
    }

    #[inline(always)]
    pub fn to_f64_from_i32(&mut self, value: i32) -> f64 {
        value as f64 / 10_f64.powi(self.decimal_point_length_i32)
    }

    #[inline(always)]
    pub fn to_f64_from_u32(&mut self, value: u32) -> f64 {
        value as f64 / 10_f64.powi(self.decimal_point_length_i32)
    }

    #[inline(always)]
    pub fn normalized_f64_from_i64(&mut self, value: i64) -> f64 {
        match self.numcfg.float_normalizer {
            Some(normalizer) => {
                let added_normalizer = normalizer + self.decimal_point_length_i32;
                let denominator = 10_f64.powi(added_normalizer);
                let (quotient, remainder) = div_rem(value, 10_i64.pow(added_normalizer as u32));

                quotient as f64 + (remainder as f64 / denominator)
            }
            None => self.to_f64_from_i64(value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderConverter {
    pub price: IntegerConverter,
    pub quantity: IntegerConverter,
    pub order_count: IntegerConverter,
}

impl OrderConverter {
    #[inline]
    pub fn to_book_price(&self, val: &[u8]) -> BookPrice {
        self.price.to_i64(val)
    }

    #[inline]
    pub fn to_book_quantity(&self, val: &[u8]) -> BookQuantity {
        self.quantity.to_u64(val)
    }

    #[inline]
    pub fn to_order_count(&self, val: &[u8]) -> OrderCount {
        assert!(val.len() <= 10, "OrderCount must be fewer than 10 bytes");
        self.order_count.to_u64(val) as u32
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_order_count_unchecked(&self, val: &[u8]) -> OrderCount {
        self.order_count.to_u32_unchecked(val)
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_book_quantity_unchecked(&self, val: &[u8]) -> BookQuantity {
        self.quantity.to_u64_unchecked(val)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeStampConverter {
    pub converter: IntegerConverter,
}

impl TimeStampConverter {
    #[inline]
    pub fn to_timestamp(&self, val: &[u8]) -> u64 {
        self.converter.to_u64(val)
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_timestamp_unchecked(&self, val: &[u8]) -> u64 {
        self.converter.to_u64_unchecked(val)
    }
}

//unsafe impl Sync for OrderConverter {}

//unsafe impl Sync for TimeStampConverter {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chars_parser() {
        let s = b"1";
        let val = parse_under8_with_floating_point(s, 1, 0);
        assert_eq!(val, 1);
        let val = parse_under16_with_floating_point(s, 1, 0);
        assert_eq!(val, 1);

        let s = b".123456";
        let val = parse_under8_with_floating_point(s, 7, 7);
        assert_eq!(val, 123456);
        let val = parse_under16_with_floating_point(s, 7, 7);
        assert_eq!(val, 123456);

        let s = b"123456.";
        let val = parse_under8_with_floating_point(s, 7, 1);
        assert_eq!(val, 123456);
        let val = parse_under16_with_floating_point(s, 7, 1);
        assert_eq!(val, 123456);

        let s = b".1234567";
        let val = parse_under8_with_floating_point(s, 8, 8);
        assert_eq!(val, 1234567);
        let val = parse_under16_with_floating_point(s, 8, 8);
        assert_eq!(val, 1234567);

        let s = b"1234567.";
        let val = parse_under8_with_floating_point(s, 8, 1);
        assert_eq!(val, 1234567);
        let val = parse_under16_with_floating_point(s, 8, 1);
        assert_eq!(val, 1234567);

        let s = b"12345678";
        let val = parse_under8_with_floating_point(s, 8, 0);
        assert_eq!(val, 12345678);
        let val = parse_under16_with_floating_point(s, 8, 0);
        assert_eq!(val, 12345678);

        let s = b".12345678901234567";
        let val = parse_under32_with_floating_point(s, 18, 18);
        assert_eq!(val, 12345678901234567);

        let s = b"12345678901234567.";
        let val = parse_under32_with_floating_point(s, 18, 1);
        assert_eq!(val, 12345678901234567);

        let s = b"123456789012345678.";
        let val = parse_under32_with_floating_point(s, 19, 1);
        assert_eq!(val, 123456789012345678);

        let s = b"1234567890123456789";
        let val = parse_under32_with_floating_point(s, 19, 0);
        assert_eq!(val, 1234567890123456789);

        let s = b"123456789012345.678";
        let val = parse_under32_with_floating_point(s, 19, 4);
        assert_eq!(val, 123456789012345678);

        let s = b"1234567890123456.78";
        let val = parse_under32_with_floating_point(s, 19, 3);
        assert_eq!(val, 123456789012345678);

        let s = b"12345678901234567.8";
        let val = parse_under32_with_floating_point(s, 19, 2);
        assert_eq!(val, 123456789012345678);

        let s = b"00001234.5";
        let val = parse_under16_with_floating_point(s, 10, 2);
        assert_eq!(val, 12345);

        let s = b"000012345.6789012345";
        let val = parse_under32_with_floating_point(s, 20, 11);
        assert_eq!(val, 123456789012345);
    }
    #[test]
    fn test_parse_small_number() {
        let cfg = NumReprCfg {
            digit_length: 4,
            decimal_point_length: 2,
            drop_decimal_point: false,
            is_signed: true,
            total_length: 7,
            float_normalizer: None,
        };
        let converter = IntegerConverter::new(cfg).unwrap();
        let s = b"01234.5";
        let val = converter.to_u64(s);
        assert_eq!(
            val,
            12_345,
            "convert failed: {:?}",
            std::str::from_utf8(s).unwrap(),
        );
    }

    #[test]
    fn test_integer_converter() -> Result<()> {
        let cfg = NumReprCfg {
            digit_length: 8,
            decimal_point_length: 3,
            drop_decimal_point: false,
            is_signed: true,
            total_length: 12,
            float_normalizer: None,
        };
        let converter = IntegerConverter::new(cfg).unwrap();
        let s = b"000001234.56";
        let val = converter.to_u64(s);
        assert_eq!(val, 123_456);

        let s = b"-10001234.56";
        let val_i64 = converter.to_i64(s);
        assert_eq!(val_i64, -1_000_123_456);

        let cfg = NumReprCfg {
            digit_length: 11,
            decimal_point_length: 0,
            drop_decimal_point: false,
            is_signed: false,
            total_length: 11,
            float_normalizer: None,
        };

        let converter = IntegerConverter::new(cfg).unwrap();
        let val_str = b"10000123456";
        let val = converter.to_u64(val_str);

        assert_eq!(val, 10_000_123_456);

        let cfg_for_big_number = NumReprCfg {
            digit_length: 15,
            decimal_point_length: 2,
            drop_decimal_point: false,
            is_signed: true,
            total_length: 18,
            float_normalizer: None,
        };

        let converter = IntegerConverter::new(cfg_for_big_number).unwrap();

        let val_str = b"-911110000123456.3";

        let val_i64 = converter.to_i64(val_str);
        assert_eq!(val_i64, -9_111_100_001_234_563);

        let val_str = b"-091110000123456.3";
        let val_i64 = converter.to_i64(val_str);
        assert_eq!(val_i64, -911_100_001_234_563);

        Ok(())
    }

    #[test]
    fn drop_decimal_point() -> Result<()> {
        let cfg = NumReprCfg {
            digit_length: 11,
            decimal_point_length: 4,
            drop_decimal_point: true,
            is_signed: true,
            total_length: 16,
            float_normalizer: None,
        };

        let converter = IntegerConverter::new(cfg).unwrap();

        let val_str = b"-10000123456.001";
        let val = converter.to_i64(val_str);

        assert_eq!(val, -10_000_123_456);

        Ok(())
    }
}
