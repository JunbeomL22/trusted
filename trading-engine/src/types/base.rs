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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OrderBase {
    pub order_count: OrderCount,
    pub book_price: BookPrice,
    pub book_quantity: BookQuantity,
}