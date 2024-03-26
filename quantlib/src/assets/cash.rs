use crate::definitions::Real;
use crate::assets::currency::Currency;
//
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cash {
    amount: Real,
    currency: Currency,
}

impl Cash {
    pub fn new(amount: Real, currency: Currency) -> Self {
        Cash { amount, currency }
    }

    pub fn get_amount(&self) -> Real {
        self.amount
    }

    pub fn get_currency(&self) -> Currency {
        self.currency
    }
}