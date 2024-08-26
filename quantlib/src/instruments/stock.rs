use crate::currency::Currency;
use crate::definitions::Real;
use crate::enums::StockRankType;
use crate::instrument::InstrumentTrait;
use serde::{Deserialize, Serialize};
use crate::InstInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Stock {
    inst_info: InstInfo,
    underlying_codes: Vec<String>,
    rank_type: StockRankType,
}

impl Stock {
    pub fn new(
        inst_info: InstInfo,
        underlying_codes: Vec<String>,
        currency: Currency,
        rank_type: Option<StockRankType>,
    ) -> Stock {
        let rank_type = rank_type.unwrap_or(StockRankType::Undefined);
        Stock {
            name,
            code,
            underlying_codes,
            currency,
            rank_type,
        }
    }
}

impl InstrumentTrait for Stock {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_type_name(&self) -> &'static str {
        "Stock"
    }

    fn get_unit_notional(&self) -> Real {
        1.0
    }

    fn get_underlying_codes(&self) -> Vec<&String> {
        vec![&self.underlying_codes[0]]
    }
}
