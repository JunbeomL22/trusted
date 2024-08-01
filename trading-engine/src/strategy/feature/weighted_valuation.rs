use crate::types::base::Real;
use crate::types::timestamp::TimeStamp;
use crate::strategy::feature::weight::TimeSeriesWeight;
/// Exponential Weighted Moving Average
/// value_t = (1 - weight) * value_t-1 + weight * value_t
#[derive(Debug, Clone)]
pub struct EWMA {
    weight:TimeSeriesWeight,
    timestamp: TimeStamp,
    value: Real,
}
impl Default for EWMA {
    fn default() -> Self {
        EWMA {
            weight: TimeSeriesWeight::default(),    
            value: 0.0,
            timestamp: TimeStamp::default(),
        }
    }
}

impl EWMA {
    pub fn new(weight: TimeSeriesWeight, value: Real, timestamp: TimeStamp) -> Self {
        EWMA {
            weight,
            value,
            timestamp,
        }
    }

    #[inline]
    pub fn update(&mut self, new_value: Real, new_timestamp: TimeStamp) {
        let interval_weight = self.weight.apply(self.timestamp, new_timestamp);
        self.value = TimeSeriesWeight::previous_weight(interval_weight) * self.value
            + TimeSeriesWeight::current_weight(interval_weight) * new_value;
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
    weight: TimeSeriesWeight,
    summation: Real,
    timestamp: TimeStamp,
}

impl Default for EWMS {
    fn default() -> Self {
        EWMS {
            weight: TimeSeriesWeight::default(),
            summation: 0.0,
            timestamp: TimeStamp::default(),
        }
    }
}

impl EWMS {
    pub fn new(weight: TimeSeriesWeight, timestamp: TimeStamp) -> Self {
        EWMS {
            weight,
            summation: 0.0,
            timestamp,
        }
    }

    #[inline]
    pub fn update(&mut self, new_value: Real, new_timestamp: TimeStamp) {
        let interval_weight = self.weight.apply(self.timestamp, new_timestamp);
        self.summation = TimeSeriesWeight::previous_weight(interval_weight) * self.summation + new_value;
    }

    #[inline]
    pub fn get_value(&self) -> Real {
        self.summation
    }
}