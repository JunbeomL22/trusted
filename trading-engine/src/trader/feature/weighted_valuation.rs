use crate::types::base::{
    Real,
    MilliTimeStamp,
};
/// Exponential Weighted Moving Average
/// value_t = (1 - weight) * value_t-1 + weight * value_t
#[derive(Debug, Clone)]
pub struct EWMA {
    weight_sec: Real,
    value: Real,
    timestamp: MilliTimeStamp,
}
impl Default for EWMA {
    fn default() -> Self {
        EWMA {
            weight_sec: 0.0,
            value: 0.0,
            timestamp: MilliTimeStamp { stamp: 0 },
        }
    }
}

impl EWMA {
    pub fn new(weight_in_sec: Real, value: Real, timestamp: MilliTimeStamp) -> Self {
        EWMA {
            weight_sec: weight_in_sec,
            value,
            timestamp,
        }
    }

    #[inline]
    pub fn update(&mut self, new_value: Real, new_timestamp: MilliTimeStamp) {
        let time_diff_sec = (new_timestamp - self.timestamp).to_seconds();
        let weight = self.weight_sec.powf(time_diff_sec);
        self.value = (1.0 - weight) * self.value + weight * new_value;
    }

    #[inline]
    pub fn get_value(&self) -> Real {
        self.value
    }
}

/// Exponential Weighted Moving Summation
/// S_t = (1 - weight) * S_t-1 + x_t
#[derive(Debug, Clone)]
pub struct EWMS {
    weight_sec: Real,
    summation: Real,
    timestamp: MilliTimeStamp,
}

impl Default for EWMS {
    fn default() -> Self {
        EWMS {
            weight_sec: 0.0,
            summation: 0.0,
            timestamp: MilliTimeStamp { stamp: 0 },
        }
    }
}

impl EWMS {
    pub fn new(weight_in_sec: Real, timestamp: MilliTimeStamp) -> Self {
        EWMS {
            weight_sec: weight_in_sec,
            summation: 0.0,
            timestamp,
        }
    }

    #[inline]
    pub fn update(&mut self, new_value: Real, new_timestamp: MilliTimeStamp) {
        let time_diff_sec = (new_timestamp - self.timestamp).to_seconds();
        let weight = self.weight_sec.powf(time_diff_sec);
        self.summation = (1.0 - weight) * self.summation + weight * new_value;
    }

    #[inline]
    pub fn get_summation(&self) -> Real {
        self.summation
    }
}