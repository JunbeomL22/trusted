use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::InstInfo;
//
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Cash {
    pub inst_info: InstInfo,
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

    pub fn new_from_inst_info(inst_info: InstInfo) -> Result<Cash> {
        if inst_info.unit_notional != 1.0 {
            return Err(anyhow::anyhow!("unit_notional must be 1.0 for Cash"));
        }

        if inst_info.issue_date.is_some() {
            return Err(anyhow::anyhow!("issue_date must be None for Cash"));
        }

        if inst_info.maturity.is_some() {
            return Err(anyhow::anyhow!("maturity must be None for Cash"));
        }

        if inst_info.accounting_level != crate::AccountingLevel::L1 {
            return Err(anyhow::anyhow!("accounting_level must be L1 for Cash"));
        }

        Ok(Cash { inst_info })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let cash = Cash::new_from_currency(Currency::USD);
        let s = serde_json::to_string(&cash).unwrap();
        println!("{:?}", s);
        let cash2: Cash = serde_json::from_str(&s).unwrap();
        assert_eq!(cash, cash2);
    }
}
