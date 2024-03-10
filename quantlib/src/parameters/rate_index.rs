use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::enums::RateIndexCode;
use crate::assets::currency::Currency;
use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateIndex {
    frequency: PaymentFrequency,
    business_day_convention: BusinessDayConvention,
    daycounter: DayCountConvention,
    tenor: Duration,
    currency: Currency,
    code: RateIndexCode,
    name: String, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
}

impl RateIndex {
    pub fn new(
        frequency: PaymentFrequency,
        business_day_convention: BusinessDayConvention,
        daycounter: DayCountConvention,
        tenor: Duration,
        currency: Currency,
        code: RateIndexCode,
        name: String,
    ) -> RateIndex {
        RateIndex {
            frequency,
            business_day_convention,
            daycounter,
            tenor,
            currency,
            code,
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_rate_index_code(&self) -> &RateIndexCode {
        &self.code
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub fn get_daycounter(&self) -> &DayCountConvention {
        &self.daycounter
    }

    pub fn get_tenor(&self) -> &Duration {
        &self.tenor
    }
}


