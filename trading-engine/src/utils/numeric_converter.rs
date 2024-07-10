use crate::{
    log_warn,
    log_error,
};
use crate::types::base::{
    BookPrice,
    BookQuantity,
    OrderCount,
};
use serde::{
    Serialize, 
    Deserialize,
    de::Deserializer,
};
use anyhow::{Result, bail, anyhow};
use std::ptr::read_unaligned;
use std::cell::UnsafeCell;

#[derive(Default, Debug, Clone, Serialize, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        let mut check_size: usize = if self.decimal_point_length == 0 {
            self.digit_length
        } else {
            self.digit_length + self.decimal_point_length + 1
        };
    
        check_size += if self.is_signed { 1 } else { 0 };
    
        if check_size != self.total_length {
            let error = || anyhow!(
                "NumReprCfg::new => size check failed: \n\
                digit_length: {:?}\n\
                decimal_point_length: {:?}\n\
                is_signed: {:?}\n\
                total_length: {:?}\n\
                check_size: {:?}",
                self.digit_length, self.decimal_point_length, self.is_signed, self.total_length, check_size
            );
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
    debug_assert!(length <= 8, "parse_under8: length must be less than or equal to 8");
    // ex) u = "123.45", length = 5, point_location = 3
    // "123.45" => "??123.45"
    let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
    if point_length == 0 { return u8_chunk_to_u64_decimal(chunk); }
    // "??123.45" => "123.4500"
    chunk <<= 64 - (length * 8);
    // "123.4500" => "12345000"
    let point_mask = 0xffff_ffff_ffff_ffff << (8 - point_length) * 8;
    let decimal_mask = !point_mask;
   
    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
    u8_chunk_to_u64_decimal(chunk)
}

#[inline(always)]
pub fn parse_under16_with_floating_point(u: &[u8], length: usize, point_length: usize) -> u128 {
    debug_assert!(length <= 16, "parse_under16: length must be less than or equal to 16");
    // ex) u = "123.45", length = 5, point_location = 3
    // "123.45" => "??123.45"
    let mut chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
    if point_length == 0 { return u8_chunk_to_u128_decimal(chunk); }
    // "??123.45" => "123.4500"
    chunk <<= 128 - (length * 8);
    // "123.4500" => "12345000"
    let point_mask = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << (16 - point_length) * 8;
    let decimal_mask = !point_mask;
   
    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
    u8_chunk_to_u128_decimal(chunk)
}

#[inline(always)]
pub fn parse_under8(u: &[u8], length: usize) -> u64 {
    debug_assert!(length <= 8, "parse_under8: length must be less than or equal to 8");
    let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
    chunk <<= 64 - (length * 8);
    u8_chunk_to_u64_decimal(chunk)
}

#[inline(always)]
pub fn parse_under16(u: &[u8], length: usize) -> u128 {
    debug_assert!(length <= 16, "parse_under16: length must be less than or equal to 16");
    let mut chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
    chunk <<= 128 - (length * 8);
    u8_chunk_to_u128_decimal(chunk)
}

#[derive(Debug, Serialize)]
pub struct IntegerConverter {
    numcfg: NumReprCfg,
    decimal_point_length_i32: i32, // for powi
    positive_digit_buffer: Vec<u8>,
    #[serde(skip)]
    first_dest_ptr: UnsafeCell<*mut u8>,
    #[serde(skip)]
    second_dest_ptr: UnsafeCell<*mut u8>,
    buffer_length: usize,
    //
    input_first_head_location: usize,
    input_second_head_location: Option<usize>,
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
        let buffer = vec![0u8; 8];
        let buffer_ptr = buffer.as_ptr() as *mut u8;
        let first_dest_ptr = unsafe {
            buffer_ptr.add(0)
        };
        let second_dest_ptr = unsafe {
            buffer_ptr.add(0)
        };
        IntegerConverter {
            numcfg: NumReprCfg::default(),
            decimal_point_length_i32: 0,
            positive_digit_buffer: buffer,
            first_dest_ptr: UnsafeCell::new(first_dest_ptr),
            second_dest_ptr : UnsafeCell::new(second_dest_ptr),
            buffer_length: 0,
            //
            input_first_head_location: 0,
            input_second_head_location: None,
        }
    }
}

impl Clone for IntegerConverter {
    fn clone(&self) -> Self {
        IntegerConverter::new(self.numcfg).unwrap()
    }
}

unsafe impl Send for IntegerConverter {}

impl IntegerConverter {
    pub fn new (numcfg: NumReprCfg) -> Result<IntegerConverter> {
        numcfg.check_validity()?;

        if !cfg!(target_endian = "little") {
            bail!("IntegerConverter parse by bit operations based only on little endian")
        }

        let mut all_digit_size = numcfg.total_length;
        if numcfg.is_signed {
            all_digit_size -= 1;
        }

        if numcfg.decimal_point_length > 0  {
            all_digit_size -= 1; // take out "."
        }

        if numcfg.drop_decimal_point {
            all_digit_size = numcfg.digit_length;
        }

        let buffer_length = match all_digit_size {
            0..=8 => 8,
            9 => 9,
            10..=16 => 16,
            17..=18 => {
                log_warn!("long digit", number_config = numcfg, all_digit_size = all_digit_size);
                32
            },
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
         
        let input_first_head_location = match numcfg.is_signed {
            true => 1,
            false => 0,
        };
        let input_first_tail_location = if !numcfg.drop_decimal_point {
            numcfg.total_length - numcfg.decimal_point_length - 1 - input_first_head_location
        } else if numcfg.decimal_point_length > 0 {
            numcfg.digit_length - 1 - input_first_head_location - 1
        } else {
            numcfg.digit_length - 1 - input_first_head_location
        };

        let input_second_head_location = if (numcfg.decimal_point_length > 0) && !numcfg.drop_decimal_point {
            Some(input_first_tail_location + 2)
        } else {
            None
        };

        let buffer_first_head_location = buffer_length - all_digit_size;
        let buffer_first_tail_location = buffer_first_head_location + numcfg.digit_length - 1;

        let buffer_second_head_location = if input_second_head_location.is_some() {
            Some(buffer_first_tail_location + 1)
        } else {
            None
        };

        //let buffer = "0".repeat(buffer_length);
        let buffer = vec![b'0'; buffer_length];
        let buffer_ptr = buffer.as_ptr() as *mut u8;
        let first_dest_ptr = unsafe {
            buffer_ptr.add(buffer_first_head_location)
        };
        let second_dest_ptr = if !numcfg.drop_decimal_point {
            match numcfg.decimal_point_length {
                0 => first_dest_ptr,
                _ => unsafe { buffer_ptr.add(buffer_second_head_location.unwrap()) },
            }
        } else {
            first_dest_ptr
        };
    

        Ok(IntegerConverter {
            numcfg,
            decimal_point_length_i32: numcfg.decimal_point_length as i32,
            positive_digit_buffer: buffer,
            //
            first_dest_ptr: UnsafeCell::new(first_dest_ptr),
            second_dest_ptr: UnsafeCell::new(second_dest_ptr),
            buffer_length,
            //
            input_first_head_location,
            input_second_head_location,
        })
    }   
        
    #[inline(always)]
    fn copy_to_buffer(&mut self, value: &[u8]) {
        unsafe {
            let value_ptr = value.as_ptr();
            
            value_ptr.add(self.input_first_head_location).copy_to_nonoverlapping(
                *self.first_dest_ptr.get(),
                self.numcfg.digit_length
            );

            // decimal range copy (if exists)
            if let Some(input_second_head) = self.input_second_head_location {
                value_ptr.add(input_second_head).copy_to_nonoverlapping(
                    *self.second_dest_ptr.get(),
                    self.numcfg.decimal_point_length
                );
            }
        }
    }

    #[inline(always)]
    pub fn to_u64(&mut self, value: &[u8]) -> u64 {
        self.copy_to_buffer(value);
        match self.buffer_length {
            _ => parse_under8(&self.positive_digit_buffer, self.numcfg.digit_length),
            //16 => parse_16_chars_with_u128(&self.positive_digit_buffer) as u64,
            //32 => parse_32_chars_by_split(&self.positive_digit_buffer) as u64,
            //_ => parse_9_chars(&self.positive_digit_buffer),
        }
    }
    
    #[inline(always)]
    pub fn to_i64(&mut self, value: &[u8]) -> i64 {
        match value[0] == b'0' {
            false => (!self.to_u64(value)).wrapping_add(1) as i64,
            _ => self.to_u64(value) as i64,
        }
    }

    #[inline(always)]
    pub fn to_f64_from_i64(&mut self, value: i64) -> f64 {
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
            },
            None => {
                self.to_f64_from_i64(value)
            }
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
    pub fn to_book_price(&mut self, val: &[u8]) -> BookPrice {
        self.price.to_i64(val)
    }

    pub fn to_book_quantity(&mut self, val: &[u8]) -> BookQuantity {
        self.quantity.to_u64(val)
    }

    pub fn to_order_count(&mut self, val: &[u8]) -> OrderCount {
        assert!(val.len() <= 10, "OrderCount must be fewer than 10 bytes");
        self.order_count.to_u64(val) as u32
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeStampConverter {
    pub converter: IntegerConverter,
}

impl TimeStampConverter {
    pub fn to_timestamp(&mut self, val: &[u8]) -> u64 {
        self.converter.to_u64(val)
    }
}

unsafe impl Sync for OrderConverter {}

unsafe impl Sync for TimeStampConverter {}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chars_parser() {
        let s = b"1";
        let val = parse_under8(s, 1);
        assert_eq!(val, 1);

        let s = b"12";
        let val = parse_under8(s, 2);
        assert_eq!(val, 12);

        let s = b"123";
        let val = parse_under8(s, 3);
        assert_eq!(val, 123);

        let s = b"1234";
        let val = parse_under8(s, 4);
        assert_eq!(val, 1234);

        let s = b"12345";
        let val = parse_under8(s, 5);
        assert_eq!(val, 12345);

        let s = b"123456";
        let val = parse_under8(s, 6);
        assert_eq!(val, 123456);

        let s = b"1234567";
        let val = parse_under8(s, 7);
        assert_eq!(val, 1234567);

        let s = b"12345678";
        let val = parse_under8(s, 8);
        assert_eq!(val, 12345678);

        let s = b"123456789";
        let val = parse_under16(s, 9);
        assert_eq!(val, 123456789);

        let s = b"1234567890";
        //let val = parse_10_chars(s);
        let val = parse_under16(s, 10);
        assert_eq!(val, 1234567890);

        let s = b"1234.56";
        let val = parse_under8_with_floating_point(s, 7, 2);
        assert_eq!(val, 123456);

        let s = b"1234.567";
        let val = parse_under8_with_floating_point(s, 8, 3);
        assert_eq!(val, 1234567);

        let s = b"012345678.901";
        let val = parse_under16_with_floating_point(s, 13, 3);
        assert_eq!(val, 12345678901);

    }
    #[test]
    fn test_parse_small_number() {
        let cfg = NumReprCfg {
            digit_length: 4,
            decimal_point_length: 1,
            drop_decimal_point: false,
            is_signed: true,
            total_length: 7,
            float_normalizer: None,
        };
        let mut converter = IntegerConverter::new(cfg).unwrap();
        let val = converter.to_u64(b"01234.5");
        assert_eq!(
            val, 12345,
            "convert failed: {:?}",
            std::str::from_utf8(&converter.positive_digit_buffer).unwrap()
        );
    }

    #[test]
    fn test_integer_converter() -> Result<()> {
        let cfg = NumReprCfg {
            digit_length: 8,
            decimal_point_length: 2,
            drop_decimal_point: false,
            is_signed: true,
            total_length: 12,
            float_normalizer: None,
        };
        let mut converter = IntegerConverter::new(cfg).unwrap();
        
        let val = converter.to_u64(b"000001234.56");
        assert_eq!(val, 123_456);

        let val_i64 = converter.to_i64(b"-10001234.56");
        assert_eq!(val_i64, -1_000_123_456);

        let cfg = NumReprCfg {
            digit_length: 11,
            decimal_point_length: 0,
            drop_decimal_point: false,
            is_signed: false,
            total_length: 11,
            float_normalizer: None,
        };

        let mut converter = IntegerConverter::new(cfg).unwrap();
        let val_str = b"10000123456";
        let val = converter.to_u64(val_str);
        
        assert_eq!(val, 10_000_123_456);
        
        let cfg_for_big_number = NumReprCfg {
            digit_length: 15,
            decimal_point_length: 1,
            drop_decimal_point: false,
            is_signed: true,
            total_length: 18,
            float_normalizer: None,
        };

        let mut converter = IntegerConverter::new(cfg_for_big_number).unwrap();

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
            decimal_point_length: 3,
            drop_decimal_point: true,
            is_signed: true,
            total_length: 16,
            float_normalizer: None,
        };

        let mut converter = IntegerConverter::new(cfg).unwrap();

        let val_str = b"-10000123456.001";
        let val = converter.to_i64(val_str);
        
        assert_eq!(val, -10_000_123_456);

        Ok(())
    }
}