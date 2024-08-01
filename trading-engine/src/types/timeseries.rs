use crate::types::{
    base::Real,
    timestamp::TimeStamp,
};

pub struct TimeSeriesPoint {
    pub value: Real,
    pub timestamp: TimeStamp,
}