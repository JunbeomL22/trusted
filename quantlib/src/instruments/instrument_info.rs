use crate::instrument::Instrument;
use crate::instruments::stock_futures::StockFutures;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Serialize, Deserialize)]
pub struct InstrumentInfo {
    instrument: Box<dyn Instrument>,
}

impl InstrumentInfo {
    pub fn new(instrument: Box<dyn Instrument>) -> Self {
        InstrumentInfo { instrument }
    }
}

// Then you can implement Serialize for your wrapper struct
impl Serialize for InstrumentInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        if let Some(stock_futures) = self.instrument.downcast_ref::<StockFutures>() {
            // If the downcast is successful, serialize the StockFutures
            return StockFutures::serialize(stock_futures, serializer);
        }
        // If the downcast was not successful, return an error
        Err(serde::ser::Error::custom("Object was not of type StockFutures"))
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

    #[test]
    fn test_instrument_info_serialization() {
        let stock_futures = StockFutures::new(
            datetime!(2021-01-01 00:00:00 +0000),
            datetime!(2021-01-01 00:00:00 +0000),
            datetime!(2021-01-01 00:00:00 +0000),
            datetime!(2021-01-01 00:00:00 +0000),
            100.0,
            "USD".to_string(),
            "AAPL".to_string(),
            "AAPL".to_string(),
        );
        let instrument_info = InstrumentInfo::new(Box::new(stock_futures));
        let serialized = to_string(&instrument_info).unwrap();
        println!("serialized = {:?}", serialized);
        let deserialized: InstrumentInfo = from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        assert_eq!(instrument_info.instrument, deserialized.instrument);
    }
}
