use crate::currency::{Currency, FxCode};
use crate::definitions::Real;
use crate::enums::{OptionDailySettlementType, OptionExerciseType, OptionType};
use crate::instrument::InstrumentTrait;
use crate::InstInfo;
//
use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use static_id::StaticId;
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VanillaOption {
    pub inst_info: InstInfo,
    pub strike: Real,
    pub settlement_date: OffsetDateTime,
    pub underlying_ids: Vec<StaticId>,
    pub underlying_currency: Currency,
    pub quanto_fx_code: Option<FxCode>,
    pub option_type: OptionType,
    pub exercise_type: OptionExerciseType,
    pub daily_settlement_type: OptionDailySettlementType,
}

impl Default for VanillaOption {
    fn default() -> VanillaOption {
        let inst_info = InstInfo::default();
        let settlement_date = inst_info.maturity.unwrap().clone();
        VanillaOption {
            inst_info,
            strike: 0.0,
            settlement_date,
            underlying_ids: vec![],
            underlying_currency: Currency::KRW,
            quanto_fx_code: None,
            exercise_type: OptionExerciseType::European,
            option_type: OptionType::Call,
            daily_settlement_type: OptionDailySettlementType::NotSettled,
        }
    }
}

impl VanillaOption {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        strike: Real,
        settlement_date: Option<OffsetDateTime>,
        underlying_id: StaticId,
        underlying_currency: Currency,
        option_type: OptionType,
        exercise_type: OptionExerciseType,
        option_daily_settlement_type: OptionDailySettlementType,
        
    ) -> VanillaOption {
        let currency = inst_info.currency;
        let settlement_date = settlement_date.unwrap_or(inst_info.maturity.unwrap().clone());

        let quanto_fx_code = if currency != underlying_currency {
            Some(FxCode::new(underlying_currency, currency))
        } else {
            None
        };

        let underlying_ids = vec![underlying_id];

        VanillaOption {
            inst_info,
            strike,
            settlement_date,
            underlying_ids,
            underlying_currency,
            quanto_fx_code,
            option_type,
            exercise_type,
            daily_settlement_type: option_daily_settlement_type,
        }
    }

    pub fn get_strike(&self) -> Real {
        self.strike
    }
}

impl InstrumentTrait for VanillaOption {
    fn get_inst_info(&self) -> &InstInfo {
        &self.inst_info
    }

    fn get_type_name(&self) -> &'static str {
        match self.option_type {
            OptionType::Call => "VanillaCall",
            OptionType::Put => "VanillaPut",
        }
    }

    fn get_underlying_currency(&self) -> Result<Currency> {
        Ok(self.underlying_currency)
    }

    fn get_underlying_ids(&self) -> Vec<StaticId> {
        vec![self.underlying_ids[0]]
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

    fn get_quanto_fxcode_und_pair(&self) -> Vec<(StaticId, &FxCode)> {
        match &self.quanto_fx_code {
            Some(fx_code) => vec![(self.underlying_ids[0], fx_code)],
            None => vec![],
        }
    }

    fn get_underlying_ids_requiring_volatility(&self) -> Vec<StaticId> {
        vec![self.underlying_ids[0]]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::enums::{OptionDailySettlementType, OptionExerciseType, OptionType};
    use crate::InstInfo;
    //
    use time::macros::datetime;

    #[test]
    fn test_vanilla_option_new() {
        let id = StaticId::from_str("AAPL", "KIS");
        
        let inst_info = InstInfo::new(
            id, 
            "AAPL".to_string(),
            crate::InstType::VanillaOption,
            Currency::USD,
            1.0,
            Some(datetime!(2021-01-01 00:00:00 +00:00)),
            Some(datetime!(2022-01-01 00:00:00 +00:00)),
            crate::AccountingLevel::L1,
        );

        let und_id = StaticId::from_str("AAPL", "KIS");

        let option = VanillaOption::new(
            inst_info,
            100.0,
            None,
            und_id,
            Currency::USD,
            OptionType::Call,
            OptionExerciseType::European,
            OptionDailySettlementType::NotSettled,
        );

        let ser = serde_json::to_string(&option).unwrap();
        let de: VanillaOption = serde_json::from_str(&ser).unwrap();

        assert_eq!(option, de);

    }
}