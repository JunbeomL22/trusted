use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::types::base::Real;
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
pub enum QuoteDataType {
    #[default]
    Price,
    Quantity,
    OrderCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TimeStampType {
    #[default]
    HHMMSSuuuuuu,
    UnixNano,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TimeStepUnit {
    #[default]
    Tick = 0,
    Nano = 1,
    Micro = 2,
    Milli = 3,
    Second = 4,
    Minute = 5,
    Hour = 6,
    Day = 7,
}

impl TimeStepUnit {
    pub fn to_millis(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Nano => Ok(0.000_001),
            TimeStepUnit::Micro => Ok(0.001),
            TimeStepUnit::Milli => Ok(1.0),
            TimeStepUnit::Second => Ok(1_000.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_millis")),
        }
    }

    pub fn to_micros(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Nano => Ok(0.001),
            TimeStepUnit::Micro => Ok(1.0),
            TimeStepUnit::Milli => Ok(1_000.0),
            TimeStepUnit::Second => Ok(1_000_000.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_micros")),
        }
    }

    pub fn to_nanos(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Nano => Ok(1.0),
            TimeStepUnit::Micro => Ok(1_000.0),
            TimeStepUnit::Milli => Ok(1_000_000.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_nanos")),
        }
    }

    pub fn to_seconds(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Micro => Ok(0.000_001),
            TimeStepUnit::Milli => Ok(0.001),
            TimeStepUnit::Second => Ok(1.0),
            TimeStepUnit::Minute => Ok(60.0),
            TimeStepUnit::Hour => Ok(3_600.0),
            TimeStepUnit::Day => Ok(86_400.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_seconds")),
        }
    }

    pub fn to_minutes(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Milli => Ok(0.000_016_666_666_666_666),
            TimeStepUnit::Second => Ok(0.016_666_666_666_666),
            TimeStepUnit::Minute => Ok(1.0),
            TimeStepUnit::Hour => Ok(60.0),
            TimeStepUnit::Day => Ok(1_440.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_minutes")),
        }
    }

    pub fn to_hours(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Second => Ok(0.000_277_777_777_777_777_8),
            TimeStepUnit::Minute => Ok(0.016_666_666_666_666),
            TimeStepUnit::Hour => Ok(1.0),
            TimeStepUnit::Day => Ok(24.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_hours")),
        }
    }

    pub fn to_days(&self) -> Result<Real> {
        match self {
            TimeStepUnit::Tick => Ok(0.0),
            TimeStepUnit::Second => Ok(0.000_011_574_074_074_074_073),
            TimeStepUnit::Minute => Ok(0.000_694_444_444_444_444_5),
            TimeStepUnit::Hour => Ok(0.041_666_666_666_666_664),
            TimeStepUnit::Day => Ok(1.0),
            _ => Err(anyhow!("Invalid TimeStepUnit in to_days")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_me_the_memory() {
        println!("Size of TimeStampType: {}", std::mem::size_of::<TimeStampType>());
    }
}
