use flexstr::LocalStr;
use serde::{Deserialize, Serialize};

pub type UnixNano = u64;
/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64;

pub type AccountId = LocalStr;

pub type TraderId = LocalStr;
// 가격
pub type BookPrice = i64;
// 수량
pub type BookQuantity = u64;
// 건수
pub type OrderCount = u32;

#[derive(Debug, Clone, Copy)]
pub struct Slice {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct QuoteBase {
    pub order_count: OrderCount,
    pub book_price: BookPrice,
    pub book_quantity: BookQuantity,
}

impl PartialEq for QuoteBase {
    fn eq(&self, other: &QuoteBase) -> bool {
        self.book_price == other.book_price && 
        self.book_quantity == other.book_quantity &&
        self.order_count == other.order_count
    }
}

impl PartialEq<&QuoteBase> for QuoteBase {
    fn eq(&self, other: &&QuoteBase) -> bool {
        self.book_price == other.book_price && 
        self.book_quantity == other.book_quantity &&
        self.order_count == other.order_count
    }
}

impl PartialEq<QuoteBase> for &QuoteBase {
    fn eq(&self, other: &QuoteBase) -> bool {
        self.book_price == other.book_price && 
        self.book_quantity == other.book_quantity &&
        self.order_count == other.order_count
    }
}

impl Eq for QuoteBase {}
