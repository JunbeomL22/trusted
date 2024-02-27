use crate::assets::currency::Currency;
use crate::definitions::Real;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedCouponBond {
    currency: Currency,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    name: String,
    code: String,
}

impl FixedCouponBond {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_maturity(&self) -> &OffsetDateTime {
        &self.maturity
    }
}