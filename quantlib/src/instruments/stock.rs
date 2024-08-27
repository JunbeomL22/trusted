use crate::currency::Currency;
use crate::definitions::Real;
use crate::enums::StockRankType;
use crate::instrument::InstrumentTrait;
use serde::{Deserialize, Serialize};
use crate::InstInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Stock {
    pub inst_info: InstInfo,
    pub underlying_codes: Vec<String>,
    pub rank_type: StockRankType,
}

impl Stock {
    pub fn new(
        inst_info: InstInfo,
        underlying_codes: Vec<String>,
        rank_type: Option<StockRankType>,
    ) -> Stock {
        let rank_type = rank_type.unwrap_or(StockRankType::Undefined);
        Stock {
            inst_info,
            underlying_codes,
            rank_type,
        }
    }
}

impl InstrumentTrait for Stock {
    fn get_inst_info(&self) ->  &InstInfo {
        &self.inst_info
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::InstInfo;
    use crate::enums::StockRankType;

    #[test]
    fn test_stock() {
        let instid = crate::InstId::new(
            crate::Symbol::Isin(crate::IsinCode::new(b"stock1").unwrap()),
            crate::Venue::KRX,
        );

        let inst_info = InstInfo::new(
            instid,
            "stock1".to_string(),
            crate::InstType::Stock,
            Currency::KRW,
            1.0,
            None,
            None,
            crate::AccountingLevel::L1,
        );

        

        let underlying_codes = vec!["stock2".to_string()];
        let stock = Stock::new(inst_info, underlying_codes, None);
        
        let ser = serde_json::to_string(&stock).unwrap();
        let de: Stock = serde_json::from_str(&ser).unwrap();
        assert_eq!(stock, de);
    }
}