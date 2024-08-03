use crate::types::base::Real;
use crate::utils::timer::get_unix_nano;
use serde::{Serialize, Deserialize};

use time::{
    Date,
    OffsetDateTime,
    macros::date,
};
use anyhow::Result;

pub type UnixNano = u64;

pub const MICRO_NANOSCALE: u64 = 1_000;
pub const MILLI_NANOSCALE: u64 = 1_000_000;
pub const SECOND_NANOSCALE: u64 = 1_000_000_000;
pub const MINUTE_NANOSCALE: u64 = 60 * SECOND_NANOSCALE;
pub const HOUR_NANOSCALE: u64 = 60 * MINUTE_NANOSCALE;
pub const DAY_NANOSCALE: u64 = 24 * HOUR_NANOSCALE;
pub const FIFTEEN_HOURS_NANOSCALE: u64 = 15 * HOUR_NANOSCALE; // check whether the date is in the same day
const EPOCH_DATE: Date = date!(1970-01-01);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub enum TimeScale {
    #[default]
    Micro,
    Milli,
    Second,
    Minute,
    Hour,
    Day,
}

/// made for pparsing each exchange protocol
#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct DateUnixNanoGenerator {
    pub utcdate_unix_nano: UnixNano, // only count the date part
    pub prev_timestamp: UnixNano,
    update_stamp: UnixNano,
}

impl std::fmt::Debug for DateUnixNanoGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let utcdate = OffsetDateTime::from_unix_timestamp_nanos(self.utcdate_unix_nano as i128).unwrap().date();
        let prev_datetime = OffsetDateTime::from_unix_timestamp_nanos(self.prev_timestamp as i128).unwrap();
        let update_datetime = OffsetDateTime::from_unix_timestamp_nanos(self.update_stamp as i128).unwrap();
        write!(f, "DateUnixNanoGenerator (converted) {{ utcdate_unix_nano: {}, prev_timestamp: {}, update_stamp: {} }}", utcdate, prev_datetime, update_datetime)
    }
}

impl From<time::Date> for DateUnixNanoGenerator {
    fn from(dt: time::Date) -> Self {
        let utc_nano = (dt - EPOCH_DATE).whole_nanoseconds() as u64;
        // get time in secs <= 86,400
        DateUnixNanoGenerator { 
            utcdate_unix_nano: utc_nano,
            update_stamp: get_unix_nano(),
            prev_timestamp: 0,
        }
    }
}

impl DateUnixNanoGenerator {
    #[inline]
    #[must_use]
    pub fn get_prev_timestamp(&self) -> UnixNano {
        self.prev_timestamp
    }

    #[inline]
    pub fn cache_timestamp(&mut self, stamp: UnixNano) {
        self.prev_timestamp = stamp;
    }

    #[inline]
    pub fn from_unixnano(unix_nano: UnixNano) -> Self {
        let utcdate_unix_nano = (unix_nano / DAY_NANOSCALE) * DAY_NANOSCALE;
        DateUnixNanoGenerator { 
            utcdate_unix_nano,
            update_stamp: get_unix_nano(),
            prev_timestamp: 0,
        }
    }

    pub fn from_today() -> Result<Self> {
        let now_unix = get_unix_nano();
        let utcdate_unix_nano = (now_unix / DAY_NANOSCALE) * DAY_NANOSCALE;
        
        Ok(DateUnixNanoGenerator { 
            utcdate_unix_nano,
            update_stamp: now_unix,
            prev_timestamp: 0,
        })
    }

    /// this can save around 7ns comparing to using set_today
    #[inline]
    pub fn set_timestamp(&mut self, timestamp: TimeStamp) {
        let unixnano = timestamp.stamp;
        let utcdate_unix_nano = ( unixnano / DAY_NANOSCALE) * DAY_NANOSCALE;
        self.utcdate_unix_nano = utcdate_unix_nano;
        self.update_stamp = unixnano;
    }

    #[inline]
    pub fn set_today(&mut self) {
        let now_unix = get_unix_nano();
        let utcdate_unix_nano = (now_unix / DAY_NANOSCALE) * DAY_NANOSCALE;
        self.utcdate_unix_nano = utcdate_unix_nano;
        self.update_stamp = now_unix;
    }

    #[inline]
    pub fn increment(&mut self, update_timestamp: UnixNano) {
        self.utcdate_unix_nano += DAY_NANOSCALE;
        self.update_stamp = update_timestamp;
    }

    #[inline]
    #[must_use]
    pub fn get_utcdate_unix_nano(&self) -> UnixNano {
        self.utcdate_unix_nano
    }

    #[inline]
    #[must_use]
    pub fn get_update_stamp(&self) -> UnixNano {
        self.update_stamp
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, PartialOrd, Ord)]
pub struct TimeStamp {
    pub stamp: UnixNano,
}

impl std::fmt::Debug for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datetime = OffsetDateTime::from_unix_timestamp_nanos(self.stamp as i128).unwrap();
        write!(f, "TimeStamp (converted) {{ stamp: {} }}", datetime)
    }
}

impl TimeStamp {
    #[inline]
    pub fn to_datetime(&self) -> Result<OffsetDateTime> {
        let res = OffsetDateTime::from_unix_timestamp_nanos(self.stamp as i128)?;
        Ok(res)
    }
    
    #[inline]
    #[must_use]
    pub fn new(unix_nano: UnixNano) -> Self {
        TimeStamp { stamp: unix_nano }
    }

    #[inline]
    pub fn to_date_utc(&self) -> Result<Date> {
        let res = OffsetDateTime::from_unix_timestamp_nanos(self.stamp as i128)?.date();
        Ok(res)
    }

    #[inline(always)]
    fn div_rem(dividend: UnixNano, divisor: UnixNano) -> (UnixNano, UnixNano) {
        let quotient = dividend / divisor;
        let remainder = dividend - (quotient * divisor);
        (quotient, remainder)
    }

    #[inline(always)]
    pub fn diff_in_nanos(&self, other: TimeStamp) -> Real {
        (self.stamp - other.stamp) as Real
    }

    #[inline(always)]
    pub fn diff_in_micros(&self, other: TimeStamp) -> Real {
        let (quotient, remainder) = Self::div_rem(self.stamp - other.stamp, MICRO_NANOSCALE);
        quotient as Real + (remainder as Real / MICRO_NANOSCALE as Real)
    }

    #[inline(always)]
    pub fn diff_in_millis(&self, other: TimeStamp) -> Real {
        let (quotient, remainder) = Self::div_rem(self.stamp - other.stamp, MILLI_NANOSCALE);
        quotient as Real + (remainder as Real / MILLI_NANOSCALE as Real)
    }

    #[inline(always)]
    pub fn diff_in_secs(&self, other: TimeStamp) -> Real {
        let (quotient, remainder) = Self::div_rem(self.stamp - other.stamp, SECOND_NANOSCALE);
        quotient as Real + (remainder as Real / SECOND_NANOSCALE as Real)
    }

    #[inline(always)]
    pub fn diff_in_mins(&self, other: TimeStamp) -> Real {
        let (quotient, remainder) = Self::div_rem(self.stamp - other.stamp, MINUTE_NANOSCALE);
        quotient as Real + (remainder as Real / MINUTE_NANOSCALE as Real)
    }

    #[inline(always)]
    pub fn diff_in_hours(&self, other: TimeStamp) -> Real {
        let (quotient, remainder) = Self::div_rem(self.stamp - other.stamp, HOUR_NANOSCALE);
        quotient as Real + (remainder as Real / HOUR_NANOSCALE as Real)
    }

    #[inline(always)]
    pub fn diff_in_days(&self, other: TimeStamp) -> Real {
        let (quotient, remainder) = Self::div_rem(self.stamp - other.stamp, DAY_NANOSCALE);
        quotient as Real + (remainder as Real / DAY_NANOSCALE as Real)
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TimeStampType {
    #[default]
    HHMMSSuuuuuu,
    UnixNanoStamp,
}
