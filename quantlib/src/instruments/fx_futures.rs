use crate::definitions::Real;
use crate::currency::{Currency, FxCode};
use crate::instrument::InstrumentTrait;
//
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FxFutures {
    average_trade_price: Real,
    first_trade_date: OffsetDateTime,
    last_trade_date: OffsetDateTime,
    maturity: OffsetDateTime,
    settlement_date: OffsetDateTime,
    unit_notional: Real,
    underlying_currency: Currency,
    currency: Currency,
    fx_code: FxCode,
    name: String,
    code: String,
}

impl Default for FxFutures {
    fn default() -> FxFutures {
        FxFutures {
            average_trade_price: 0.0,
            first_trade_date: OffsetDateTime::now_utc(),
            last_trade_date: OffsetDateTime::now_utc(),
            maturity: OffsetDateTime::now_utc(),
            settlement_date: OffsetDateTime::now_utc(),
            unit_notional: 0.0,
            underlying_currency: Currency::USD,
            currency: Currency::KRW,
            name: String::from(""),
            code: String::from(""),
            fx_code: FxCode::default(),
        }
    }
}

impl FxFutures {
    pub fn new(
        average_trade_price: Real,
        first_trade_date: OffsetDateTime,
        last_trade_date: OffsetDateTime,
        maturity: OffsetDateTime,
        settlement_date: OffsetDateTime,
        unit_notional: Real,
        currency: Currency,
        underlying_currency: Currency,
        name: String,
        code: String,
    ) -> FxFutures {
        let fx_code = FxCode::new(
            underlying_currency.clone(),
            currency.clone(),
        );
        FxFutures {
            average_trade_price,
            first_trade_date,
            last_trade_date,
            maturity,
            settlement_date,
            unit_notional,
            currency,
            underlying_currency,
            fx_code,
            name,
            code,
        }
    }
}

impl InstrumentTrait for FxFutures {
    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_type_name(&self) -> &'static str {
        "FxFutures"
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_underlying_currency(&self) -> Result<&Currency> {
        Ok(&self.underlying_currency)
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_average_trade_price(&self) -> Real {
        self.average_trade_price
    }

    fn get_fxfutres_und_fxcode(&self) -> Result<&FxCode> {
        Ok(&self.fx_code)
    }

    fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> {
        vec![self.fx_code.clone()]
    }

}