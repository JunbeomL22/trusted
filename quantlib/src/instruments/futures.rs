use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::InstInfo;
//
use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Futures {
    pub inst_info: InstInfo,
    pub average_trade_price: Real,
    pub settlement_date: OffsetDateTime,
    pub underlying_currency: Currency,
    pub underlying_codes: Vec<String>,
}

impl Default for Futures {
    fn default() -> Futures {
        Futures {
            inst_info: InstInfo::default(),
            average_trade_price: 0.0,
            settlement_date: OffsetDateTime::now_utc(),
            underlying_currency: Currency::KRW,
            underlying_codes: vec!["".to_string()],
        }
    }
}

impl Futures {
    //#[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        average_trade_price: Real,
        settlement_date: Option<OffsetDateTime>,
        underlying_currency: Currency,
        underlying_code: String,
    ) -> Futures {
        let settlement_date = match settlement_date {
            Some(date) => date,
            None => inst_info.maturity.unwrap().clone(),
        };

        Futures {
            inst_info,
            average_trade_price,
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
    use crate::{
        InstId,
        InstType,
        IsinCode,
        Symbol,
        Venue,
        AccountingLevel,
    };
    #[test]
    fn test_stock_futures_serialization() {
        let isin_code = IsinCode::new(b"KR7005930003").unwrap();
        let inst_id = InstId::new(Symbol::Isin(isin_code), Venue::KRX);
        let inst_info = InstInfo {
            id: inst_id,
            inst_type: InstType::Futures,
            name: "KOSPI2 Fut Mar24".to_string(),
            currency: Currency::KRW,
            maturity: Some(datetime!(2022-01-01 15:40:00 +09:00)),
            issue_date: Some(datetime!(2021-01-01 09:00:00 +09:00)),
            accounting_level: AccountingLevel::L1,
            unit_notional: 100.0,
        };

        let stock_futures = Futures {
            inst_info: inst_info,
            average_trade_price: 100.0,
            settlement_date: datetime!(2021-01-01 09:00:00 +09:00),
            underlying_currency: Currency::KRW,
            underlying_codes: vec!["KOSPI2".to_string()],
        };

        let serialized = serde_json::to_string(&stock_futures).unwrap();
        let deserialized: Futures = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stock_futures, deserialized);
    }
}
