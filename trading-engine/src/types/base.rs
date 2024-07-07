use flexstr::LocalStr;
use serde::{Deserialize, Serialize};

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
pub type OrderCount = u64;

#[derive(Debug, Clone, Copy)]
pub struct Slice {
    pub start: usize,
    pub end: usize,
}