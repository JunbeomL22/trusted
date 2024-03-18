use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::enums::RateIndexCode;
use crate::instruments::schedule::BaseSchedule;
use crate::parameters::zero_curve::ZeroCurve;
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::assets::currency::Currency;
use crate::time::jointcalendar::JointCalendar;
use crate::utils::string_arithmetic::add_period;
use crate::enums::Compounding;
//
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateIndex {
    frequency: PaymentFrequency,
    business_day_convention: BusinessDayConvention,
    daycounter: DayCountConvention,
    tenor: String,
    calendar: JointCalendar,
    currency: Currency,
    name: String, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
    code: RateIndexCode, 
}

impl RateIndex {
    pub fn new(
        frequency: PaymentFrequency,
        business_day_convention: BusinessDayConvention,
        daycounter: DayCountConvention,
        tenor: String,
        calendar: JointCalendar,
        currency: Currency,
        code: RateIndexCode,
        name: String,
    ) -> RateIndex {
        RateIndex {
            frequency,
            business_day_convention,
            daycounter,
            calendar,
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

    // if CloseDate has rate in the fixing date, return the rate
    // otherwise, it retuns zero rate in the evaluation date
    pub fn get_coupon_amount(
        &self,
        base_schedule: &BaseSchedule,
        zero_curve: &ZeroCurve,
        close_data: &CloseData,
    ) -> Result<Real> {
        let fixing_date = base_schedule.get_fixing_date();
        let rate = close_data.get(fixing_date);
        let res = match rate {
            Some(rate) => *rate,
            None => {
                let forward_date = add_period(
                    &zero_curve.get_evaluation_date_clone().borrow().get_date_clone(),
                    self.tenor.as_str(),
                );

                zero_curve.get_forward_rate_from_evaluation_date(
                    &forward_date,
                    Compounding::Simple, 
                )?
            }
        };
        let frac = self.calendar.year_fraction(
            base_schedule.get_calc_start_date(),
            base_schedule.get_calc_end_date(),
            &self.daycounter,
        )?;
        
        Ok(res * frac)
    }
}


