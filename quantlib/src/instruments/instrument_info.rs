use std::fmt::Debug;
use crate::assets::currency::Currency;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::definitions::Real;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstrumentInfo {
    name: String,
    code: String,
    instrument_type: String,
    currency: Currency,
    unit_notional: Real,
    maturity: Option<OffsetDateTime>,
}

impl Default for InstrumentInfo {
    fn default() -> InstrumentInfo {
        InstrumentInfo {
            name: "".to_string(),
            code: "".to_string(),
            instrument_type: "".to_string(),
            currency: Currency::NIL,
            unit_notional: 1.0,
            maturity: None,
        }
    }
}

impl InstrumentInfo {
    pub fn new(
        name: String,
        code: String,
        instrument_type: String,
        currency: Currency,
        unit_notional: Real,
        maturity: Option<&OffsetDateTime>,
    ) -> InstrumentInfo {
        let maturity = match maturity {
            Some(maturity) => Some(maturity.clone()),
            None => None,
        };

        InstrumentInfo {
            name,
            code,
            instrument_type,
            currency,
            unit_notional,
            maturity,
        }
    }
    
    pub fn type_name(&self) -> &String {
        &self.instrument_type
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_currency(&self) -> Currency {
        self.currency
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_maturity(&self) -> Option<&OffsetDateTime> {
        self.maturity.as_ref()
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
            "StockFutures".to_string(),
            Currency::USD,
            100.0,
            None,
        );

        let serialized = serde_json::to_string_pretty(&instrument_info).unwrap();

        println!("serialized = {}", serialized);

        let deserialized: InstrumentInfo = serde_json::from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(instrument_info, deserialized);
    }
}
