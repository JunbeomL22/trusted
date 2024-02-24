use crate::definitions::Real;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use crate::parameters::currency::Currency;
use crate::instrument::Instrument;
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockFutures {
    strike: Real,
    listing_date: OffsetDateTime,
    last_trada_date: OffsetDateTime,
    settlement_date: OffsetDateTime,
    unit_notional: Real,
    currency: Currency,
    underlyings: Vec<String>,
    name: String,
    code: String,
}

impl EquityFutures {
    pub fn new(
        strike: Real,
        maturity: OffsetDateTime,
        unit_notional: Real,
        currency: Currency,
        underlying: String,
    ) -> EquityFutures {
        EquityFutures {
            strike,
            maturity,
            unit_notional,
            currency,
            vec![underlying],

        }
    }

    pub fn get_strike(&self) -> Real {
        self.strike
    }

    pub fn get_maturity(&self) -> &OffsetDateTime {
        &self.maturity
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_underlying_asset(&self) -> &Vec<String> {
        &self.underlying_asset
    }
}

impl Instrument for EquityFutures {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }
}
// make a test for serialization
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equity_futures_serialization() {
        let equity_futures = EquityFutures::new(
            OffsetDateTime::now(), 
            250_000.0, 
            Currency::KRW, 
            "KOSPI200".to_string()
        );
        let serialized = to_string(&equity_futures).unwrap();
        println!("{:?}", serialized);
        let deserialized: EquityFutures = from_str(&serialized).unwrap();

        assert_eq!(deserialized, equity_futures);
    }

    #[test] // json serialization
    fn test_equity_futures_json() {
        let equity_futures = EquityFutures::new(
            OffsetDateTime::now(), 
            250_000.0, 
            Currency::KRW, 
            "KOSPI200".to_string()
        );
        let json = json!(equity_futures);
        println!("{:?}", json);
        let deserialized: EquityFutures = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized, equity_futures);
    }
}
