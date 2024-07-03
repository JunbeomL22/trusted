use itoa::Buffer as IntegerBuffer;
use crate::types::precision::PrecisionHelper;
use crate::types::base::NumReprCfg;
use anyhow::{Result, bail};
use core::arch::x86_64::{
    _mm_cvtsi128_si64, _mm_lddqu_si128, _mm_madd_epi16, _mm_maddubs_epi16, _mm_packus_epi32,
    _mm_set1_epi8, _mm_set_epi16, _mm_set_epi8, _mm_sub_epi16,
};

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
    positive_digit_buffer: String,
    buffer_length: usize,
    //
    input_first_head_location: usize,
    input_first_tail_location: usize,
    input_second_head_location: Option<usize>,
    input_second_tail_location: Option<usize>,
    //
    buffer_first_head_location: usize,
    buffer_first_tail_location: usize,
    buffer_second_head_location: Option<usize>,
    buffer_second_tail_location: Option<usize>,

}

impl Default for IntegerConverter {
    fn default() -> IntegerConverter {
        IntegerConverter {
            positive_digit_buffer: String::new(),
            buffer_length: 0,
            //
            input_first_head_location: 0,
            input_first_tail_location: 0,
            input_second_head_location: None,
            input_second_tail_location: None,
            //
            buffer_first_head_location: 0,
            buffer_first_tail_location: 0,
            buffer_second_head_location: None,
            buffer_second_tail_location: None,
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
            bail!("Invalid total_length: {}", cfg.total_length);
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

        let buffer_second_tail_location = if input_second_tail_location.is_some() {
            Some(buffer_length - 1)
        } else {
            None
        };

        Ok(IntegerConverter {
            positive_digit_buffer: "0".repeat(buffer_length),
            buffer_length,
            //
            input_first_head_location,
            input_first_tail_location,
            input_second_head_location,
            input_second_tail_location,
            //
            buffer_first_head_location,
            buffer_first_tail_location,
            buffer_second_head_location,
            buffer_second_tail_location,
        })
    }
        
        
    pub fn to_u64(&mut self, value: &str) -> Result<u64> {
        unsafe {
            // Ensure the lengths match to avoid buffer overflow or underflow
            let src_len = self.input_first_tail_location - self.input_first_head_location + 1;
            let dest_len = self.buffer_first_tail_location - self.buffer_first_head_location + 1;
            assert_eq!(src_len, dest_len, "Source and destination lengths must match.");
        
            // Get pointers to the start of the source and destination slices
            let src_ptr = value.as_ptr().add(self.input_first_head_location);
            let dest_ptr = self.positive_digit_buffer.as_mut_ptr().add(self.buffer_first_head_location);
        
            // Copy from source to destination
            std::ptr::copy(src_ptr, dest_ptr, src_len);
        
            // Repeat for the second range if it exists
            if let (Some(input_second_head_location), Some(buffer_second_head_location), Some(input_second_tail_location), Some(buffer_second_tail_location)) = (
                self.input_second_head_location,
                self.buffer_second_head_location,
                self.input_second_tail_location,
                self.buffer_second_tail_location,
            ) {
                let src_len = input_second_tail_location - input_second_head_location + 1;
                let dest_len = buffer_second_tail_location - buffer_second_head_location + 1;
                assert_eq!(src_len, dest_len, "Source and destination lengths must match.");
        
                let src_ptr = value.as_ptr().add(input_second_head_location);
                let dest_ptr = self.positive_digit_buffer.as_mut_ptr().add(buffer_second_head_location);
        
                std::ptr::copy(src_ptr, dest_ptr, src_len);
            }
        }
        /*
        self.positive_digit_buffer.replace_range(
            self.buffer_first_head_location..=self.buffer_first_tail_location,
            &value[self.input_first_head_location..=self.input_first_tail_location]
        );

        if self.buffer_second_head_location.is_some() {
            self.positive_digit_buffer.replace_range(
                self.buffer_second_head_location.unwrap()..=self.buffer_second_tail_location.unwrap(),
                &value[self.input_second_head_location.unwrap()..=self.input_second_tail_location.unwrap()]
            );
        }
         */
        if self.buffer_length == 8 {
            Ok(parse_8_chars(&self.positive_digit_buffer))
        } else {
            Ok(parse_16_chars_simd_c16(&self.positive_digit_buffer.as_str()))
        }
    }

    pub fn to_i64(&mut self, value: &str) -> Result<i64> {
        if value.starts_with('-') {
            Ok((!self.to_u64(value)? + 1) as i64)
        } else {
            Ok(self.to_u64(value)? as i64)
        }
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
        dbg!(&converter);
        assert_eq!(val, 123456);

        Ok(())
    }
}