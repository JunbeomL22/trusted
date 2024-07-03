use itoa::Buffer as IntegerBuffer;
use crate::types::precision::PrecisionHelper;
use crate::types::base::NumReprCfg;
use anyhow::{Result, bail};

fn parse_8_chars(s: &str) -> u64 { // no need to benchmark this, to be used later
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
    numeric_only_length: usize,
}

impl Default for IntegerConverter {
    fn default() -> IntegerConverter {
        IntegerConverter {
            cfg: NumReprCfg::default(),
            positive_digit_buffer: String::new(),
            numeric_only_length: 8,
        }
    }
}

impl IntegerConverter {
    pub fn new (cfg: NumReprCfg) -> Result<IntegerConverter> {
        let mut numeric_only_length = cfg.total_length;
        if cfg.include_negative {
            numeric_only_length -= 1;
        }

        if cfg.decimal_point_length > 0 {
            numeric_only_length -= 1; // take out "."
        }

        if numeric_only_length <= 8 {
            numeric_only_length = 8;
        } else if {
            numeric_only_length = 16;
        } else {
            bail!("Invalid total_length: {}", cfg.total_length);
        }

        Ok(IntegerConverter {
            cfg,
            positive_digit_buffer: "0".repeat(numeric_only_length as usize),
            numeric_only_length: numeric_only_length as usize,
        })
    }
        
    pub fn save_buffer(&mut self, value: &str) -> Result<()>{
        let n = self.numeric_only_length;
        let offset = self.cfg.decimal_point_length as usize;
        let input_length = self.cfg.total_length as usize;
        if input_length != value.len() {
            bail!("Invalid input length: {}", value);
        }

        self.positive_digit_buffer.replace_range((n - offset)..n, value[(input_length - offset)..].as_ref());
        self.positive_digit_buffer.replace_range(0..(n - offset), "0".repeat(n - offset));
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
}