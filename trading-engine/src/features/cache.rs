use crate::types::base::{
    TimeInterval,
    MilliTimeStamp,
    CoarseTimeSeriesPoint,
    FineTimeSeriesPoint,
    Real,
};
use crate::types::enums::TimeStepUnit;
use crate::utils::timer::get_unix_nano;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context, anyhow};

fn average_value(
    prev_proc: &CoarseTimeSeriesPoint, 
    current: &CoarseTimeSeriesPoint, 
    _prev_data: &CoarseTimeSeriesPoint,
    prev_count: Real, 
) -> CoarseTimeSeriesPoint {
    let timestamp = current.timestamp;
    let value = (prev_proc.value * prev_count + current.value) / (prev_count + 1.0);

    CoarseTimeSeriesPoint {
        timestamp,
        value,
    }
}

fn mid_point_integration(
    prev_proc: &CoarseTimeSeriesPoint, 
    current: &CoarseTimeSeriesPoint, 
    prev_data: &CoarseTimeSeriesPoint,
    _prev_count: Real,
) -> CoarseTimeSeriesPoint {
    let timestamp = current.timestamp;
    let diff_time = (current.timestamp - prev_data.timestamp).as_real();
    let mid_value = (prev_data.value + current.value) / 2.0;
    let value = prev_proc.value + mid_value * diff_time;

    CoarseTimeSeriesPoint {
        timestamp,
        value,
    }
}

fn sum_value(
    prev_proc: &CoarseTimeSeriesPoint, 
    current: &CoarseTimeSeriesPoint, 
    _prev_data: &CoarseTimeSeriesPoint,
    _prev_count: Real,
) -> CoarseTimeSeriesPoint {
    let value = prev_proc.value + current.value;
    let timestamp = current.timestamp;

    CoarseTimeSeriesPoint {
        timestamp,
        value,
    }
}

fn close_price(
    _prev: &CoarseTimeSeriesPoint, 
    current: &CoarseTimeSeriesPoint, 
    _prev_data: &CoarseTimeSeriesPoint,
    _prev_count: Real,
) -> CoarseTimeSeriesPoint {
    current.clone()
}

#[derive(Debug, Clone, Serialize, 
    //Deserialize // implement this later
)]
pub struct CoarseTimeSeriesProcessor {
    pub processed_values: Vec<CoarseTimeSeriesPoint>, // average, close, sum, etc it must less than window_period
    prev_data: CoarseTimeSeriesPoint,
    tick_count: Real,
    //
    window_period_sec: Real, 
    processing_period_sec: Real, // processed_values not used where None
    //
    process_head: usize,
    process_tail: usize,
    //
    update_timestamp_milli: MilliTimeStamp,
    update_initialized: bool,
    #[serde(skip)]
    cache_function: fn(&CoarseTimeSeriesPoint, &CoarseTimeSeriesPoint, &CoarseTimeSeriesPoint, Real) -> CoarseTimeSeriesPoint,
}

impl CoarseTimeSeriesProcessor {
    pub fn new(
        window_period_sec: Real,
        cache_function: fn(&CoarseTimeSeriesPoint, &CoarseTimeSeriesPoint, &CoarseTimeSeriesPoint, Real) -> CoarseTimeSeriesPoint,
    ) -> Result<Self> {
        if window_period_sec < 3.0 {
            let err = || anyhow!(
                "Given window period: {} \n\
                window_period_sec must be greater than 5.0sec \n\
                For very short period features, use other structs. ",
                window_period_sec
            );
            return Err(err());
        }
        let processing_period_sec = (window_period_sec / 50.0).min(5.0).max(0.25) as Real;
        let process_size = (window_period_sec.ceil() / processing_period_sec).ceil() as usize;

        let processed_values = vec![CoarseTimeSeriesPoint::default(); process_size];

        Ok(CoarseTimeSeriesProcessor {
            processed_values,
            prev_data: CoarseTimeSeriesPoint::default(),
            tick_count: 0.0,
            //
            window_period_sec,
            processing_period_sec,
            //
            process_head: 0,
            process_tail: 0,
            //
            update_timestamp_milli: MilliTimeStamp::default(),
            update_initialized: false,
            cache_function,
        })
    }

    #[inline]
    fn initialize_where_first(&mut self, data_point: &CoarseTimeSeriesPoint) -> bool {
        if self.update_initialized == false {
            self.update_timestamp_milli = data_point.timestamp;
            self.tick_count = 1.0;
            self.prev_data = data_point.clone();
            self.processed_values[0] = data_point.clone();
            true
        } else {
            false
        }
    }

    #[inline]
    fn ignore_where_wrong_timestamp(&mut self, data_point: &CoarseTimeSeriesPoint) -> bool {
        if self.update_timestamp_milli > data_point.timestamp {
            // if the timestamp is wrong, we just ignore the data
            let last_timestamp = self.update_timestamp_milli;
            crate::log_info!("WrongTimestamp", latest_timestamp = last_timestamp, new_timestamp = data_point.timestamp);
            true
        } else {
            false
        }
    }

    pub fn update_current_data(&mut self, data_point: &CoarseTimeSeriesPoint) -> Result<()> {
        // never updated before
        if self.initialize_where_first(data_point) | self.ignore_where_wrong_timestamp(data_point) {
            return Ok(());
        }

        let head_idx = self.process_head;
        let tail_idx = self.process_tail;

        let update_time_diff = (data_point.timestamp - self.processed_values[tail_idx].timestamp).as_real();
        if update_time_diff <= self.processing_period_sec {
            self.processed_values[tail_idx] = (self.cache_function)(
                &self.processed_values[tail_idx], 
                &data_point, 
                &self.prev_data, 
                self.tick_count,
            );
            self.prev_data = data_point.clone();
            self.tick_count += 1.0;
        } else {
            let mut new_tail_idx = tail_idx + 1;
            if new_tail_idx >= self.processed_values.len() {
                new_tail_idx = 0;
            }
            self.process_tail = new_tail_idx;
            let null_cache = CoarseTimeSeriesPoint { 
                timestamp: data_point.timestamp, 
                value: 0.0 
            };
            self.processed_values[new_tail_idx] = (self.cache_function)(
                &null_cache, 
                &data_point, 
                &self.prev_data, 
                0.0,
            );

            self.prev_data = data_point.clone();
            self.tick_count = 1.0;

            // head idx
            let window_diff = (data_point.timestamp - self.processed_values[head_idx].timestamp).as_real();
            if window_diff > self.window_period_sec {
                let mut new_head_idx = head_idx + 1;
                if new_head_idx >= self.processed_values.len() {
                    new_head_idx = 0;
                }
                self.process_head = new_head_idx;
            }
        }

        self.update_timestamp_milli = data_point.timestamp;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::TimeStepUnit;
    use crate::types::base::{
        MilliTimeStamp,
    };
    use crate::utils::memory_investigation::print_struct_info;

    #[test]
    fn test_cache_creation() -> Result<()> {
        let window_period = TimeInterval {
            interval: 30.0,
            unit: TimeStepUnit::Second,
        };
        let cache = Cache::new(window_period, last_value).unwrap();
        assert_eq!(cache.cache.len(), 60 + 1);
        assert_eq!(cache.current.len(), DEFAULT_VECTOR_SIZE_CHUNK);

        let window_period = TimeInterval {
            interval: 10.0,
            unit: TimeStepUnit::Second,
        };
        let mut cache = Cache::new(window_period, last_value).unwrap();

        assert_eq!(cache.cache.len(), 40 + 1);
        assert_eq!(cache.current.len(), DEFAULT_VECTOR_SIZE_CHUNK);

        let data_point = CoarseTimeSeriesPoint {
            timestamp: MilliTimeStamp {
                stamp: 9_07_19_283,
            },
            value: 1.0,
        };

        cache.update_current_data(data_point.clone())?;
        cache.update_current_data(data_point.clone())?;

        let data_point = CoarseTimeSeriesPoint {
            timestamp: MilliTimeStamp {
                stamp: 9_07_20_283,
            },
            value: 2.0,
        };

        cache.update_current_data(data_point.clone())?;
        dbg!(cache.cache_tail_idx);
        dbg!(cache.cache_head_idx);
        dbg!(cache.current_tail_idx);
        dbg!(cache.cache[..=cache.cache_tail_idx].to_vec());
        dbg!(cache.current[..=cache.current_tail_idx].to_vec());
        

        assert!(true);
        Ok(())
    }
}