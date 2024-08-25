use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::InstInfo;
//
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Cash {
    inst_info: InstInfo,
}

impl Cash {
    pub fn new_from_currency(currency: Currency) -> Cash {
        let inst_info = InstInfo {
            name: currency.as_str().to_string(),
            currency: currency,
            ..Default::default()
        };
        Cash { inst_info }
    }
}

impl InstrumentTrait for Cash {
    fn get_inst_info(&self) -> &InstInfo {
        &self.inst_info
    }
    
    fn get_type_name(&self) -> &'static str {
        "Cash"
    }

    fn get_unit_notional(&self) -> Real {
        1.0
    }
}
