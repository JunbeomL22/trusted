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
    prev_proc: &CoarseTimeSeriesPoint, // processed_values[-1]
    current: &CoarseTimeSeriesPoint, // updating value
    _last_updated_point: &CoarseTimeSeriesPoint, // previous updating value
    prev_count: Real, // the number of updating values for making prev_proc
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
    last_updated_point: &CoarseTimeSeriesPoint,
    _prev_count: Real,
) -> CoarseTimeSeriesPoint {
    let timestamp = current.timestamp;
    let diff_time = (current.timestamp - last_updated_point.timestamp).as_real();
    let mid_value = (last_updated_point.value + current.value) / 2.0;
    let value = prev_proc.value + mid_value * diff_time;

    CoarseTimeSeriesPoint {
        timestamp,
        value,
    }
}

fn sum_value(
    prev_proc: &CoarseTimeSeriesPoint, 
    current: &CoarseTimeSeriesPoint, 
    _last_updated_point: &CoarseTimeSeriesPoint,
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
    _last_updated_point: &CoarseTimeSeriesPoint,
    _prev_count: Real,
) -> CoarseTimeSeriesPoint {
    current.clone()
}

#[derive(Debug, Clone)]
pub struct CachedSummation {
    value: Real,
    time_series_processor: CoarseTimeSeriesProcessor,
    last_processed_values_head: usize,
    last_processed_values_tail: usize,
    timestamp: MilliTimeStamp,
}

impl CachedSummation {
    pub fn set_value(&mut self, data: &CoarseTimeSeriesPoint) -> Result<()> {
        self.time_series_processor.update_current_data(data)?;
        
        let last_tail = self.last_processed_values_tail;
        let last_head = self.last_processed_values_head;

        let current_tail = self.time_series_processor.process_tail;
        let current_head = self.time_series_processor.process_head;

        if current_tail == last_tail && current_head == last_head {
            self.value += data.value;
        } else if current_tail != last_tail && current_head != last_head {
            self.value -= self.time_series_processor.processed_values[last_head].value;
            self.last_processed_values_tail = current_tail;
            self.last_processed_values_head = current_head;
        } else {
            let err = || anyhow!(
                "CachedSummation: Error in updating the processed values. \n\
                current_tail: {}, last_tail: {}, current_head: {}, last_head: {}",
                current_tail, last_tail, current_head, last_head
            );
            return Err(err());
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CoarseTimeSeriesProcessor {
    pub processed_values: Vec<CoarseTimeSeriesPoint>, // average, close, sum, etc it must less than window_period
    last_updated_point: CoarseTimeSeriesPoint,
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
    //
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
        let processing_period_sec = (window_period_sec / 30.0).min(6.0).max(0.2) as Real;
        let process_size = (window_period_sec.ceil() / processing_period_sec).ceil() as usize;
        let processed_values = vec![CoarseTimeSeriesPoint::default(); process_size];

        Ok(CoarseTimeSeriesProcessor {
            processed_values,
            last_updated_point: CoarseTimeSeriesPoint::default(),
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
            self.last_updated_point = data_point.clone();
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

    fn cache_null_precessed_values(&mut self) {
        let cache_null = CoarseTimeSeriesPoint::null_point();
        self.processed_values[self.process_tail] = cache_null;
        self.tick_count = 0.0;
    }
    
    pub fn update_current_data(&mut self, data_point: &CoarseTimeSeriesPoint) -> Result<()> {
        // never updated before
        if self.initialize_where_first(data_point) | self.ignore_where_wrong_timestamp(data_point) {
            return Ok(());
        }

        let tail_idx = self.process_tail;

        let update_time_diff = (data_point.timestamp - self.processed_values[tail_idx].timestamp).as_real();
        if update_time_diff <= self.processing_period_sec {
            self.processed_values[tail_idx] = (self.cache_function)(
                &self.processed_values[tail_idx], 
                &data_point, 
                &self.last_updated_point, 
                self.tick_count,
            );
        } else {
            let vec_size = self.processed_values.len();

            self.process_tail = (tail_idx + 1) % vec_size;
            self.cache_null_precessed_values();
            self.processed_values[self.process_tail] = (self.cache_function)(
                &self.processed_values[self.process_tail],
                &data_point, 
                &self.last_updated_point, 
                self.tick_count,
            );
            // head idx
            let window_diff = (data_point.timestamp - self.processed_values[self.process_head].timestamp).as_real();
            if window_diff > self.window_period_sec {
                self.process_head = (self.process_head + 1) % vec_size;
            }
        }

        self.last_updated_point = data_point.clone();
        self.tick_count += 1.0;
        self.update_timestamp_milli = data_point.timestamp;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::base::MilliTimeStamp;

    #[test]
    fn test_cache_creation() -> Result<()> {
        
        let time_series_processor = CoarseTimeSeriesProcessor::new(10.0, sum_value)?;
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