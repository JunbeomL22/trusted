use flexstr::LocalStr;
use serde::{Deserialize, Serialize};
use crate::types::enums::TimeStepUnit;
use std::ops::{Add, Sub, AddAssign, SubAssign};

pub type UnixNano = u64;
/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64;

pub type AccountId = LocalStr;

pub type TraderId = LocalStr;
// 수익률
pub type BookYield = i64;
// 가격
pub type BookPrice = i64;
// 수량
pub type BookQuantity = u64;
// 건수
pub type OrderCount = u32;

pub type Real = f32;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MicroTimeStamp{
    // HHMMSSuuuuuu
    pub stamp: u64,
}

impl MicroTimeStamp {
    pub fn new(stamp: u64) -> Self {
        MicroTimeStamp { stamp }
    }
}

impl MicroTimeStamp {
    #[inline]
    pub fn as_real(&self) -> Real {
        self.stamp as Real
    }
}
impl Default for MicroTimeStamp {
    fn default() -> Self {
        MicroTimeStamp { stamp: 0 }
    }
}

impl AddAssign<MicroTimeStamp> for MicroTimeStamp {
    fn add_assign(&mut self, other: MicroTimeStamp) {
        self.stamp += other.stamp;
    }
}

impl SubAssign<MicroTimeStamp> for MicroTimeStamp {
    fn sub_assign(&mut self, other: MicroTimeStamp) {
        self.stamp -= other.stamp;
    }
}

impl Add<MicroTimeStamp> for MicroTimeStamp {
    type Output = MicroTimeStamp;

    fn add(self, other: MicroTimeStamp) -> MicroTimeStamp {
        MicroTimeStamp { stamp: self.stamp + other.stamp}
    }
}

impl Sub<MicroTimeStamp> for MicroTimeStamp {
    type Output = MicroTimeStamp;

    fn sub(self, other: MicroTimeStamp) -> MicroTimeStamp {
        MicroTimeStamp { stamp: self.stamp - other.stamp }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MilliTimeStamp{
    // HHMMSSmmm
    pub stamp: u32,
}

impl MilliTimeStamp {
    pub fn new(stamp: u32) -> Self {
        MilliTimeStamp { stamp }
    }
}
 
impl MilliTimeStamp {
    #[inline]
    pub fn as_real(&self) -> Real {
        self.stamp as Real
    }

    #[inline]
    pub fn to_seconds(&self) -> Real {
        self.stamp as Real / 1000.0
    }

    #[inline]
    pub fn to_micros(&self) -> Real {
        self.stamp as Real * 1000.0
    }
}

impl Default for MilliTimeStamp {
    fn default() -> Self {
        MilliTimeStamp { stamp: 0 }
    }
}

impl AddAssign<MilliTimeStamp> for MilliTimeStamp {
    fn add_assign(&mut self, other: MilliTimeStamp) {
        self.stamp += other.stamp;
    }
}

impl SubAssign<MilliTimeStamp> for MilliTimeStamp {
    fn sub_assign(&mut self, other: MilliTimeStamp) {
        self.stamp -= other.stamp;
    }
}

impl Add<MilliTimeStamp> for MilliTimeStamp {
    type Output = MilliTimeStamp;

    fn add(self, other: MilliTimeStamp) -> MilliTimeStamp {
        MilliTimeStamp { stamp: self.stamp + other.stamp}
    }
}

impl Sub<MilliTimeStamp> for MilliTimeStamp {
    type Output = MilliTimeStamp;

    fn sub(self, other: MilliTimeStamp) -> MilliTimeStamp {
        MilliTimeStamp{ 
            stamp: self.stamp - other.stamp
        }
    }
}

impl MicroTimeStamp {
    /// cut off the last 3 digits, in other words, quotient of 1000
    pub fn to_millis(&self) -> MilliTimeStamp {
        MilliTimeStamp{
            stamp: (self.stamp / 1000) as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CoarseTimeSeriesPoint {
    pub timestamp: MilliTimeStamp, 
    pub value: Real,
}

impl CoarseTimeSeriesPoint {
    pub fn from_timestamp(timestamp: MilliTimeStamp) -> Self {
        CoarseTimeSeriesPoint {
            timestamp,
            value: 0.0,
        }
    }
    
    pub fn null_point() -> Self {
        CoarseTimeSeriesPoint {
            timestamp: MilliTimeStamp { stamp: 0 },
            value: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct FineTimeSeriesPoint {
    pub timestamp: MicroTimeStamp, 
    pub value: Real,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Slice {
    pub start: usize,
    pub end: usize,
}

pub type NormalizedReal = Real;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Quote {
    pub price: NormalizedReal,
    pub quantity: NormalizedReal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LevelSnapshot {
    pub order_count: Option<OrderCount>,
    pub book_price: BookPrice,
    pub book_quantity: BookQuantity,
    pub lp_quantity: Option<BookQuantity>,
}

impl Default for LevelSnapshot {
    fn default() -> Self {
        LevelSnapshot {
            order_count: None,
            book_price: 0,
            book_quantity: 0,
            lp_quantity: None,
        }
    }
}

impl PartialEq for LevelSnapshot {
    fn eq(&self, other: &LevelSnapshot) -> bool {
        self.book_price == other.book_price && 
        self.book_quantity == other.book_quantity &&
        self.order_count == other.order_count && 
        self.lp_quantity == other.lp_quantity
    }
}

impl PartialEq<&LevelSnapshot> for LevelSnapshot {
    fn eq(&self, other: &&LevelSnapshot) -> bool {
        self.book_price == other.book_price && 
        self.book_quantity == other.book_quantity &&
        self.order_count == other.order_count &&
        self.lp_quantity == other.lp_quantity
    }
}

impl PartialEq<LevelSnapshot> for &LevelSnapshot {
    fn eq(&self, other: &LevelSnapshot) -> bool {
        self.book_price == other.book_price && 
        self.book_quantity == other.book_quantity &&
        self.order_count == other.order_count &&
        self.lp_quantity == other.lp_quantity
    }
}

impl Eq for LevelSnapshot {}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeInterval {
    pub unit: TimeStepUnit,
    pub interval: Real,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::TimeStepUnit;

    #[test]
    fn test_micro_to_milli() {
        let micro = MicroTimeStamp{stamp: 123456789};
        let milli = micro.to_millis();
        assert_eq!(milli.stamp, 123456);
    }

    #[test]
    fn test_time_interval() {
        let ti = TimeInterval {
            interval: 1.0,
            unit: TimeStepUnit::Second,
        };
        assert_eq!((ti.interval - 1.0) < Real::EPSILON, true);
        assert_eq!(ti.unit, TimeStepUnit::Second);
    }
}