use crate::definitions::Real;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use crate::assets::currency::Currency;
use crate::util::type_name;
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct StockFutures {
    average_trade_price: Real,
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
            underlying_names: vec![],
            name: "".to_string(),
            code: "".to_string(),
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
        underlying_name: String,
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
            underlying_names: vec![underlying_name],
            name,
            code,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_underlying_names(&self) -> Option<&Vec<String>> {
        Some(&self.underlying_names)
    }

    pub fn get_average_trade_price(&self) -> Real {
        self.average_trade_price
    }

    pub fn get_type_name(&self) -> &'static str {
        type_name(&self)
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
            "KOSPI200".to_string(),
            "KOSPI2 Fut Mar24".to_string(),
            "165AAA".to_string(),
        );

        let serialized = serde_json::to_string(&stock_futures).unwrap();
        let deserialized: StockFutures = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stock_futures, deserialized);
    }
}