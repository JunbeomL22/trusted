use crate::time::calendar_trait::CalendarTrait;
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::enums::RateIndexCode;
use crate::instruments::schedule::BaseSchedule;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::assets::currency::Currency;
use crate::time::jointcalendar::JointCalendar;
use crate::utils::string_arithmetic::{add_period, sub_period};
use crate::enums::Compounding;
use crate::utils::string_arithmetic::from_period_string_to_float;
//
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::{
    rc::Rc,
    cell::RefCell,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
/// * Tenor is forward curve calculation period\n
/// * Compounding_tenor (Option<String>) is the period of compounding\n
/// At each the end date of the previous compounding_tenor, 
/// the rate is computed by the tenor, and compounded by the compounding_tenor.
/// For example, if tenor is 91D and compounding_tenor is 1D, and payment_frequency is 3M,
/// that's the case of KODEX CD ETF (A459580)
/// Theoretically, the compound tenor can be greater than the tenor,
/// but I have not seen such a case in the market, so I chose not to allow such case (return error)
pub struct RateIndex {
    payment_frequency: PaymentFrequency,
    business_day_convention: BusinessDayConvention,
    daycounter: DayCountConvention,
    tenor: String,
    compounding_tenor: Option<String>,
    compounding_fixing_days: Option<i64>,
    calendar: JointCalendar,
    currency: Currency,
    name: String, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
    code: RateIndexCode, 
}

impl RateIndex {
    pub fn new(
        payment_frequency: PaymentFrequency,
        business_day_convention: BusinessDayConvention,
        daycounter: DayCountConvention,
        tenor: String,
        compounding_tenor: Option<String>,
        compounding_fixing_days: Option<i64>,
        calendar: JointCalendar,
        currency: Currency,
        code: RateIndexCode,
        name: String,
    ) -> Result<RateIndex> {
        // compounding_tenor and compounding_fixing_days must be both None or Some
        // if they are both Some, compounding_fixing_days must be greater than 0
        // and compounding_tenor must be less than tenor
        match compounding_tenor.as_ref() {
            Some(comp_tenor) => {
                match compounding_fixing_days {
                    Some(fixing_days) => {
                        if fixing_days <= 0 {
                            return Err(anyhow!(
                                "{}:{} compounding_fixing_days = {}, but it must be greater than 0\n\
                                name: {} code : {}",
                                file!(), line!(), fixing_days, name, code,
                            ));
                        }
                        if from_period_string_to_float(comp_tenor.as_str())? > from_period_string_to_float(tenor.as_str())? {
                            return Err(anyhow!(
                                "{}:{} compounding_tenor = {:?}, but it must be less than tenor = {:?}\n\
                                name: {} code : {}",
                                file!(), line!(), compounding_tenor, tenor, name, code,
                            ));
                        }
                    },
                    None => {
                        return Err(anyhow!(
                            "{}:{} compounding_fixing_days = {:?}, but compounding_tenor = {:?}\n\
                            name: {} code : {}",
                            file!(), line!(), compounding_fixing_days, compounding_tenor,
                            name, code,
                        ));
                    }
                }
            },
            None => {
                match compounding_fixing_days {
                    Some(_) => {
                        return Err(anyhow!(
                            "{}:{} compounding_fixing_days = {:?}, but compounding_tenor = {:?}\n\
                            name: {} code : {}",
                            file!(), line!(), compounding_fixing_days, compounding_tenor,
                            name, code,
                        ));
                    },
                    None => {}
                }
            }
        }

        Ok(RateIndex {
            payment_frequency,
            business_day_convention,
            daycounter,
            calendar,
            tenor,
            compounding_tenor,
            compounding_fixing_days,
            currency,
            code,
            name,
        })
    }

    pub fn get_frequency(&self) -> &PaymentFrequency {
        &self.payment_frequency
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
        forward_curve: &ZeroCurve,
        close_data: &CloseData,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> Result<Real> {
        match self.compounding_tenor.as_ref() {
            None => {
                let fixing_date = base_schedule.get_fixing_date();
                let rate = close_data.get(fixing_date);
                let res = match rate {
                    Some(rate) => *rate,
                    None => {
                        let forward_date = add_period(
                            &forward_curve.get_evaluation_date_clone().borrow().get_date_clone(),
                            self.tenor.as_str(),
                        );

                        forward_curve.get_forward_rate_from_evaluation_date(
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
            },
            Some(comp_tenor) => {
                let eval_date = evaluation_date.borrow().get_date_clone();
                let calc_date = base_schedule.get_calc_start_date();
                let fixing_days = self.compounding_fixing_days.unwrap();
                let fixing_days_string = format!("{}D", fixing_days);
                let fixing_days_str = fixing_days_string.as_str();
                let mut fixing_date = sub_period(&calc_date, fixing_days_str);
                let last_fixing_date = sub_period(&base_schedule.get_calc_end_date(), comp_tenor.as_str());
                let spot_rate = forward_curve.get_short_rate_at_time(0.0)?;
                let mut compounded_value: Real = 1.0;
                let mut rate: Real;
                // compound by 1 + rate * frac
                // frac is the year fraction between the previous fixing date and the current fixing date
                // In case of the fixing date is before the evaluation date,
                // if the fixing_date is in the past_date, use the rate in the past_date
                // otherwise, use the zero rate in the evaluation date
                // if the fixing_date is after the evaluation date, use the simple forward rate from the fixing date to the next fixing date
                // if the fixing date is the last fixing date, then the compounding stops
                while fixing_date < last_fixing_date {
                    let next_fixing_date = self.calendar.adjust(
                        &add_period(&fixing_date, comp_tenor.as_str()),
                        &BusinessDayConvention::Following,
                    );

                    let frac = self.calendar.year_fraction(
                        &fixing_date, 
                        &next_fixing_date,
                        &self.daycounter
                    )?;

                    if fixing_date < eval_date {
                        rate = match close_data.get(&fixing_date) {
                            Some(rate) => *rate,
                            None => {
                                println!(
                                    "{}:{} fixing_date = {:?} is before the evaluation date = {:?}, \
                                    but there is no rate in the fixing date",
                                    file!(), line!(), fixing_date, eval_date
                                );
                                spot_rate
                            },
                        }
                    } else {
                        rate = forward_curve.get_forward_rate_between_dates(
                            &fixing_date,
                            &next_fixing_date,
                            Compounding::Simple,
                        )?;
                    };
                    
                    compounded_value *= 1.0 + rate * frac;
                    fixing_date = next_fixing_date;

                    println!("fixing_date = {:?}", fixing_date);
                }

                let res = compounded_value - 1.0;
                Ok(res)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::time::{
        calendars::{
            southkorea::{SouthKorea, SouthKoreaType},
            unitedstates::{UnitedStates, UnitedStatesType},
        },
        jointcalendar::JointCalendar,
        calendar::Calendar,
        calendar_trait::CalendarTrait,
    };
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use crate::assets::currency::Currency;
    use crate::enums::RateIndexCode;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::data::{
        history_data::CloseData,
        vector_data::VectorData,
    };
    use crate::instruments::schedule::BaseSchedule;
    //
    use time::{
        macros::datetime,
        Duration,
    };
    use anyhow::{Result, anyhow};
    use ndarray::array;
    
    #[test]
    fn test_rate_index() -> Result<()> {
        let dt = datetime!(2024-01-02 16:30:00 -05:00);
        let evaluation_date = Rc::new(RefCell::new(
            EvaluationDate::new(dt.clone())
        ));
        let payment_frequency = PaymentFrequency::Quarterly;
        let business_day_convention = BusinessDayConvention::ModifiedFollowing;
        let daycounter = DayCountConvention::Actual360;
        let tenor = "1D".to_string();
        let compounding_tenor = Some("1D".to_string());
        let compounding_fixing_days = Some(7);
        let calendar = JointCalendar::new(
            vec![
                Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Sofr, false)),
                ]
            )?;
        let currency = Currency::USD;
        let code = RateIndexCode::SOFR;
        let name = "SOFR1D".to_string();
        let rate_index = RateIndex::new(
            payment_frequency,
            business_day_convention,
            daycounter,
            tenor,
            compounding_tenor,
            compounding_fixing_days,
            calendar.clone(),
            currency,
            code,
            name,
        )?;

        let curve_data = VectorData::new(
            array![0.03, 0.03],
            None,
            Some(array![0.2, 0.5]),
            dt.clone(),
            Currency::USD,
            "USDOIS".to_string(),
        )?;
        
        let zero_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &curve_data,
            "USDOIS".to_string(),
            "USDOIS".to_string(),
        )?;

        let base_schedule = BaseSchedule::new(
            calendar.adjust(&(dt - Duration::days(7)), &BusinessDayConvention::Following),
            dt,
            dt + Duration::days(90),
            dt + Duration::days(90),
            None,
        );

        let mut history_map = HashMap::new();
        history_map.insert(datetime!(2024-01-01 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-31 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-30 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-29 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-28 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-27 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-26 16:30:00 -05:00), 0.03);
        
        let close_date = CloseData::new(
            history_map,
            "SOFR1D".to_string(),
            "SOFR1D".to_string(),
        );

        let compound = rate_index.get_coupon_amount(
            &base_schedule,
            &zero_curve,
            &close_date,
            evaluation_date.clone(),
        )?;

        println!("coumpound = {}", compound);

        Ok(())
    }
}