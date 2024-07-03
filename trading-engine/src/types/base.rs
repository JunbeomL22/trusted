use ustr::Ustr;
use serde::{Serialize, Deserialize};

/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64; 

pub type AccountId = Ustr;

pub type TraderId = Ustr;

#[derive(Default, Debug, Clone, Serialize, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumReprCfg {
    pub digit_length: u8,
    pub decimal_point_length: u8,
    pub include_negative: bool,
    pub total_length: u8,
}

pub type BookPrice = i64;

pub type BookQuantity = u64;
