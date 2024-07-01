use ustr::Ustr;
use serde::{
    Serialize, 
    Deserialize,
    de::Deserializer,
};

/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64; 

pub type AccountId = Ustr;

pub type TraderId = Ustr;