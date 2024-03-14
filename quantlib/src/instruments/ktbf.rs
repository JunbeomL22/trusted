use crate::assets::currency::Currency;
use crate::definitions::Real;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::instruments::bonds::bond::Bond;
use crate::time::conventions::{DayCountConvention, PaymentFrequency, BusinessDayConvention};

pub struct VirtualBond {
    coudpon: Real,
    payment_frequency: PaymentFrequency,
    maturity: OffsetDateTime,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct KTBF {
    currency: Currency,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    settlement_date: OffsetDateTime,
    virtual_bond: Bond,
    underlying_bonds: Vec<Bond>,
    name: String,
    code: String,
}

impl KTBF {
    pub fn new(
        currency: Currency,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        maturity: OffsetDateTime,
        settlement_date: OffsetDateTime,
        virtual_bond: Bond,
        underlying_bonds: Vec<Bond>,
        name: String,
        code: String,
    ) -> KTBF {
        KTBF {
            currency,
            unit_notional,
            issue_date,
            maturity,
            settlement_date,
            virtual_bond,
            underlying_bonds,
            name,
            code,
        }
    }

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