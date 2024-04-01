use crate::currency::Currency;
use crate::instrument::InstrumentTrait;
use crate::definitions::Real;   
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    name: String,
    code: String,
    currency: Currency,
}

impl Stock {
    pub fn new(name: String, code: String, currency: Currency) -> Stock {
        Stock {
            name,
            code,
            currency,
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
}