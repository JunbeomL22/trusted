use std::fmt::Debug;
use crate::assets::currency::Currency;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstrumentInfo {
    name: String,
    code: String,
    currency: Currency,
    instrument_type: String,
    maturity: Option<OffsetDateTime>,
}

impl Default for InstrumentInfo {
    fn default() -> InstrumentInfo {
        InstrumentInfo {
            name: "".to_string(),
            code: "".to_string(),
            currency: Currency::NIL,
            instrument_type: "".to_string(),
            maturity: None,
        }
    }
}

impl InstrumentInfo {
    pub fn new(
        name: String,
        code: String,
        currency: Currency,
        instrument_type: String,
        maturity: Option<OffsetDateTime>,
    ) -> InstrumentInfo {
        InstrumentInfo {
            name,
            code,
            currency,
            instrument_type,
            maturity,
        }
    }
    
    pub fn type_name(&self) -> &str {
        &self.instrument_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::assets::currency::Currency;

    #[test]
    fn test_instrument_info_serialization() {
        let instrument_info = InstrumentInfo::new(
            "AAPL".to_string(),
            "CodeAAPL".to_string(),
            Currency::USD,
            "StockFutures".to_string(),
            None,
        );

        let serialized = serde_json::to_string_pretty(&instrument_info).unwrap();

        println!("serialized = {}", serialized);

        let deserialized: InstrumentInfo = serde_json::from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(instrument_info, deserialized);
    }
}
