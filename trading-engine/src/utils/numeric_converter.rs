use crate::{
    log_warn,
    log_error,
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


#[inline]
fn div_rem(dividend: i64, divisor: i64) -> (i64, i64) {
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);
    (quotient, remainder)
}

#[inline]
pub fn parse_16_chars_by_split(s: &[u8]) -> u64 {
    let (upper_digits, lower_digits) = s.split_at(8);
    parse_8_chars(upper_digits) * 10_000_000 + parse_8_chars(lower_digits)
}

#[inline(always)]
pub fn parse_8_chars(s: &[u8]) -> u64 { 
    debug_assert!(s.len() >= 8, "Input slice must be at least 8 bytes long(pasre_8_chars)");

    let mut chunk: u64 = unsafe { read_unaligned(s.as_ptr() as *const u64) };

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


#[inline]
pub fn parse_32_chars_by_split(s: &[u8]) -> u64 {
    parse_16_chars_with_u128(&s[16..]) + parse_16_chars_with_u128(&s[..16]) * 10_000_000_000_000_000
}

#[inline(always)]
pub fn parse_16_chars_with_u128(s: &[u8]) -> u64 {
    debug_assert!(s.len() >= 16, "Input slice must be at least 16 bytes long (parse_16_chars_with_u128)");
    let mut chunk: u128 = unsafe { read_unaligned(s.as_ptr() as *const u128) };

    // 1-byte mask trick (works on 8 pairs of single digits)
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
    let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
    chunk = lower_digits + upper_digits;
    // 
    chunk as u64
}

#[derive(Debug, Serialize)]
pub struct IntegerConverter {
    numcfg: NumReprCfg,
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

        let buffer_length = if all_digit_size <= 8 {
            8
        } else if all_digit_size <= 16 {
            16
        } else if all_digit_size <= 18 {
            log_warn!("long digit", number_config = numcfg, all_digit_size = all_digit_size);
            32
        } else {
            log_error!(
                "unsupported digit size", 
                number_config = numcfg, 
                digit_size = all_digit_size,
                message = "number show digit is larger than 18 digits is over the capacity of u64 type",
            );
            bail!("unsupported digit size")
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
            16 => parse_16_chars_with_u128(&self.positive_digit_buffer),
            32 => parse_32_chars_by_split(&self.positive_digit_buffer),
            _ => parse_8_chars(&self.positive_digit_buffer),
        }
    }
     
    #[inline(always)]
    pub fn to_i64(&mut self, value: &[u8]) -> i64 {
        match value[0] == b'-' {
            false => self.to_u64(value) as i64,
            _ => (!self.to_u64(value)).wrapping_add(1) as i64,
        }
    }

    #[inline]
    pub fn to_f64_from_i64(&mut self, value: i64) -> f64 {
        value as f64 / 10_f64.powi(self.numcfg.decimal_point_length as i32)
    }

    #[inline]
    pub fn normalized_f64_from_i64(&mut self, value: i64) -> f64 {
        match self.numcfg.float_normalizer {
            Some(normalizer) => {
                let added_normalizer = normalizer + self.numcfg.decimal_point_length as i32;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chars_parser() {
        let s = b"00001234";
        assert_eq!(parse_8_chars(s), 1_234);

        let s = b"00000000000012340000123400001234";
        let val = parse_32_chars_by_split(s);
        assert_eq!(val, 12_340_000_123_400_001_234);
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