use serde::{Deserialize, Serialize};

pub trait FromI8 {
    fn from_i8(value: i8) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum OrderSide {
    Buy = 1,
    NoSide = 0,
    Sell = -1,
}

impl FromI8 for OrderSide {
    fn from_i8(v: i8) -> Option<Self> {
        match v {
            1 => Some(OrderSide::Buy),
            0 => Some(OrderSide::NoSide),
            -1 => Some(OrderSide::Sell),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookType { 
    L1 = 1,
    L2 = 2,
    L3 = 3,
}

impl FromI8 for BookType {
    fn from_i8(v: i8) -> Option<Self> {
        match v {
            1 => Some(BookType::L1),
            2 => Some(BookType::L2),
            3 => Some(BookType::L3),
            _ => None,
        }
    }
}

