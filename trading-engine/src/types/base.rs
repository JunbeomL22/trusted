use flexstr::LocalStr;
use serde::{Deserialize, Serialize};

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

pub type TimeStamp = u64;

pub type TimeSeriesPoint = (TimeStamp, Real);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Slice {
    pub start: usize,
    pub end: usize,
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
