use crate::instrument::InstrumentTrait;
use crate::InstInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BondFutures {
    pub inst_info: InstInfo,
}

impl BondFutures {
    pub fn new(inst_info: InstInfo) -> Self {
        BondFutures {
            inst_info,
        }
    }
}

impl InstrumentTrait for BondFutures {
    fn get_inst_info(&self) ->  &InstInfo {
        &self.inst_info
    }
}
