use crate::assets::currency::Currency;
use crate::definitions::Real;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::parameters::rate_index::RateIndex;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IRS {
    currency: Currency,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    rate_index: RateIndex,
    name: String,
    code: String,
}

impl IRS {
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

    pub fn get_rate_index(&self) -> &RateIndex {
        &self.rate_index
    }

    pub fn get_rate_forward_curve_name(&self) -> Option<&'static str> {
        Some(self.rate_index.get_rate_forward_curve_name())
    }
}