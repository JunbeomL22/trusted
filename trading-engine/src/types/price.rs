/// The price struct used in trading system
/// This is made for avoiding floating posint error
/// value = 1,234,000,000 and precision = 3. Then the original value was 1.234
pub struct IoI64 {
    value: i64,
    precision: u8,
}

impl IoI64 {
    pub fn new(value: i64, precision: u8) -> IoI64 {
        IoI64 {
            value,
            precision,
        }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn precision(&self) -> u8 {
        self.precision
    }
}