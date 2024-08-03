use crate::types::base::{
    BookPrice, 
    BookQuantity, 
    OrderCount,
    BookYield,
};
use crate::get_unix_nano;
use crate::topics::LogTopic;
use crate::types::base::{
    Real,
    NormalizedReal,
};

use crate::types::timestamp::{
    TimeStamp,
    DateUnixNanoGenerator,
    SECOND_NANOSCALE,
    DAY_NANOSCALE,
    FIFTEEN_HOURS_NANOSCALE,
};
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
    pub unused_length: usize,
    pub total_length: usize,
    pub float_normalizer: Option<i32>,
    pub drop_decimal_point: bool,
}

impl NumReprCfg {
    pub fn check_validity(&self) -> Result<()> {
        let mut check_size = self.digit_length + self.decimal_point_length + self.unused_length;

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
fn div_rem_u64(u: u64, v: u64) -> (u64, u64) {
    let quotient = u / v;
    let remainder = u - (quotient * v);
    (quotient, remainder)
}

#[inline(always)]
pub fn parse_hhmmss_to_nanos(u: &[u8]) -> u64 {
    // "123456" => [?, ?, "6", "5", "4", "3", "2", "1"]
    let mut seconds: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
    // [?, ?, "6", "5", "4", "3", "2", "1"] => ["6", "5", "4", "3", "2", "1", 0, 0] = b"00123456"
    seconds <<= 16;
    // lower_digits = [6, 0, 4, 0, 2, 0, 0, 0] => [0, 6, 0, 4, 0, 2, 0, 0]
    let lower_digits = (seconds & 0x0f000f000f000f00) >> 8;
    // upper_digits = [0, 5, 0, 3, 0, 1, 0, 0] => [0, 50, 0, 30, 0, 10, 0, 0]
    let upper_digits = (seconds & 0x000f000f000f000f) * 10;
    // seconds = [0, 56, 0, 34, 0, 12, 0, 0]
    seconds = lower_digits + upper_digits;

    // lower_digits = [0, 0, 0, 56, 0, 0, 0, 12]
    let lower_digits = (seconds & 0x00ff000000ff0000) >> 16;
    // upper_digits = [0, 0, 34, 0, 0, 0, 0, 0] => [0, 0, 34 * 60, 0, 0, 0, 0, 0]
    let upper_digits = (seconds & 0x000000ff000000ff) * 60;
    // seconds = [0, 0, 34 * 60, 56, 0, 0, 0, 12]
    seconds = lower_digits + upper_digits;

    // lower_digits = [0, 0, 0, 0, 0, 0, 34 * 60, 56]
    let lower_digits = (seconds & 0x0000ffff00000000) >> 32;
    // upper_digits = [0, 0, 0, 0, 0, 0, 0, 12 * 3600]
    let upper_digits = (seconds & 0x000000000000ffff) * 3_600;
    seconds = lower_digits + upper_digits;

    seconds * SECOND_NANOSCALE

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
            
            upper_val * power_val as u64 + lower_val
        }
    }
}
#[inline(always)]
pub fn parse_2digits(u: &[u8]) -> u16 {
    let chunk: u16 = unsafe { read_unaligned(u.as_ptr() as *const u16) };
    let lower = (chunk & 0x0f00) >> 8;
    let upper = (chunk & 0x000f) * 10;
    lower + upper
}

#[inline(always)]
pub fn parse_3digits(u: &[u8]) -> u32 {
    let mut chunk: u32 = unsafe { read_unaligned(u.as_ptr() as *const u32) };
    chunk <<= 8;

    let lower: u32 = (chunk & 0x0f000f00) >> 8; 
    // [0x04, 0x00, 0x02, 0x00] >> 8 = [0x00, 0x04, 0x00, 0x02]
    let upper: u32 = (chunk & 0x000f000f) * 10; 
    // [0x00, 0x03, 0x00, 0x01] * 10 =  [00, 30, 00, 10] (formally, not rigorous bit representation)
    chunk = lower + upper;
    // [00, 34, 00, 12]
    let lower: u32 = (chunk & 0x00ff0000) >> 16; 
    // [00, 34, 00, 12] >> 16 = [00, 00, 00, 34] = 34
    let upper: u32 = (chunk & 0x000000ff) * 100; 
    //   [00, 34, 00, 12] 
    //   &
    //   [00, 00, 00, ff]
    // = [00, 00, 00, 12] => *100 => 1200
    lower + upper // 34 + 1200
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
    decimal_number_point_length_i32: i32,
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
            decimal_number_point_length_i32: 0,
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

        let is_sigined_usize = numcfg.is_signed as usize;
        let mut all_digit_size = numcfg.total_length - numcfg.unused_length - is_sigined_usize;

        let start_index = is_sigined_usize + numcfg.unused_length;

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
                parse_under32_with_floating_point
            }
            _ => {
                crate::log_error!(
                    LogTopic::UnsupportedDigitSize.as_str(),
                    struct_info = numcfg.clone(),
                    message = "number over 18 digits can't be handled by u64"
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
            decimal_number_point_length_i32: (decimal_point_length as i32 - 1i32).max(0),
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
    /// This function is the input is non-negative integer
    #[inline(always)]
    pub unsafe fn to_nonnegative_i64_unchecked(&self, value: &[u8]) -> i64 {
        self.to_u64_unchecked(value) as i64
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
    pub unsafe fn to_long_u64_unchecked(&self, value: &[u8]) -> u64 {
        parse_under32(value, self.parsing_length) as u64
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
        value as f64 / 10_f64.powi(self.decimal_number_point_length_i32)
    }

    #[inline(always)]
    pub fn to_f64_from_u64(&self, value: u64) -> f64 {
        value as f64 / 10_f64.powi(self.decimal_number_point_length_i32)
    }

    #[inline(always)]
    pub fn to_f64_from_i32(&self, value: i32) -> f64 {
        value as f64 / 10_f64.powi(self.decimal_number_point_length_i32)
    }

    #[inline(always)]
    pub fn to_f64_from_u32(&self, value: u32) -> f64 {
        value as f64 / 10_f64.powi(self.decimal_number_point_length_i32)
    }

    #[inline(always)]
    pub fn normalized_real_from_i64(&self, value: i64) -> NormalizedReal {
        match self.numcfg.float_normalizer {
            Some(normalizer) => {
                let added_normalizer = normalizer + self.decimal_number_point_length_i32;
                let denominator = (10.0 as Real).powi(added_normalizer);
                let (quotient, remainder) = div_rem(value, 10_i64.pow(added_normalizer as u32));

                quotient as Real + (remainder as Real / denominator)
            }
            None => self.to_f64_from_i64(value) as Real
        }
    }

    #[inline(always)]
    pub fn normalized_real_from_u64(&self, value: u64) -> NormalizedReal {
        match self.numcfg.float_normalizer {
            Some(normalizer) => {
                let added_normalizer = normalizer + self.decimal_number_point_length_i32;
                let denominator = 10_f64.powi(added_normalizer);
                let (quotient, remainder) = div_rem_u64(value, 10_u64.pow(added_normalizer as u32));

                quotient as Real + (remainder as Real / denominator as Real)
            }
            None => self.to_f64_from_u64(value) as Real,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OrderCounter {
    pub order_count: IntegerConverter,
}

impl OrderCounter {
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OrderConverter {
    pub price: IntegerConverter,
    pub quantity: IntegerConverter,
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

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_book_price_unchecked(&self, val: &[u8]) -> BookPrice {
        self.price.to_nonnegative_i64_unchecked(val)
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_book_quantity_unchecked(&self, val: &[u8]) -> BookQuantity {
        self.quantity.to_u64_unchecked(val)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TimeStampConverter {
    pub converter: IntegerConverter,
    offset_nanos: u64, // offset in nano seconds. e.g., in Seoul, it is 9 * 3600 * 1_000_000_000
}

impl TimeStampConverter {
    pub fn new(converter: IntegerConverter, offset_nanos: u64) -> TimeStampConverter {
        TimeStampConverter {
            converter,
            offset_nanos,
        }
    }

    #[inline]
    pub fn parse_hhmmssuuuuuu(
        &self, 
        val: &[u8],
        system_timestamp: Option<TimeStamp>,
        date_generator: &mut DateUnixNanoGenerator,
    ) -> Result<TimeStamp> {
        let mut current_timestamp = parse_under8(&val[6..12], 6) * 1000;
        current_timestamp += parse_hhmmss_to_nanos(&val[0..6]);
        current_timestamp = current_timestamp + self.offset_nanos + date_generator.utcdate_unix_nano;

        let prev_timestamp = date_generator.prev_timestamp;

        while prev_timestamp >= FIFTEEN_HOURS_NANOSCALE + current_timestamp { // 1 day elapsed
            crate::log_info!(
                LogTopic::DateInUtcShift.as_str(),
                message = "timestampsinsec drops more than 70,000 secs, so date generator set to today",
                struct_info = "struct1: prev_timestamp, struct2: current_timestamp",
                struct1 = TimeStamp { stamp: prev_timestamp },
                struct2 = TimeStamp { stamp: current_timestamp },
            );
            if let Some(sys) = system_timestamp {
                date_generator.increment(sys.stamp);
            } else {
                date_generator.increment(get_unix_nano());
            }
            
            current_timestamp += DAY_NANOSCALE;
        } 

        date_generator.prev_timestamp = current_timestamp;
        
        Ok(TimeStamp { stamp: current_timestamp })
    }

    /*
    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_timestamp_unchecked(&self, val: &[u8]) -> u64 {
        self.converter.to_u64_unchecked(val)
    }
    */
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YieldConverter {
    pub converter: IntegerConverter,
}

impl YieldConverter {
    #[inline]
    pub fn to_yield(&self, val: &[u8]) -> BookYield {
        self.converter.to_i64(val)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CumQntConverter {
    pub converter: IntegerConverter,
}

impl CumQntConverter {
    #[inline]
    pub fn to_cum_qnt(&self, val: &[u8]) -> BookQuantity {
        self.converter.to_u64(val)
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_cum_qnt_unchecked(&self, val: &[u8]) -> BookQuantity {
        self.converter.to_u64_unchecked(val)
    }

    /// # Safety
    /// This function is unsafe because it does not check the input format.
    #[inline]
    pub unsafe fn to_long_cum_qnt_unchecked(&self, val: &[u8]) -> BookQuantity {
        self.converter.to_long_u64_unchecked(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::krx::krx_converter::get_krx_timestamp_converter;

    use crate::types::timestamp::{
        MICRO_NANOSCALE,
        MILLI_NANOSCALE,
        SECOND_NANOSCALE,
        MINUTE_NANOSCALE,   
        HOUR_NANOSCALE,
    };

    #[test]
    fn test_timestamp_converter() -> Result<()> {
        let timestamp_converter = get_krx_timestamp_converter();
        let dt = time::macros::date!(2023-12-28);
        
        let mut date_generator = DateUnixNanoGenerator::from(dt);
        let first_date_gen_nano = date_generator.utcdate_unix_nano;
        
        let timestamp_bytes = b"163020300111";
        
        let timestamp = timestamp_converter.parse_hhmmssuuuuuu(
            timestamp_bytes, 
            Some(TimeStamp { stamp: crate::get_unix_nano() }),
            &mut date_generator
        ).unwrap();
        assert_eq!(first_date_gen_nano, date_generator.utcdate_unix_nano);
        
        let mut expected_nano = date_generator.utcdate_unix_nano;
        expected_nano += 16*HOUR_NANOSCALE;
        expected_nano += 30*MINUTE_NANOSCALE;
        expected_nano += 20*SECOND_NANOSCALE;
        expected_nano += 300*MILLI_NANOSCALE;
        expected_nano += 111*MICRO_NANOSCALE;
        //
        expected_nano += timestamp_converter.offset_nanos;

        dbg!(date_generator);
        assert_eq!(timestamp.stamp, expected_nano);
        assert_eq!(date_generator.utcdate_unix_nano, first_date_gen_nano);
        
        let timestamp_bytes = b"003020300111";
        let timestamp = timestamp_converter.parse_hhmmssuuuuuu(
            timestamp_bytes, 
            Some(TimeStamp { stamp: crate::get_unix_nano() }),
            &mut date_generator
        ).unwrap();
        
        let mut expected_nano = date_generator.utcdate_unix_nano;
        expected_nano += 30 * MINUTE_NANOSCALE;
        expected_nano += 20 * SECOND_NANOSCALE;
        expected_nano += 300 * MILLI_NANOSCALE;
        expected_nano += 111 * MICRO_NANOSCALE;
        //
        expected_nano += 9 * HOUR_NANOSCALE;

        dbg!(date_generator);
        assert_eq!(timestamp.stamp, expected_nano);
        assert_eq!(first_date_gen_nano + DAY_NANOSCALE, date_generator.utcdate_unix_nano);
    
        let timestamp_bytes = b"183020300111";
        let timestamp = timestamp_converter.parse_hhmmssuuuuuu(
            timestamp_bytes, 
            Some(TimeStamp { stamp: crate::get_unix_nano() }),
            &mut date_generator
        ).unwrap();

        
        let mut expected_nano = date_generator.utcdate_unix_nano;
        expected_nano += 18 * HOUR_NANOSCALE;
        expected_nano += 30 * MINUTE_NANOSCALE;
        expected_nano += 20 * SECOND_NANOSCALE;
        expected_nano += 300 * MILLI_NANOSCALE;
        expected_nano += 111 * MICRO_NANOSCALE;
        //
        expected_nano += 9 * HOUR_NANOSCALE;
        //
        dbg!(date_generator);
        assert_eq!(timestamp.stamp, expected_nano);
        assert_eq!(first_date_gen_nano + DAY_NANOSCALE, date_generator.utcdate_unix_nano);
        //
        let timestamp_bytes = b"023020300111";
        let timestamp = timestamp_converter.parse_hhmmssuuuuuu(
            timestamp_bytes, 
            Some(TimeStamp { stamp: crate::get_unix_nano() }),
            &mut date_generator
        ).unwrap();

        let mut expected_nano = date_generator.utcdate_unix_nano;
        expected_nano += 2 * HOUR_NANOSCALE;
        expected_nano += 30 * MINUTE_NANOSCALE;
        expected_nano += 20 * SECOND_NANOSCALE;
        expected_nano += 300 * MILLI_NANOSCALE;
        expected_nano += 111 * MICRO_NANOSCALE;
        //
        expected_nano += 9 * HOUR_NANOSCALE;
        //
        dbg!(date_generator);
        assert_eq!(timestamp.stamp, expected_nano);
        assert_eq!(first_date_gen_nano + 2*DAY_NANOSCALE, date_generator.utcdate_unix_nano);

        Ok(())
    }
    
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
            unused_length: 0,
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
            unused_length: 0,
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
            unused_length: 0,
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
            unused_length: 0,
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
            unused_length: 0,
            total_length: 16,
            float_normalizer: None,
        };

        let converter = IntegerConverter::new(cfg).unwrap();

        let val_str = b"-10000123456.001";
        let val = converter.to_i64(val_str);

        assert_eq!(val, -10_000_123_456);

        Ok(())
    }

    #[test]
    fn mock() {
        let x: &[u8; 1] = b"8";
        let x: u8 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u8) };
        let y: u8 = x & 0x0f;
        assert_eq!(y, 8);

        let x: &[u8; 2] = b"12";
        let x: u16 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u16) };
        let lower: u16 = (x & 0x0f00) >> 8;
        let upper: u16 = (x & 0x000f) * 10;

        dbg!(lower, upper);

        let x: &[u8; 4] = b"1234";  
        let x: u32 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u32) };

        let lower: u32 = (x & 0x0f000f00) >> 8;
        let upper: u32 = (x & 0x000f000f) * 10;
        let chunk = lower + upper;

        let lower: u32 = (chunk & 0x00ff0000) >> 16;
        let upper: u32 = (chunk & 0x000000ff) * 100;

        dbg!(lower, upper);
        

    }
}
