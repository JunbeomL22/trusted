use crate::time::conventions::{PaymentFrequency, BusinessDayConvention};
use crate::parameters::zero_curve::ZeroCurve;
use std::rc::Rc;
use std::cell::RefCell;
use crate::enums::RateIndexCode;
use crate::assets::currency::Currency;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RateIndex<'a> {
    frequency: PaymentFrequency,
    business_day_convention: BusinessDayConvention,
    currency: Currency,
    code: RateIndexCode,
    forward_curve: Rc<RefCell<ZeroCurve>>,
    name: &'a str, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
}

impl<'a> RateIndex<'a> {
    pub fn new(
        frequency: PaymentFrequency,
        business_day_convention: BusinessDayConvention,
        currency: Currency,
        code: RateIndexCode,
        forward_curve: Rc<RefCell<ZeroCurve>>,
        name: &'a str,
    ) -> RateIndex {
        RateIndex {
            frequency,
            business_day_convention,
            currency,
            code,
            forward_curve, 
            name,
        }
    }

    pub fn get_frequency(&self) -> &PaymentFrequency {
        &self.frequency
    }

    pub fn get_business_day_convention(&self) -> &BusinessDayConvention {
        &self.business_day_convention
    }

    pub fn get_code(&self) -> &RateIndexCode {
        &self.code
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_rate_index_code(&self) -> &RateIndexCode {
        &self.code
    }

    pub fn get_rate_forward_curve_name(&self) -> &str {
        self.code.get_forward_curve_name()
    }

}


