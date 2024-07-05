use crate::{
    log_warn,
    log_error,
};
use serde::{Serialize, Deserialize};
use anyhow::{Result, bail, anyhow};
use std::ptr::copy_nonoverlapping;
use std::mem::size_of_val;

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

pub fn parse_8_chars(s: &[u8]) -> u64 { // no need to benchmark this, to be used later
    let s = s.as_ptr() as *const _;
    let mut chunk = 0;
    unsafe {
        copy_nonoverlapping(s, &mut chunk, size_of_val(&chunk));
    }

    let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f) * 10;
    let chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on 2 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff) * 100;
    let chunk = lower_digits + upper_digits;

    // 4-byte mask trick (works on a pair of four digits)
    let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff) * 10000;
    let chunk = lower_digits + upper_digits;    

    chunk
}

#[inline]
pub fn parse_32_chars_by_split(s: &[u8]) -> u64 {
    let (upper_digits, lower_digits) = s.split_at(16);
    parse_16_chars_with_u128(upper_digits) * 10_000_000_000_000_000 + parse_16_chars_with_u128(lower_digits)
}

pub fn parse_16_chars_with_u128(s: &[u8]) -> u64 {
    let s = s.as_ptr() as *const u128;
    let mut chunk = 0_u128;
    unsafe {
        copy_nonoverlapping(s, &mut chunk, size_of_val(&chunk));
    }
    // 1-byte mask trick (works on 8 pairs of single digits)
    let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
    let chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on 4 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
    let chunk = lower_digits + upper_digits;

    // 4-byte mask trick (works on 2 pairs of four digits)
    let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
    let chunk = lower_digits + upper_digits;
    
    // 8-byte mask trick (works on a pair of eight digits)
    let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
    let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
    let chunk = lower_digits + upper_digits;
    
    chunk as u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct IntegerConverter {
    numcfg: NumReprCfg,
    positive_digit_buffer: String,
    #[serde(skip)]
    first_dest_ptr: *mut u8,
    #[serde(skip)]
    second_dest_ptr: *mut u8,
    buffer_length: usize,
    //
    input_first_head_location: usize,
    input_second_head_location: Option<usize>,
}

impl Default for IntegerConverter {
    fn default() -> IntegerConverter {
        let buffer = "0".repeat(8);
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
            //buffer_ptr,
            first_dest_ptr,
            second_dest_ptr,
            buffer_length: 0,
            //
            input_first_head_location: 0,
            input_second_head_location: None,
        }
    }
}

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

        let buffer = "0".repeat(buffer_length);
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
            //buffer_ptr,
            first_dest_ptr,
            second_dest_ptr,
            buffer_length,
            //
            input_first_head_location,
            input_second_head_location,
        })
    }   
        
    #[inline]
    pub fn to_u64(&mut self, value: &[u8]) -> u64 {
        unsafe {
            // Cache value pointer
            let value_ptr = value.as_ptr();
            //let src_ptr = value_ptr.add(self.input_first_head_location);
            copy_nonoverlapping(
                //src_ptr,
                value_ptr.add(self.input_first_head_location),
                self.first_dest_ptr, 
                self.numcfg.digit_length
            );
        
            // decimal range copy (if exists)
            if let Some(input_second_head) = self.input_second_head_location {
                //let src_ptr = value_ptr.add(input_second_head);
                copy_nonoverlapping(
                    //src_ptr, 
                    value_ptr.add(input_second_head),
                    self.second_dest_ptr, 
                    self.numcfg.decimal_point_length
                );
            }
        }

        if self.buffer_length == 8 {
            //parse_8_chars(&self.positive_digit_buffer)
            let mut chunk = 0;
            unsafe {
                copy_nonoverlapping(
                    self.positive_digit_buffer.as_str().as_ptr() as *const u64, 
                    &mut chunk, 
                    size_of_val(&chunk));
            }

            let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
            let upper_digits = (chunk & 0x000f000f000f000f) * 10;
            let chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 2 pairs of two digits)
            let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
            let upper_digits = (chunk & 0x000000ff000000ff) * 100;
            let chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on a pair of four digits)
            let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
            let upper_digits = (chunk & 0x000000000000ffff) * 10000;
            let chunk = lower_digits + upper_digits;    

            chunk
        } else if self.buffer_length == 16 {
            //parse_16_chars_with_u128(&self.positive_digit_buffer)
            //let s = s.as_ptr() as *const u128;
            let mut chunk = 0_u128;
            unsafe {
                copy_nonoverlapping(
                    self.positive_digit_buffer.as_str().as_ptr() as *const u128,
                    &mut chunk,
                    size_of_val(&chunk));
            }
            // 1-byte mask trick (works on 8 pairs of single digits)
            let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
            let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
            let chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 4 pairs of two digits)
            let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
            let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
            let chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on 2 pairs of four digits)
            let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
            let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
            let chunk = lower_digits + upper_digits;
            
            // 8-byte mask trick (works on a pair of eight digits)
            let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
            let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
            let chunk = lower_digits + upper_digits;
            
            chunk as u64
        } else {
            //parse_32_chars_by_split(&self.positive_digit_buffer)
            let (upper_part, lower_part) = self.positive_digit_buffer.as_str().split_at(16);
            let mut chunk = 0_u128;
            unsafe {
                copy_nonoverlapping(
                    upper_part.as_ptr() as *const u128,
                    &mut chunk,
                    size_of_val(&chunk));
            }
            // 1-byte mask trick (works on 8 pairs of single digits)
            let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
            let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
            let chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 4 pairs of two digits)
            let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
            let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
            let chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on 2 pairs of four digits)
            let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
            let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
            let chunk = lower_digits + upper_digits;
            
            // 8-byte mask trick (works on a pair of eight digits)
            let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
            let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
            let chunk = lower_digits + upper_digits;

            let mut second_chunk = 0_u128;
            unsafe {
                copy_nonoverlapping(
                    lower_part.as_ptr() as *const u128,
                    &mut second_chunk,
                    size_of_val(&chunk));
            }

            // 1-byte mask trick (works on 8 pairs of single digits)
            let lower_digits = (second_chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
            let upper_digits = (second_chunk & 0x000f000f000f000f000f000f000f000f) * 10;
            let second_chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 4 pairs of two digits)
            let lower_digits = (second_chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
            let upper_digits = (second_chunk & 0x000000ff000000ff000000ff000000ff) * 100;
            let second_chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on 2 pairs of four digits)
            let lower_digits = (second_chunk & 0x0000ffff000000000000ffff00000000) >> 32;
            let upper_digits = (second_chunk & 0x000000000000ffff000000000000ffff) * 10000;
            let second_chunk = lower_digits + upper_digits;
            
            // 8-byte mask trick (works on a pair of eight digits)
            let lower_digits = (second_chunk & 0x00000000ffffffff0000000000000000) >> 64;
            let upper_digits = (second_chunk & 0x000000000000000000000000ffffffff) * 100000000;
            let second_chunk = lower_digits + upper_digits;

            chunk as u64 * 10_000_000_000_000_000 + second_chunk as u64
        }
    }

    #[inline]
    pub fn to_i64(&mut self, value: &[u8]) -> i64 {
        let value_ptr = value.as_ptr();
        unsafe {
            copy_nonoverlapping(
                value_ptr.add(self.input_first_head_location),
                self.first_dest_ptr, 
                self.numcfg.digit_length
            );
            
            // decimal range copy (if exists)
            if let Some(input_second_head) = self.input_second_head_location {
                //let src_ptr = value_ptr.add(input_second_head);
                copy_nonoverlapping(
                    value_ptr.add(input_second_head),
                    self.second_dest_ptr, 
                    self.numcfg.decimal_point_length
                );
            }
        }

        let val_u64 = if self.buffer_length == 8 {
            //parse_8_chars(&self.positive_digit_buffer)
            let mut chunk = 0;
            unsafe {
                copy_nonoverlapping(
                    self.positive_digit_buffer.as_str().as_ptr() as *const u64, 
                    &mut chunk, 
                    size_of_val(&chunk));
            }

            let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
            let upper_digits = (chunk & 0x000f000f000f000f) * 10;
            let chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 2 pairs of two digits)
            let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
            let upper_digits = (chunk & 0x000000ff000000ff) * 100;
            let chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on a pair of four digits)
            let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
            let upper_digits = (chunk & 0x000000000000ffff) * 10000;
            let chunk = lower_digits + upper_digits;    

            chunk
        } else if self.buffer_length == 16 {
            //parse_16_chars_with_u128(&self.positive_digit_buffer)
            //let s = s.as_ptr() as *const u128;
            let mut chunk = 0_u128;
            unsafe {
                copy_nonoverlapping(
                    self.positive_digit_buffer.as_str().as_ptr() as *const u128,
                    &mut chunk,
                    size_of_val(&chunk));
            }

            // 1-byte mask trick (works on 8 pairs of single digits)
            let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
            let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
            let chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 4 pairs of two digits)
            let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
            let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
            let chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on 2 pairs of four digits)
            let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
            let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
            let chunk = lower_digits + upper_digits;
            
            // 8-byte mask trick (works on a pair of eight digits)
            let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
            let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
            let chunk = lower_digits + upper_digits;
            
            chunk as u64
        } else {
            //parse_32_chars_by_split(&self.positive_digit_buffer)
            //let s = self.positive_digit_buffer.as_str();
            let (upper_part, lower_part) = self.positive_digit_buffer.as_str().split_at(16);
            let mut chunk = 0_u128;
            unsafe {
                copy_nonoverlapping(
                    upper_part.as_ptr() as *const u128,
                    &mut chunk,
                    size_of_val(&chunk));
            }

            // 1-byte mask trick (works on 8 pairs of single digits)
            let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
            let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
            let chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 4 pairs of two digits)
            let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
            let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
            let chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on 2 pairs of four digits)
            let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
            let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
            let chunk = lower_digits + upper_digits;
            
            // 8-byte mask trick (works on a pair of eight digits)
            let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
            let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
            let chunk = lower_digits + upper_digits;

            let mut second_chunk = 0_u128;
            unsafe {
                copy_nonoverlapping(
                    lower_part.as_ptr() as *const u128,
                    &mut second_chunk,
                    size_of_val(&chunk));
            }

            // 1-byte mask trick (works on 8 pairs of single digits)
            let lower_digits = (second_chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
            let upper_digits = (second_chunk & 0x000f000f000f000f000f000f000f000f) * 10;
            let second_chunk = lower_digits + upper_digits;

            // 2-byte mask trick (works on 4 pairs of two digits)
            let lower_digits = (second_chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
            let upper_digits = (second_chunk & 0x000000ff000000ff000000ff000000ff) * 100;
            let second_chunk = lower_digits + upper_digits;

            // 4-byte mask trick (works on 2 pairs of four digits)
            let lower_digits = (second_chunk & 0x0000ffff000000000000ffff00000000) >> 32;
            let upper_digits = (second_chunk & 0x000000000000ffff000000000000ffff) * 10000;
            let second_chunk = lower_digits + upper_digits;
            
            // 8-byte mask trick (works on a pair of eight digits)
            let lower_digits = (second_chunk & 0x00000000ffffffff0000000000000000) >> 64;
            let upper_digits = (second_chunk & 0x000000000000000000000000ffffffff) * 100000000;
            let second_chunk = lower_digits + upper_digits;

            chunk as u64 * 10_000_000_000_000_000 + second_chunk as u64
            //parse_16_chars_with_u128(upper_digits) * 10_000_000_000_000_000 + parse_16_chars_with_u128(lower_digits)
        };

        let is_negative = unsafe {
            (!((*value_ptr == b'-') as u64)).wrapping_add(1)
        };
        let abs_value = ((!val_u64).wrapping_add(1) & is_negative).wrapping_add(val_u64 & !is_negative);
        //let abs_value = ((!val_u64 + 1) & is_negative) + (val_u64 & !is_negative);
            
        abs_value as i64
        /*
        if value.starts_with("-") {
            (!val_u64 + 1) as i64
        } else {
            val_u64 as i64
        }
         */
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