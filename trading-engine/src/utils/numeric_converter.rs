use crate::types::base::NumReprCfg;
use anyhow::{Result, bail};
use core::arch::x86_64::{
    _mm_cvtsi128_si64, _mm_lddqu_si128, _mm_madd_epi16, _mm_maddubs_epi16, _mm_packus_epi32,
    _mm_set1_epi8, _mm_set_epi16, _mm_set_epi8, _mm_sub_epi16,
};

fn div_rem(dividend: i64, divisor: i64) -> (i64, i64) {
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);
    (quotient, remainder)
}

pub fn parse_16_chars_simd_c16(s: &str) -> u64 {
    let d: &mut [u8; 16] = &mut b"0000000000000000".clone();
    let b: &[u8] = s.as_bytes();
    d.copy_from_slice(b);

    unsafe {
        let chunk = _mm_lddqu_si128(std::mem::transmute_copy(&d));
        let zeros = _mm_set1_epi8(b'0' as i8);
        let chunk = _mm_sub_epi16(chunk, zeros);

        let mult = _mm_set_epi8(1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10);
        let chunk = _mm_maddubs_epi16(chunk, mult);

        let mult = _mm_set_epi16(1, 100, 1, 100, 1, 100, 1, 100);
        let chunk = _mm_madd_epi16(chunk, mult);

        let chunk = _mm_packus_epi32(chunk, chunk);
        let mult = _mm_set_epi16(0, 0, 0, 0, 1, 10000, 1, 10000);
        let chunk = _mm_madd_epi16(chunk, mult);

        let chunk = _mm_cvtsi128_si64(chunk) as u64;
        ((chunk & 0xffffffff) * 100000000) + (chunk >> 32)
    }
}

pub fn parse_8_chars(s: &str) -> u64 { // no need to benchmark this, to be used later
    let s = s.as_ptr() as *const _;
    let mut chunk = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(s, &mut chunk, std::mem::size_of_val(&chunk));
    }

    // 1-byte mask trick (works on 4 pairs of single digits)
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

#[derive(Debug, Clone)]
pub struct IntegerConverter {
    cfg: NumReprCfg,
    positive_digit_buffer: String,
    buffer_ptr: *mut u8,
    first_dest_ptr: *mut u8,
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
            cfg: NumReprCfg::default(),
            positive_digit_buffer: buffer,
            buffer_ptr,
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
    pub fn new (cfg: NumReprCfg) -> Result<IntegerConverter> {
        let mut digit_size = cfg.total_length;
        if cfg.include_negative {
            digit_size -= 1;
        }

        if cfg.decimal_point_length > 0 {
            digit_size -= 1; // take out "."
        }

        let buffer_length = if digit_size <= 8 {
            8
        } else if digit_size <= 16 {
            16
        } else {
            bail!("Invalid total_length: {:?}", cfg);
        };
        let input_first_head_location = match cfg.include_negative {
            true => 1,
            false => 0,
        };
        let input_first_tail_location = cfg.total_length - cfg.decimal_point_length - 1 - input_first_head_location;
        let (input_second_head_location, input_second_tail_location) = if cfg.decimal_point_length > 0 {
            (Some(input_first_tail_location + 2), Some(cfg.total_length - 1))
        } else {
            (None, None)
        };

        let buffer_first_head_location = buffer_length - digit_size;

        let buffer_first_tail_location = buffer_first_head_location + cfg.digit_length - 1;

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
        let second_dest_ptr = unsafe {
            match cfg.decimal_point_length {
                0 => first_dest_ptr,
                _ => buffer_ptr.add(buffer_second_head_location.unwrap())
            }
        };

        Ok(IntegerConverter {
            cfg,
            positive_digit_buffer: buffer,
            buffer_ptr,
            first_dest_ptr,
            second_dest_ptr,
            buffer_length,
            //
            input_first_head_location,
            input_second_head_location,
        })
    }
        
        
    #[inline]
    pub fn to_u64(&mut self, value: &str) -> Result<u64> {
        unsafe {
            // Cache value pointer
            let value_ptr = value.as_ptr();

            let src_ptr = value_ptr.add(self.input_first_head_location);
            
            std::ptr::copy_nonoverlapping(src_ptr, self.first_dest_ptr, self.cfg.digit_length);
        
            // decimal range copy (if exists)
            if let Some(input_second_head) = self.input_second_head_location {
                let src_ptr = value_ptr.add(input_second_head);
                
                std::ptr::copy_nonoverlapping(src_ptr, self.second_dest_ptr, self.cfg.decimal_point_length);
            }
        }

        if self.buffer_length == 8 {
            Ok(parse_8_chars(&self.positive_digit_buffer))
        } else {
            let (upper_digits, lower_digits) = self.positive_digit_buffer.as_str().split_at(8);
            Ok(parse_8_chars(upper_digits) * 100000000 + parse_8_chars(lower_digits))
        }
    }

    #[inline]
    pub fn to_i64(&mut self, value: &str) -> Result<i64> {
        if value.starts_with('-') {
            Ok((!self.to_u64(value)? + 1) as i64)
        } else {
            Ok(self.to_u64(value)? as i64)
        }
    }

    #[inline]
    pub fn to_f64_from_i64(&mut self, value: i64) -> f64 {
        value as f64 / 10_f64.powi(self.cfg.decimal_point_length as i32)
    }

    #[inline]
    pub fn normalized_f64_from_i64(&mut self, value: i64, add_normalizer: u32) -> f64 {
        let normalizer = self.cfg.decimal_point_length as u32 + add_normalizer;
        let denominator = 10_f64.powi(normalizer as i32);

        let (quotient, remainder) = div_rem(value, 10_i64.pow(normalizer));

        quotient as f64 + (remainder as f64 / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_8_chars_parser() {
        let s = "02345678";
        assert_eq!(parse_8_chars(s), 2_345_678);
    }

    #[test]
    fn test_integer_converter() -> Result<()> {
        let cfg = NumReprCfg::new(
            8,
            2,
            true,
            12,
        )?;
        let mut converter = IntegerConverter::new(cfg).unwrap();
        
        let val = converter.to_u64("000001234.56")?;
        assert_eq!(val, 123456);

        let cfg = NumReprCfg::new(
            11,
            0,
            false,
            11,
        )?;

        let mut converter = IntegerConverter::new(cfg).unwrap();
        let val_str = "10000123456";
        let val = converter.to_u64(val_str)?;
        
        assert_eq!(val, 10_000_123_456);
        
        Ok(())
    }
}