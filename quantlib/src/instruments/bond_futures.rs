use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BondFutures {
    currency: Currency,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    name: String,
    code: String,
}

impl InstrumentTrait for BondFutures {
    fn get_type_name(&self) -> &'static str {
        "BondFutures"
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }
}
