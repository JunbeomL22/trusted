use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::types::isin_code::IsinCode;
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
    Ask = 1,
    Bid = 2,
}

impl FromU8 for OrderSide {
    fn from_u8(v: u8) -> Result<Self> {
        match v {
            1 => Ok(OrderSide::Ask),
            2 => Ok(OrderSide::Bid),
            _ => Err(anyhow!("Invalid OrderSide in from_u8")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DataType {
    L1 = 1,
    #[default]
    L2 = 2,
    L3 = 3,
}

impl FromU8 for DataType {
    fn from_u8(v: u8) -> Result<Self> {
        match v {
            1 => Ok(DataType::L1),
            2 => Ok(DataType::L2),
            3 => Ok(DataType::L3),
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
pub enum Currency {
    #[default]
    KRW,
    USD,
    EUR,
    GBP,
    JPY,
    CNY,
    HKD,
}

impl Currency {
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::KRW => "KRW",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CNY => "CNY",
            Currency::HKD => "HKD",
        }
    }

    #[inline]
    #[must_use]
    pub fn as_isin_bytes(&self) -> &'static [u8; 12] {
        match self {
            Currency::KRW => b"KRW000000000",
            Currency::USD => b"USD000000000",
            Currency::EUR => b"EUR000000000",
            Currency::GBP => b"GBP000000000",
            Currency::JPY => b"JPY000000000",
            Currency::CNY => b"CNY000000000",
            Currency::HKD => b"HKD000000000",
        }
    }

    #[inline]
    #[must_use]
    pub fn as_isin_code(&self) -> IsinCode {
        IsinCode::new(self.as_isin_bytes()).expect("failed to create IsinCode from Currency")
    }
}

