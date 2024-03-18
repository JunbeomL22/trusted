use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::enums::RateIndexCode;
use crate::instruments::schedule::BaseSchedule;
use crate::parameters::zero_curve::ZeroCurve;
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::assets::currency::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateIndex {
    frequency: PaymentFrequency,
    business_day_convention: BusinessDayConvention,
    daycounter: DayCountConvention,
    fixing_days: i64,
    tenor: String,
    currency: Currency,
    name: String, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
    code: RateIndexCode, 
}

impl RateIndex {
    pub fn new(
        frequency: PaymentFrequency,
        business_day_convention: BusinessDayConvention,
        daycounter: DayCountConvention,
        fixing_days: i64,
        tenor: String,
        currency: Currency,
        code: RateIndexCode,
        name: String,
    ) -> RateIndex {
        RateIndex {
            frequency,
            business_day_convention,
            daycounter,
            fixing_days,
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

    pub fn get_tenor(&self) -> &String {
        &self.tenor
    }

    pub fn get_fixing_days(&self) -> i64 {
        self.fixing_days
    }

    pub fn get_coupon_amount(
        &self,
        base_schedule: &BaseSchedule,
        zero_curve: &ZeroCurve,
        close_data: &CloseData,
    ) -> Real {

    }
}


