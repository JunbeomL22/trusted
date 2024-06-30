use crate::currency::Currency;
use crate::definitions::Real;
use crate::utils::number_format::write_number_with_commas;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;


#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

impl std::fmt::Debug for InstrumentInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "    name: {:?},", self.name)?;
        writeln!(f, "    code: {:?},", self.code)?;
        writeln!(f, "    instrument_type: {:?},", self.instrument_type)?;
        writeln!(f, "    currency: {:?},", self.currency)?;
        write!(f, "    unit_notional: ")?;
        write_number_with_commas(f, self.unit_notional)?;
        writeln!(f)?;
        if let Some(maturity) = self.maturity {
            writeln!(f, "    maturity: {:?}", maturity.date())
        } else {
            writeln!(f, "    maturity: None")
        }
    }
}

impl InstrumentInfo {
    pub fn new(
        name: String,
        code: String,
        instrument_type: &'static str,
        currency: Currency,
        unit_notional: Real,
        maturity: Option<&OffsetDateTime>,
    ) -> InstrumentInfo {
        let maturity = maturity.cloned();

        InstrumentInfo {
            name,
            code,
            instrument_type: instrument_type.to_string(),
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

    use crate::currency::Currency;

    #[test]
    fn test_instrument_info_serialization() {
        let instrument_info = InstrumentInfo::new(
            "AAPL".to_string(),
            "CodeAAPL".to_string(),
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
