use crate::types::timestamp::{
    TimeScale,
    TimeStamp,
};
use crate::types::base::Real;
use serde::{Serialize, Deserialize};

/// weight is the weight of the previous value close to 1.0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default, PartialOrd)]
pub struct TimeSeriesWeight {
    pub weight: Real,
    pub scale: TimeScale,
}

impl TimeSeriesWeight {
    pub fn apply(&self, prev_ts: TimeStamp, current_ts: TimeStamp) -> Real {
        let diff = match self.scale {
            TimeScale::Minute => current_ts.diff_in_mins(prev_ts),
            TimeScale::Milli => current_ts.diff_in_millis(prev_ts),
            TimeScale::Hour => current_ts.diff_in_hours(prev_ts),
            TimeScale::Day => current_ts.diff_in_days(prev_ts),
            TimeScale::Micro => current_ts.diff_in_micros(prev_ts),
            _ => current_ts.diff_in_secs(prev_ts), // to incentivize the use of TimeScale::Second
        };
        self.weight.powf(diff)
    }
    
    #[inline(always)]
    #[must_use]
    pub fn current_weight(interval_weight: Real) -> Real {
        1.0 - interval_weight
    }

    #[inline(always)]
    #[must_use]
    pub fn previous_weight(interval_weight: Real) -> Real {
        interval_weight
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::timestamp::{
        TimeStamp,
        SECOND_NANOSCALE,
        MILLI_NANOSCALE,
        MINUTE_NANOSCALE,
    };

    #[test]
    fn test_time_series_weight() {
        let ts_weight = TimeSeriesWeight {
            weight: 0.5,
            scale: TimeScale::Second,
        };
        let prev_ts = TimeStamp {
            stamp: 1 * SECOND_NANOSCALE,
        };
        let current_ts = TimeStamp {
            stamp: 1 * SECOND_NANOSCALE + 50 * MILLI_NANOSCALE,
        };
        // diff => 0.05
        let result = ts_weight.apply(prev_ts, current_ts);
        assert_eq!((result - (0.5 as Real).powf(0.05)).abs() < 1e-6, true);
    }
}