use crate::instrument::Instrument;
use crate::instruments::stock_futures::StockFutures;
use std::{any::Any, fmt::Debug};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

pub struct InstrumentInfo {
    instrument: Box<dyn Instrument>,
}

impl Clone for InstrumentInfo {
    fn clone(&self) -> Self {
        Self {
            instrument: self.instrument.clone(),
        }
    }
}

impl PartialEq for InstrumentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.instrument.get_name() == other.instrument.get_name() &&
        self.instrument.get_currency() == other.instrument.get_currency() &&
        self.instrument.get_code() == other.instrument.get_code() &&
        self.instrument.type_name() == other.instrument.type_name()
    }
}

impl Debug for InstrumentInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstrumentInfo")
            .field("Name", &self.instrument.get_name())
            .field("Code", &self.instrument.get_code())
            .field("Currency", &self.instrument.get_currency())
            .field("Type", &self.instrument.type_name())
            .finish()
    }
}

impl InstrumentInfo {
    pub fn new(instrument: Box<dyn Instrument>) -> Self {
        InstrumentInfo { instrument }
    }
    
    pub fn type_name(&self) -> &str {
        self.instrument.type_name()
    }
}

impl Default for InstrumentInfo {
    fn default() -> Self {
        InstrumentInfo {
            instrument: Box::new(StockFutures::default()),
        }
    }
}

// Then you can implement Serialize for your wrapper struct
impl Serialize for InstrumentInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let any = self.instrument.as_any();
        if let Some(stock_futures) = any.downcast_ref::<StockFutures>()
        {
            return StockFutures::serialize(stock_futures, serializer);
        }
        else {
            return Err(serde::ser::Error::custom("Undefined serialization for InstrumentInfo."));
        }
    }
}

// And you can implement Deserialize for your wrapper struct
impl<'de> Deserialize<'de> for InstrumentInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        // Deserialize to StockFutures
        let stock_futures = StockFutures::deserialize(deserializer)?;

        // Return a new InstrumentWrapper
        Ok(InstrumentInfo {
            instrument: Box::new(stock_futures),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use serde_json::{to_string, from_str};
    use crate::assets::currency::Currency;

    #[test]
    fn test_instrument_info_serialization() {
        let stock_futures = StockFutures::new(
            datetime!(2021-01-01 00:00:00 +0000),
            datetime!(2021-01-01 00:00:00 +0000),
            datetime!(2021-01-01 00:00:00 +0000),
            datetime!(2021-01-01 00:00:00 +0000),
            100.0,
            Currency::USD,
            "AAPL".to_string(),
            "NameAAPL".to_string(),
            "CodeAAPL".to_string(),
        );

        let instrument_info = InstrumentInfo::new(Box::new(stock_futures));

        let serialized = to_string(&instrument_info).unwrap();

        println!("serialized = {:?}", serialized);

        let deserialized: InstrumentInfo = from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(instrument_info, deserialized);
    }
}
