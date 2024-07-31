use crate::types::base::Real;
use crate::utils::timer::get_unix_nano;
use std::ops::{Add, Sub, AddAssign, SubAssign};
use serde::{Serialize, Deserialize};

use chrono::{
    NaiveDate,
    Utc,
    Duration,
};

pub const ONE_DAY_IN_SEC: Real = 86_400.0;

const EPOCH_ANCHOR: Option<NaiveDate> = NaiveDate::from_ymd_opt(1970, 1, 1);

/// made for pparsing each exchange protocol
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct DateStampGenerator {
    days_from_anchor: u32,
    cached_timestamp_in_sec: Real,
    update_stamp: u64,
}

impl From<NaiveDate> for DateStampGenerator {
    fn from(dt: NaiveDate) -> Self {
        let days = dt.signed_duration_since(EPOCH_ANCHOR.unwrap()).num_days() as u32;
        // get time in secs <= 86,400
        DateStampGenerator { 
            days_from_anchor: days, 
            update_stamp: get_unix_nano(),
            cached_timestamp_in_sec: 0.0,
        }
    }
}

impl DateStampGenerator {
    #[inline]
    #[must_use]
    pub fn get_cached_timestamp_in_sec(&self) -> Real {
        self.cached_timestamp_in_sec
    }

    #[inline]
    pub fn set_cached_timestamp_in_sec(&mut self, stamp: Real) {
        self.cached_timestamp_in_sec = stamp;
    }

    #[inline]
    pub fn new(days: u32, update_stamp: u64) -> Self {
        DateStampGenerator { 
            days_from_anchor: days,
            update_stamp,
            cached_timestamp_in_sec: 0.0,           
        }
    }

    pub fn from_today() -> Self {
        let now_unix = get_unix_nano();
        let today = chrono::DateTime::from_timestamp_nanos(now_unix as i64);
        let today_date = today.naive_utc().date();
        let days = today_date.signed_duration_since(EPOCH_ANCHOR.unwrap()).num_days() as u32;
        DateStampGenerator { 
            days_from_anchor: days, 
            update_stamp: now_unix,
            cached_timestamp_in_sec: 0.0,
        }
    }

    #[inline]
    pub fn set_today(&mut self) {
        let now_unix = get_unix_nano();
        let today = chrono::DateTime::from_timestamp_nanos(now_unix as i64).naive_utc().date();
        let days = today.signed_duration_since(EPOCH_ANCHOR.unwrap()).num_days() as u32;
        self.days_from_anchor = days;
        self.update_stamp = now_unix;
    }

    #[inline]
    pub fn increment(&mut self) {
        self.days_from_anchor += 1;
        self.update_stamp = get_unix_nano();
    }

    pub fn get(&self) -> DateStamp {
        DateStamp { days_from_anchor: self.days_from_anchor }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, PartialOrd, Ord)]
pub struct DateStamp {  
    days_from_anchor: u32,
}

impl DateStamp {
    #[inline]
    pub fn new(days: u32) -> Self {
        DateStamp { days_from_anchor: days }
    }

    #[inline]
    pub fn to_date_utc(&self) -> NaiveDate {
        let res = EPOCH_ANCHOR.unwrap() + Duration::days(self.days_from_anchor as i64);
        res
    }

    #[inline]
    pub fn today_as_ordinal() -> u32 {
        let now = Utc::now().date_naive();
        let anchor = EPOCH_ANCHOR.unwrap();
        let days = now.signed_duration_since(anchor).num_days() as u32;
        days
    }

    #[inline]
    pub fn today() -> DateStamp {
        DateStamp::new(Self::today_as_ordinal())
    }

    #[inline]
    pub fn from_generator(generator: &DateStampGenerator) -> DateStamp {
        DateStamp { days_from_anchor: generator.days_from_anchor}
    }

    #[inline]
    pub fn as_real(&self) -> Real {
        self.days_from_anchor as Real
    }

    #[inline]
    pub fn get_days_from_anchor(&self) -> u32 {
        self.days_from_anchor
    }

    #[inline]
    pub fn get_diff(&self, other: DateStamp) -> u32 {
        self.days_from_anchor - other.get_days_from_anchor()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TimeStampType {
    #[default]
    HHMMSSuuuuuu,
    UnixNanoStamp,
}

/// This is a time in seconds from the midnight of the day, hence it can't be bigger than 86,400.0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd)]
pub struct TimeStampInSec { 
    pub stamp: Real,
}

impl Default for TimeStampInSec {
    fn default() -> Self {
        TimeStampInSec { stamp: 0.0 }
    }
}

impl TimeStampInSec {
    pub fn new(stamp: Real) -> Self {
        TimeStampInSec { stamp }
    }

    pub fn as_real(&self) -> Real {
        self.stamp
    }
}


impl AddAssign<TimeStampInSec> for TimeStampInSec {
    fn add_assign(&mut self, other: TimeStampInSec) {
        self.stamp += other.stamp;
    }
}

impl SubAssign<TimeStampInSec> for TimeStampInSec {
    fn sub_assign(&mut self, other: TimeStampInSec) {
        self.stamp -= other.stamp;
    }
}

impl Add<TimeStampInSec> for TimeStampInSec {
    type Output = TimeStampInSec;

    fn add(self, other: TimeStampInSec) -> TimeStampInSec {
        TimeStampInSec { stamp: self.stamp + other.stamp}
    }
}

impl Sub<TimeStampInSec> for TimeStampInSec {
    type Output = TimeStampInSec;

    fn sub(self, other: TimeStampInSec) -> TimeStampInSec {
        TimeStampInSec { stamp: self.stamp - other.stamp}
    }
}

/// For longer period, we can use this
/// But tick like style can't use this if it uses the past data
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DateTimeStampInSec {
    pub date: DateStamp,
    pub time: TimeStampInSec,
}

impl DateTimeStampInSec {
    #[inline]
    pub fn new(date: DateStamp, time: TimeStampInSec) -> Self {
        DateTimeStampInSec { date, time }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_stamp() {
        let mut gen = DateStampGenerator::from_today();
        let date = gen.get();

        let today = DateStamp::today();
        assert_eq!(date, today);

        gen.increment();

        let date = gen.get();

        let tomorrow = chrono::Utc::now().naive_utc().date();
        let days_tomorrow = tomorrow.signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as u32;
        assert_eq!(date.days_from_anchor, days_tomorrow);

    }
}