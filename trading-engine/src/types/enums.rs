use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
pub trait FromU8 {
    fn from_u8(v: u8) -> Result<Self>
    where
        Self: Sized;
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default, PartialOrd, Ord,
)]
pub enum OrderSide {
    #[default]
    NoSide = 0,
    Ask = 1,
    Bid = 2,
}

impl FromU8 for OrderSide {
    fn from_u8(v: u8) -> Result<Self> {
        match v {
            0 => Ok(OrderSide::NoSide),
            1 => Ok(OrderSide::Ask),
            2 => Ok(OrderSide::Bid),
            _ => Err(anyhow!("Invalid OrderSide in from_u8")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BookType {
    L1 = 1,
    #[default]
    L2 = 2,
    L3 = 3,
}

impl FromU8 for BookType {
    fn from_u8(v: u8) -> Result<Self> {
        match v {
            1 => Ok(BookType::L1),
            2 => Ok(BookType::L2),
            3 => Ok(BookType::L3),
            _ => Err(anyhow!("Invalid BookType in from_u8")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TradeType {
    #[default]
    Undefined = 0,
    Sell = 1,
    Buy = 2,
}

impl FromU8 for TradeType {
    fn from_u8(v: u8) -> Result<Self> {
        match v {
            0 => Ok(TradeType::Undefined),
            1 => Ok(TradeType::Sell),
            2 => Ok(TradeType::Buy),
            _ => Err(anyhow!("Invalid TradeType in from_u8")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum QuoteDataType {
    #[default]
    Price,
    Quantity,
    OrderCount,
}
