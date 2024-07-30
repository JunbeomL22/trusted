use crate::types::isin_code::IsinCode;
use crate::types::enums::TimeStepUnit;
use crate::types::venue::Venue;
use crate::types::timestamp::{CorseTimeStamp, MicroTimeStamp, TimeStamp};
use std::ops::{Index, IndexMut};
use flexstr::LocalStr;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, AddAssign, SubAssign};

/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualOrderId {
    pub order_id: OrderId,
}

impl Default for VirtualOrderId {
    fn default() -> Self {
        VirtualOrderId { order_id: 0 }
    }
}

impl VirtualOrderId {
    pub fn new(order_id: OrderId) -> Self {
        VirtualOrderId { order_id }
    }
    pub fn next(&mut self) -> OrderId {
        let res = self.order_id;
        self.order_id += 1;
        res
    }
}
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
//
pub type Real = f32;
/

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradeHistory {
    history: Vec<(BookPrice, BookQuantity)>,
}

impl Default for TradeHistory {
    fn default() -> Self {
        TradeHistory {
            history: Vec::new(),
        }
    }
}

impl TradeHistory {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        TradeHistory {
            history: Vec::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        TradeHistory {
            history: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.history.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    #[inline]
    pub fn push(&mut self, price: BookPrice, quantity: BookQuantity) {
        self.history.push((price, quantity));
    }

    #[inline]
    #[must_use]
    pub fn get_last_trade(&self) -> Option<(BookPrice, BookQuantity)> {
        self.history.last().cloned()
    }

    #[inline]
    #[must_use]
    pub fn average_trade_price(&self) -> Real {
        let mut sum = 0;
        let mut total_quantity = 0;
        for (price, quantity) in self.history.iter() {
            sum += price * (*quantity) as BookPrice;
            total_quantity += quantity;
        }
        sum as Real / total_quantity as Real
    }
}

// Implementing Index trait for immutable indexing
impl Index<usize> for TradeHistory {
    type Output = (BookPrice, BookQuantity);

    fn index(&self, index: usize) -> &Self::Output {
        &self.history[index]
    }
}

// Implementing IndexMut trait for mutable indexing
impl IndexMut<usize> for TradeHistory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.history[index]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CoarseTimeSeriesPoint {
    pub timestamp: CorseTimeStamp, 
    pub value: Real,
}

impl TimeSeriesPoint {
    pub fn from_timestamp(timestamp: TimeStamp) -> Self {
        TimeSeriesPoint {
            timestamp,
            value: 0.0,
        }
    }
    
    pub fn null_point() -> Self {
        TimeSeriesPoint {
            timestamp: TimeStamp { stamp: 0 },
            value: 0.0,
        }
    }
}

#[warn(dead_code)]
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

#[warn(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeInterval {
    pub unit: TimeStepUnit,
    pub interval: Real,
}

#[warn(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstrumentIdentifier {
    isin: IsinCode, 
    venue: Venue,
}

impl InstrumentIdentifier {
    pub fn new(isin: IsinCode, venue: Venue) -> Self {
        InstrumentIdentifier {
            isin,
            venue,
        }
    }

    #[inline]
    #[must_use]
    pub fn get_isin_code(&self) -> &IsinCode {
        &self.isin
    }

    #[inline]
    #[must_use]
    pub fn get_venue(&self) -> Venue {
        self.venue
    }
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