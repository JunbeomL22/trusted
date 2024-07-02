use ustr::Ustr;
use serde::{Serialize, Deserialize};

/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64; 

pub type AccountId = Ustr;

pub type TraderId = Ustr;

#[derive(Debug, Clone, Serialize, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberRepresentationConfigurations {
    pub precision: u8,
    pub length: u8,
}

pub type BookPrice = i64;

pub type BookQuantity = u64;
