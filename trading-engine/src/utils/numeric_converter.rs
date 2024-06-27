use ryu::{
    Buffer as FloatBuffer,
    Float,
};

use itoa::Buffer as IntegerBuffer;

pub struct FloatConverter {
    float_buffer: FloatBuffer,
}

impl FloatConverter {
    pub fn new() -> Self {
        Self {
            float_buffer: FloatBuffer::new(),
        }
    }

    pub fn float_to_str<T: Float>(&mut self, value: T) -> &str {
        self.float_buffer.format(value) // 40ns where std_string is 140ns
    }

    pub fn str_to_f64(&self, value: &str) -> f64 {
        value.parse::<f64>().unwrap() // 8ns
    }

    pub fn str_to_f32(&self, value: &str) -> u64 {
        value.parse::<u64>().unwrap() // 10ns (why take longer than f64?)
    }
}

pub struct IntegerConverter {
    integer_buffer: IntegerBuffer,
}

impl IntegerConverter {
    pub fn new() -> Self {
        Self {
            integer_buffer: IntegerBuffer::new(),
        }
    }

    pub fn u32_to_str(&mut self, value: u32) -> &str {
        self.integer_buffer.format(value) // 2ns
    }

    pub fn str_to_u32(&self, value: &str) -> u32 {
        value.parse::<u32>().unwrap() // 10ns
    }


    pub fn u64_to_str(&mut self, value: u64) -> &str {
        self.integer_buffer.format(value) // 2ns
    }

    pub fn str_to_u64(&self, value: &str) -> u64 {
        value.parse::<u64>().unwrap() // 10ns
    }

    pub fn i64_to_str(&mut self, value: i64) -> &str {
        self.integer_buffer.format(value) // 2ns
    }

    pub fn str_to_i64(&self, value: &str) -> i64 {
        value.parse::<i64>().unwrap() // 10ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_converter() {
        let mut converter = FloatConverter::new();

        let f64_str = "000324.234";
        let res: f64 = converter.str_to_f64(f64_str);
        assert_eq!(res, 324.234);

        let str_again = converter.float_to_str(324.234);
        assert_eq!(str_again, "324.234");
    }

    #[test]
    fn test_integer_converter() {
        let mut converter = IntegerConverter::new();

        let u64_str = "001234567890";
        let res: u64 = converter.str_to_u64(u64_str);
        assert_eq!(res, 1234567890);

        let str_again = converter.u64_to_str(1234567890);
        assert_eq!(str_again, "1234567890");
    }
}