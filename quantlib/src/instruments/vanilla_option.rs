use crate::currency::{Currency, FxCode};
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::enums::{OptionType, OptionDailySettlementType, OptionExerciseType};
//
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use anyhow::Result;
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VanillaOption {
    strike: Real,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    last_trade_date: OffsetDateTime,
    maturity: OffsetDateTime,
    settlement_date: OffsetDateTime,
    underlying_codes: Vec<String>,
    underlying_currency: Currency,
    currency: Currency,
    quanto_fx_code: Option<FxCode>,
    option_type: OptionType,
    exercise_type: OptionExerciseType,
    daily_settlement_type: OptionDailySettlementType,
    name: String,
    code: String,
}

impl Default for VanillaOption {
    fn default() -> VanillaOption {
        VanillaOption {
            strike: 0.0,
            unit_notional: 0.0,
            issue_date: OffsetDateTime::now_utc(),
            last_trade_date: OffsetDateTime::now_utc(),
            maturity: OffsetDateTime::now_utc(),
            settlement_date: OffsetDateTime::now_utc(),
            underlying_codes: vec![],
            underlying_currency: Currency::KRW,
            currency: Currency::KRW,
            quanto_fx_code: None,
            exercise_type: OptionExerciseType::European,
            option_type: OptionType::Call,
            daily_settlement_type: OptionDailySettlementType::NotSettled,
            name: String::from(""),
            code: String::from(""),
        }
    }
}

impl VanillaOption {
    pub fn new(
        strike: Real,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        last_trade_date: OffsetDateTime,
        maturity: OffsetDateTime,
        settlement_date: OffsetDateTime,
        underlying_codes: Vec<String>,
        underlying_currency: Currency,
        currency: Currency,
        option_type: OptionType,
        exercise_type: OptionExerciseType,
        option_daily_settlement_type: OptionDailySettlementType,
        name: String,
        code: String,
    ) -> VanillaOption {
        let quanto_fx_code = if currency != underlying_currency {
            Some(FxCode::new(underlying_currency.clone(), currency.clone()))
        } else {
            None
        };

        VanillaOption {
            strike,
            unit_notional,
            issue_date,
            last_trade_date,
            maturity,
            settlement_date,
            underlying_codes,
            underlying_currency,
            quanto_fx_code,
            currency,
            option_type,
            exercise_type,
            daily_settlement_type: option_daily_settlement_type,
            name,
            code,
        }
    }

    pub fn get_strike(&self) -> Real {
        self.strike
    }
}

impl InstrumentTrait for VanillaOption {
    fn get_name(&self) -> &String {
        &self.name
    }
    
    fn get_type_name(&self) -> &'static str {
        match self.option_type {
            OptionType::Call => "VanillaCall",
            OptionType::Put => "VanillaPut"
        }
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }
    fn get_code(&self) -> &String {
        &self.code
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

    fn get_underlying_codes(&self) -> Vec<&String> {
        vec![&self.underlying_codes[0]]
    }

    fn get_option_type(&self) -> Result<OptionType> {
        Ok(self.option_type)
    }

    fn get_option_daily_settlement_type(&self) -> Result<OptionDailySettlementType> {
        Ok(self.daily_settlement_type)
    }

    fn get_strike(&self) -> Result<Real> {
        Ok(self.strike)
    }

    fn get_quanto_fxcode_und_pair(&self) -> Vec<(&String, &FxCode)> {
        match &self.quanto_fx_code {
            Some(fx_code) => vec![(&self.underlying_codes[0], fx_code)],
            None => vec![],
        }
    }

    fn get_underlying_codes_requiring_volatility(&self) -> Vec<&String> {
        vec![&self.underlying_codes[0]]
    }
}
