use static_id::StaticId;
use crate::definitions::Real;
use crate::enums::StockRankType;
use crate::instrument::InstrumentTrait;
use serde::{Deserialize, Serialize};
use crate::InstInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Stock {
    pub inst_info: InstInfo,
    pub underlying_ids: Vec<StaticId>,
    pub rank_type: StockRankType,
}

impl Stock {
    pub fn new(
        inst_info: InstInfo,
        underlying_id: StaticId,
        rank_type: Option<StockRankType>,
    ) -> Stock {
        let rank_type = rank_type.unwrap_or(StockRankType::Undefined);
        let underlying_ids = vec![underlying_id];
        Stock {
            inst_info,
            underlying_ids,
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

    fn get_underlying_ids(&self) -> Vec<StaticId> {
        vec![self.underlying_ids[0]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::InstInfo;

    #[test]
    fn test_stock() {
        let id = StaticId::from_str("stock1", "KRX");

        let inst_info = InstInfo::new(
            id,
            "stock1".to_string(),
            crate::InstType::Stock,
            Currency::KRW,
            1.0,
            None,
            None,
            crate::AccountingLevel::L1,
        );

        let id = StaticId::default();
        
        let stock = Stock::new(inst_info, id, None);
        
        let ser = serde_json::to_string(&stock).unwrap();
        let de: Stock = serde_json::from_str(&ser).unwrap();
        assert_eq!(stock, de);
    }
}