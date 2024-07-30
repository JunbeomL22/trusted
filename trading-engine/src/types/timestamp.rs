use std::ops::{Add, Sub, AddAssign, SubAssign};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TimeStampType {
    #[default]
    HHMMSSuuuuuu,
    UnixNano,
}

/// It is for saving the memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CoarseTimeStampType {
    #[default]
    HHMMSSmmm,
    UnixSec,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorseTimeStamp {
    pub stamp: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeStamp {
    pub stamp: u64,
}

impl TimeStamp {
    pub fn to_coarse(
        &self,
        stamp_type: TimeStampType,
        coarse_type: CoarseTimeStampType,
    ) -> Result<CorseTimeStamp> {
        match (stamp_type, coarse_type) {
            (TimeStampType::HHMMSSuuuuuu, CoarseTimeStampType::HHMMSSmmm) => {
                Ok(CorseTimeStamp { stamp: (self.stamp / 1000) as u32 })
            },
            (TimeStampType::UnixNano, CoarseTimeStampType::UnixSec) => {
                Ok(CorseTimeStamp { stamp: (self.stamp / 1_000_000) as u32 })
            },
            _ => {
                Err(anyhow::anyhow!("Invalid conversion"))
            },
        }
    }
}

impl Default for TimeStamp {
    fn default() -> Self {
        TimeStamp { stamp: 0 }
    }
}

impl AddAssign<TimeStamp> for TimeStamp {
    fn add_assign(&mut self, other: TimeStamp) {
        self.stamp += other.stamp;
    }
}

impl SubAssign<TimeStamp> for TimeStamp {
    fn sub_assign(&mut self, other: TimeStamp) {
        self.stamp -= other.stamp;
    }
}

impl Add<TimeStamp> for TimeStamp {
    type Output = TimeStamp;

    fn add(self, other: TimeStamp) -> TimeStamp {
        TimeStamp { stamp: self.stamp + other.stamp}
    }
}

impl Sub<TimeStamp> for TimeStamp {
    type Output = TimeStamp;

    fn sub(self, other: TimeStamp) -> TimeStamp {
        TimeStamp { stamp: self.stamp.saturating_sub(other.stamp) }
    }
}

impl TimeStamp {
    pub fn new(stamp: u64) -> Self {
        TimeStamp { stamp }
    }

    #[inline]
    pub fn as_real(&self) -> Real {
        self.stamp as Real
    }

    #[inline]
    pub fn diff_secs(
        &self, 
        other: TimeStamp,
        stamp_type: TimeStampType,
    ) -> Real {
        match stamp_type {
            TimeStampType::UnixNano => {
                (self.stamp - other.stamp) as Real / 1_000_000_000.0
            },
            _ => {
                (self.stamp - other.stamp) as Real / 1_000_000.0
            },
        }
    }
    #[inline]
    pub fn diff_millis(
        &self, 
        other: TimeStamp,
        stamp_type: TimeStampType,
    ) -> Real {
        match stamp_type {
            TimeStampType::UnixNano => {
                (self.stamp - other.stamp) as Real / 1_000_000.0
            },
            _ => {
                (self.stamp - other.stamp) as Real / 1_000.0
            },
        }
    }

    #[inline]
    pub fn diff_micros(
        &self, 
        other: TimeStamp,
        stamp_type: TimeStampType,
    ) -> Real {
        match stamp_type {
            TimeStampType::UnixNano => {
                (self.stamp - other.stamp) as Real / 1_000.0
            },
            _ => { (self.stamp - other.stamp) as Real },
        }
    }
}