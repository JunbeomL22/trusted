use crate::currency::{Currency, FxCode};
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::InstInfo;

//
use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FxFutures {
    pub inst_info: InstInfo,
    pub average_trade_price: Real,
    pub settlement_date: OffsetDateTime,
    pub underlying_currency: Currency,
    pub fx_code: FxCode,
}

impl Default for FxFutures {
    fn default() -> FxFutures {
        FxFutures {
            inst_info: InstInfo::default(),
            average_trade_price: 0.0,
            settlement_date: OffsetDateTime::now_utc(),
            underlying_currency: Currency::KRW,
            fx_code: FxCode::default(),
        }
    }
}

impl FxFutures {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        average_trade_price: Real,
        settlement_date: Option<OffsetDateTime>,
        underlying_currency: Currency,
    ) -> FxFutures {
        let settlement_date = match settlement_date {
            Some(date) => date,
            None => inst_info.maturity.unwrap().clone(),
        };

        let fx_code = FxCode::new(underlying_currency, inst_info.currency);
        FxFutures {
            inst_info,
            average_trade_price,
            settlement_date,
            underlying_currency,
            fx_code,
        }
    }
}

impl InstrumentTrait for FxFutures {
    fn get_inst_info(&self) ->  &InstInfo {
        &self.inst_info
    }

    fn get_type_name(&self) -> &'static str {
        "FxFutures"
    }

    fn get_underlying_currency(&self) -> Result<Currency> {
        Ok(self.underlying_currency)
    }

    fn get_average_trade_price(&self) -> Real {
        self.average_trade_price
    }

    fn get_fxfutres_und_fxcode(&self) -> Result<FxCode> {
        Ok(self.fx_code)
    }

    fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> {
        vec![self.fx_code]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::{
        InstType,
        ID,
        Symbol,
        Ticker,
        Venue,
        AccountingLevel,
    };
    use time::macros::datetime;

    #[test]
    fn test_serde() {
        let inst_info = InstInfo::new(
            ID::new(Symbol::Ticker(Ticker::new(b"USD_KRW").unwrap()), Venue::KRX),
            String::from("USD_KRW"),
            InstType::FxFutures,
            Currency::KRW,
            1.0,
            Some(datetime!(2021-07-01 00:00:00 +00:00)),
            Some(datetime!(2022-07-01 00:00:00 +00:00)),
            AccountingLevel::L1,
        );

        let fxfutures = FxFutures::new(
            inst_info,
            100.0,
            None,
            Currency::KRW,
        );

        let serialized = serde_json::to_string(&fxfutures).unwrap();
        let deserialized: FxFutures = serde_json::from_str(&serialized).unwrap();
        assert_eq!(fxfutures, deserialized);
    }
}