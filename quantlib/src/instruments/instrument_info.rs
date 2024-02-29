use std::fmt::Debug;
use crate::assets::currency::Currency;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::definitions::Real;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstrumentInfo<'a> {
    name: &'a str,
    code: &'a str,
    instrument_type: &'a str,
    currency: Currency,
    unit_notional: Real,
    maturity: Option<OffsetDateTime>,
}

impl<'a> Default for InstrumentInfo<'a> {
    fn default() -> InstrumentInfo<'a> {
        InstrumentInfo {
            name: "",
            code: "",
            instrument_type: "",
            currency: Currency::NIL,
            unit_notional: 1.0,
            maturity: None,
        }
    }
}

impl<'a> InstrumentInfo<'a> {
    pub fn new(
        name: &'a str,
        code: &'a str,
        instrument_type: &'a str,
        currency: Currency,
        unit_notional: Real,
        maturity: Option<&OffsetDateTime>,
    ) -> InstrumentInfo<'a> {
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
    
    pub fn type_name(&self) -> &str {
        self.instrument_type
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_code(&self) -> &str {
        self.code
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
            "AAPL",
            "CodeAAPL",
            "StockFutures",
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
