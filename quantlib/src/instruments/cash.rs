use crate::definitions::Real;
use crate::currency::Currency;
use crate::instrument::InstrumentTrait;
//
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cash {
    amount: Real,
    currency: Currency,
    name: String,
}

impl Cash {
    pub fn new(amount: Real, currency: Currency) -> Self {
        Cash { 
            amount, 
            currency,
            name: currency.as_str().to_string()
        }
    }
}

impl InstrumentTrait for Cash {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.name
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_unit_notional(&self) -> Real {
        self.amount
    }

    fn get_type_name(&self) -> &'static str {
        "Cash"
    }
}