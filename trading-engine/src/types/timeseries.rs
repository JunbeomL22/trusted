use crate::types::{
    base::Real,
    timestamp::{TimeStampInSec, DateTimeStampInSec},
};

pub struct TimeSeriesPoint {
    pub value: Real,
    pub timestamp: TimeStampInSec,
}
pub struct DateTimeSeriesPoint {
    pub value: Real,
    pub timestamp: DateTimeStampInSec,
}