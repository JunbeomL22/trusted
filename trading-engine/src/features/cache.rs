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

const DEFAULT_VECTOR_SIZE_CHUNK: usize = 128;

fn last_value(current: &Vec<CoarseTimeSeriesPoint>, last_idx: usize) -> CoarseTimeSeriesPoint {
    let timestamp = current[last_idx].timestamp;
    let value = current[last_idx].value;
    CoarseTimeSeriesPoint {
        timestamp,
        value,
    }
}

fn average_value(current: &Vec<CoarseTimeSeriesPoint>, last_idx: usize) -> CoarseTimeSeriesPoint {
    let mut sum = 0.0;
    for i in 0..last_idx {
        sum += current[i].value;
    }
    let value = sum / (last_idx + 1) as Real;
    let timestamp = current[last_idx].timestamp;
    CoarseTimeSeriesPoint {
        timestamp,
        value,
    }
}

fn sum_value(current: &Vec<FineTimeSeriesPoint>, last_idx: usize) -> CoarseTimeSeriesPoint {
    let mut sum = 0.0;
    for i in 0..last_idx {
        sum += current[i].value;
    }
    let timestamp = current[last_idx].timestamp.to_millis();
    CoarseTimeSeriesPoint {
        timestamp,
        value: sum,
    }
}

/// Cache is a struct that holds the past data
/// Current data is the data after the last cache time T, and before T + cache_time_interval
/// Cache can be made by many methods, like close, average, sum, 
/// Ex) 30sec ofi with 1sec cache => 
/// cache_time_interval = 1sec, current_time_interval = Tick, window_period = 30sec
/// In this case, the actual 30sec ofi time range is from (29sec, 30sec]
/// To avoid any headache, we will limit the unit of cache_time_interval is smaller than window_period (both can't be milliseconds)
/// The last cache is updading current value
/// Note that helper is "current", not "cache". We, in the end, use "cache" for signal generation.
/// The "current" is used for signal only when the cashe.len() == 0
#[derive(Debug, Clone, Serialize, 
    //Deserialize // implement this later
)]
pub struct Cache {
    pub window_period: TimeInterval,
    pub cache: Vec<CoarseTimeSeriesPoint>,
    pub current: Vec<CoarseTimeSeriesPoint>,
    cache_period_sec: Option<Real>, // None if tick means cache is not used, otherwise recommended to be at least 2 seconds
    window_period_sec: Option<Real>, // this must be positive
    cache_head_idx: usize,
    cache_tail_idx: usize,
    current_tail_idx: usize,
    last_timestamp: MilliTimeStamp,
    update_initialized: bool,
    #[serde(skip)]
    cache_function: fn(&Vec<CoarseTimeSeriesPoint>, usize) -> CoarseTimeSeriesPoint,
}

impl Cache {
    pub fn new(
        window_period: TimeInterval, 
        cache_function: fn(&Vec<CoarseTimeSeriesPoint>, usize) -> CoarseTimeSeriesPoint,
    ) -> Result<Self> {
        match window_period.unit {
            TimeStepUnit::Tick => {
                // the case where window_period is tick
                Ok(Cache {
                    window_period,
                    cache: vec![],
                    current: vec![CoarseTimeSeriesPoint::default()],
                    cache_period_sec: None,
                    window_period_sec: None,
                    cache_head_idx: 0,
                    cache_tail_idx: 0,
                    current_tail_idx: 0,
                    last_timestamp: MilliTimeStamp { stamp: 0 },
                    update_initialized: false,
                    cache_function,
                })
            },
            _ => {
                let window_period_sec = window_period.interval * window_period.unit.to_seconds()?;
                if window_period_sec > 86_400.0 {
                    return Err(anyhow!(
                        "window_period can not be more than 1 day in Cache struct\n\
                        if you want this, you need to use other struct or modify this struct\n\
                        It is desirable to set under an hour")
                    );
                }

                let cache_period_sec: Option<Real> = if window_period_sec > 600.0 {
                    Some(2.0)
                } else if window_period_sec < 4.0 {
                    None
                } else if window_period_sec < 20.0 {
                    Some(0.25)
                } else if window_period_sec < 60.0 {
                    Some(0.5)
                } else {
                    Some(1.0)
                };
                
                let cache = if let Some(cache_period_sec) = cache_period_sec {
                    let size = (window_period_sec.ceil() / cache_period_sec).ceil() as usize + 1;
                    vec![CoarseTimeSeriesPoint::default(); size]
                } else {
                    vec![]
                };
                

                let current = if let Some(cache_period_sec) = cache_period_sec {
                    let size = (cache_period_sec.ceil() as usize * DEFAULT_VECTOR_SIZE_CHUNK).min(1024);
                    vec![CoarseTimeSeriesPoint::default(); size]
                } else {
                    vec![CoarseTimeSeriesPoint::default(); DEFAULT_VECTOR_SIZE_CHUNK]
                };

                Ok(Cache {
                    window_period,
                    cache,
                    current,
                    cache_period_sec,
                    window_period_sec: Some(window_period_sec),
                    cache_head_idx: 0,
                    cache_tail_idx: 0,
                    current_tail_idx: 0,
                    last_timestamp: MilliTimeStamp { stamp: 0 },
                    update_initialized: false,
                    cache_function,
                })
            }
        }        
    }

    /// let's say
    pub fn update_current_data(&mut self, data_point: CoarseTimeSeriesPoint) -> Result<()> {
        if self.update_initialized == false {
            self.update_initialized = true;
            self.last_timestamp = data_point.timestamp;
            self.current[0] = data_point;
            if self.cache_period_sec.is_none() {
                self.current_tail_idx = 1;
                return Ok(());
            } else {
                let cache_data = (self.cache_function)(&self.current, 0);
                self.cache[0] = cache_data;
                self.current_tail_idx = 1;
            }
            return Ok(());
        }

        if self.last_timestamp > data_point.timestamp {
            let last_timestamp = self.last_timestamp;
            crate::log_info!("WrongTimestamp", latest_timestamp = last_timestamp, new_timestamp = data_point.timestamp);
            return Ok(());
        }

        if self.current_tail_idx == self.current.len() {
            // if the current is full, we need to newly allocate the current vector
            let st = get_unix_nano();
            self.current.extend(vec![CoarseTimeSeriesPoint::default(); DEFAULT_VECTOR_SIZE_CHUNK]);
            let tm = get_unix_nano() - st;
            let add_msg = format!(
                "current data is full, allocating current data vector. 
                To avoid this, it is suggested to increase the current_timeinterval.\
                it took {} nanos to allocate and attach {} FinTimeSeriesPoint", 
                tm, DEFAULT_VECTOR_SIZE_CHUNK
            );

            crate::log_warn!(
                "FatFeature",
                message = add_msg,
                data_point = data_point.clone(),
                note = add_msg,
            );
            
        } 
        
        self.current[self.current_tail_idx] = data_point;
        self.current_tail_idx += 1;
        if self.cache_period_sec.is_none() {
            return Ok(());
        }

        // updating cache
        let cache_data = (self.cache_function)(&self.current, (self.current_tail_idx - 1));
       
        let last_cache_timestamp = self.cache[self.cache_tail_idx].timestamp;
       
        let cache_time_diff = cache_data.timestamp - last_cache_timestamp;
        // dbg!(cache_data);
        // dbg!(last_cache_timestamp);
        // dbg!(cache_time_diff.as_real() / 1_000.0);
        

        if self.cache_period_sec.is_some() &&
            cache_time_diff.as_real() / 1_000.0 < self.cache_period_sec.unwrap() {
            // If cache data is given in too short period of time, we just modify the last data
            self.cache[self.cache_tail_idx] = cache_data;
        } else {
            // otherwise, we need to push the data to the cache vector
            self.cache_tail_idx = (self.cache_tail_idx + 1) % self.cache.len();
            self.cache_head_idx = (self.cache_tail_idx + 1) % self.cache.len();
            self.current_tail_idx = 0;
            self.cache[self.cache_tail_idx] = cache_data;
        }

        self.last_timestamp = data_point.timestamp;
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