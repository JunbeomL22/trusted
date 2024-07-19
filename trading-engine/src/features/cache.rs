use crate::types::base::{
    TimeInterval,
    MilliTimeSeriesPoint,
    MicroTimeSeriesPoint,
};
/// Cache is a struct that holds the past data
/// Current data is the data after the last cache time T, and before T + cache_time_interval
/// Cache can be made by many methods, like close, average, sum, etc
#[derive(Debug, Clone)]
pub struct Cache {
    pub cache_timeinterval: TimeInterval,
    pub current_timeinterval: TimeInterval,
    pub cache: Vec<MilliTimeSeriesPoint>,
    pub current: Vec<MicroTimeSeriesPoint>,
}