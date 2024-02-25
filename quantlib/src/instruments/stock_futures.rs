use crate::definitions::Real;
use time::{OffsetDateTime, format_description};
use serde::{Deserialize, Serialize};
use crate::assets::currency::Currency;
use crate::instrument::Instrument;
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct StockFutures {
    first_trade_date: OffsetDateTime,
    last_trade_date: OffsetDateTime,
    maturity: OffsetDateTime,
    settlement_date: OffsetDateTime,
    unit_notional: Real,
    currency: Currency,
    underlying_names: Vec<String>,
    name: String,
    code: String,
}

impl StockFutures {
    pub fn new(
        first_trade_date: OffsetDateTime,
        last_trade_date: OffsetDateTime,
        maturity: OffsetDateTime,
        settlement_date: OffsetDateTime,
        unit_notional: Real,
        currency: Currency,
        underlying_name: String,
        name: String,
        code: String,
    ) -> StockFutures {
        StockFutures {
            first_trade_date,
            last_trade_date,
            maturity,
            settlement_date,
            unit_notional,
            currency,
            underlying_names: vec![underlying_name],
            name,
            code,
        }
    }

    pub fn get_maturity(&self) -> &OffsetDateTime {
        &self.maturity
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_underlying_asset(&self) -> &Vec<String> {
        &self.underlying_names
    }
}

impl Instrument for StockFutures {
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
    use time::macros::datetime;
    #[test]
    fn test_stock_futures_serialization() {
        let stock_futures = StockFutures::new(
            datetime!(2021-01-01 09:00:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            100.0,
            Currency::KRW,
            "KOSPI200".to_string(),
            "KOSPI2 Fut Mar24".to_string(),
            "165AAA".to_string(),
        );

        let serialized = serde_json::to_string(&stock_futures).unwrap();
        let deserialized: StockFutures = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stock_futures, deserialized);
    }
}