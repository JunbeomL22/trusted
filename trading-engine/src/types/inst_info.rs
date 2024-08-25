use crate::Currency;
use crate::InstId;
use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InstType {
    #[default]
    Cash,
    Stock,
    Bond,
    EquityFutures,
    FxFutures,
    CommodityFutures,
    EquityOption,
    Swap,
    Crypto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BaseInstInfo {
    pub inst_type: InstType,
    pub currency: Currency,
}

impl BaseInstInfo {
    pub fn new(inst_type: InstType, currency: Currency) -> Self {
        BaseInstInfo {
            inst_type,
            currency,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BaseInstMap {
    pub map: FxHashMap<InstId, BaseInstInfo>,    
}

impl BaseInstMap {
    pub fn new() -> Self {
        BaseInstMap {
            map: FxHashMap::default(),
        }
    }

    pub fn insert(&mut self, inst_id: InstId, inst_info: BaseInstInfo) {
        self.map.insert(inst_id, inst_info);
    }

    pub fn get(&self, inst_id: InstId) -> Option<&BaseInstInfo> {
        self.map.get(&inst_id)
    }
}
