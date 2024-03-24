use crate::definitions::Real;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use crate::assets::currency::Currency;
//
use crate::instrument::InstrumentTrait;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct StockFutures {
    average_trade_price: Real,
    first_trade_date: OffsetDateTime,
    last_trade_date: OffsetDateTime,
    maturity: OffsetDateTime,
    settlement_date: OffsetDateTime,
    unit_notional: Real,
    currency: Currency,
    underlying_codes: Vec<String>,
    name: String,
    code: String,
}

impl Default for StockFutures {
    fn default() -> StockFutures {
        StockFutures {
            average_trade_price: 0.0,
            first_trade_date: OffsetDateTime::now_utc(),
            last_trade_date: OffsetDateTime::now_utc(),
            maturity: OffsetDateTime::now_utc(),
            settlement_date: OffsetDateTime::now_utc(),
            unit_notional: 0.0,
            currency: Currency::KRW,
            underlying_codes: vec![],
            name: String::from(""),
            code: String::from(""),
        }
    }
}

impl StockFutures {
    pub fn new(
        average_trade_price: Real,
        first_trade_date: OffsetDateTime,
        last_trade_date: OffsetDateTime,
        maturity: OffsetDateTime,
        settlement_date: OffsetDateTime,
        unit_notional: Real,
        currency: Currency,
        futures_currency: Currency,
        underlying_code: String,
        name: String,
        code: String,
    ) -> StockFutures {
        StockFutures {
            average_trade_price,
            first_trade_date,
            last_trade_date,
            maturity,
            settlement_date,
            unit_notional,
            currency,
            underlying_codes: vec![underlying_code],
            name,
            code,
        }
    }

}

impl InstrumentTrait for StockFutures {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_type_name(&self) -> &'static str {
        "StockFutures"
    }

    fn get_underlying_codes(&self) -> Vec<&String> {
        let underlying_codes_ref: Vec<&String> = self.underlying_codes.iter().collect();
        underlying_codes_ref
    }

    fn get_average_trade_price(&self) -> Real {
        self.average_trade_price
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
            100.0,
            datetime!(2021-01-01 09:00:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            100.0,
            Currency::KRW,
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