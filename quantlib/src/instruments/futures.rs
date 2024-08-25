use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::InstInfo;
//
use anyhow::{
    anyhow,
    Result
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Futures {
    pub inst_info: InstInfo,
    pub average_trade_price: Real,
    pub first_trade_date: OffsetDateTime,
    pub last_trade_date: OffsetDateTime,
    pub settlement_date: OffsetDateTime,
    pub underlying_currency: Currency,
    pub underlying_codes: Vec<String>,
}

impl Default for Futures {
    fn default() -> Futures {
        Futures {
            inst_info: InstInfo::default(),
            average_trade_price: 0.0,
            first_trade_date: OffsetDateTime::now_utc(),
            last_trade_date: OffsetDateTime::now_utc(),
            settlement_date: OffsetDateTime::now_utc(),
            underlying_currency: Currency::KRW,
            underlying_codes: vec![],
        }
    }
}

impl Futures {
    //#[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        average_trade_price: Real,
        first_trade_date: OffsetDateTime,
        last_trade_date: OffsetDateTime,
        settlement_date: OffsetDateTime,
        underlying_currency: Currency,
        underlying_code: String,
    ) -> Futures {
        Futures {
            inst_info,
            average_trade_price,
            first_trade_date,
            last_trade_date,
            settlement_date,
            underlying_currency,
            underlying_codes: vec![underlying_code],
        }
    }
}

impl InstrumentTrait for Futures {
    fn get_inst_info(&self) ->  &InstInfo {
        &self.inst_info
    }

    fn get_currency(&self) -> Currency {
        self.currency
    }

    fn get_underlying_currency(&self) -> Result<Currency> {
        Ok(self.underlying_currency)
    }

    fn get_type_name(&self) -> &'static str {
        "Futures"
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
        let stock_futures = Futures::new(
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
        let deserialized: Futures = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stock_futures, deserialized);
    }
}
