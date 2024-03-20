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
use crate::utils::string_arithmetic::add_period;
use crate::enums::Compounding;
use crate::utils::string_arithmetic::from_period_string_to_float;
//
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::{
    rc::Rc,
    cell::RefCell,
};
use time::{Duration, OffsetDateTime};


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
    calc_day_convention: BusinessDayConvention,
    daycounter: DayCountConvention,
    curve_tenor: String,
    compounding_tenor: Option<String>,
    fixing_days: i64,
    calendar: JointCalendar,
    currency: Currency,
    name: String, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
    code: RateIndexCode, 
}

impl RateIndex {
    pub fn new(
        payment_frequency: PaymentFrequency,
        calc_day_convention: BusinessDayConvention,
        daycounter: DayCountConvention,
        curve_tenor: String,
        compounding_tenor: Option<String>,
        fixing_days: i64,
        calendar: JointCalendar,
        currency: Currency,
        code: RateIndexCode,
        name: String,
    ) -> Result<RateIndex> {

        Ok(RateIndex {
            payment_frequency,
            calc_day_convention,
            daycounter,
            calendar,
            curve_tenor,
            compounding_tenor,
            fixing_days,
            currency,
            code,
            name,
        })
    }

    pub fn get_frequency(&self) -> &PaymentFrequency {
        &self.payment_frequency
    }

    pub fn get_fixing_days(&self) -> i64 {
        self.fixing_days
    }

    pub fn get_calc_day_convention(&self) -> &BusinessDayConvention {
        &self.calc_day_convention
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

    pub fn get_curve_tenor(&self) -> &String {
        &self.curve_tenor
    }

    // if CloseDate has rate in the fixing date, return the rate
    // otherwise, it retuns zero rate in the evaluation date
    pub fn get_coupon_amount(
        &self,
        base_schedule: &BaseSchedule,
        spread: Option<Real>,
        forward_curve: &ZeroCurve,
        close_data: &CloseData,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> Result<Real> {
        let spread = spread.unwrap_or(0.0);
        match self.compounding_tenor.as_ref() {
            None => {// None means that it is not a overnight type index
                let eval_dt = evaluation_date.borrow().get_date_clone();
                let fixing_date = base_schedule.get_fixing_date();
                let curve_end_date = add_period(&eval_dt, self.curve_tenor.as_str());
                let res: Real;
                if fixing_date < &eval_dt {
                    res = match close_data.get(fixing_date) {
                        Some(rate) => *rate,
                        None => {
                            println!(
                                "{}:{} fixing_date = {:?} is before the evaluation date = {:?}, \
                                but there is no rate in the fixing date",
                                file!(), line!(), fixing_date, eval_dt
                            );

                            forward_curve.get_forward_rate_from_evaluation_date(
                                &curve_end_date, Compounding::Simple,
                            )?
                        },
                    };                    
                } else { // fixing_date >= eval_dt
                    res = forward_curve.get_forward_rate_between_dates(
                        &fixing_date, 
                        &curve_end_date, 
                        Compounding::Simple,
                    )?;
                }
                    
                let frac = self.calendar.year_fraction(
                    base_schedule.get_calc_start_date(),
                    base_schedule.get_calc_end_date(),
                    &self.daycounter,
                )?;
                
                Ok((res + spread) * frac)
            },
            Some(comp_tenor) => {
                let eval_date = evaluation_date.borrow().get_date_clone();
                let curve_end_date_from_eval_date = add_period(&eval_date, &comp_tenor.as_str());
                let mut calc_start_date = base_schedule.get_calc_start_date().clone();
                let mut next_calc_date: OffsetDateTime;
                let calc_end_date = base_schedule.get_calc_end_date().clone();
                let fixing_days = self.fixing_days;
                
                let spot_rate = forward_curve.get_forward_rate_between_dates(
                    &eval_date,
                    &curve_end_date_from_eval_date,
                    Compounding::Simple,
                )?;

                let mut compounded_value: Real = 1.0;
                let mut rate: Real;
                let mut fixing_date: OffsetDateTime;
                let mut frac: Real;
                
                while calc_start_date < calc_end_date {
                    fixing_date = self.calendar.adjust(
                        &(calc_start_date - Duration::days(fixing_days)),
                        &BusinessDayConvention::Preceding,
                    );

                    next_calc_date = add_period(&calc_start_date, &comp_tenor.as_str());

                    frac = self.calendar.year_fraction(
                        &calc_start_date,
                        &next_calc_date,
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
                            &add_period(&fixing_date, &self.curve_tenor.as_str()),
                            Compounding::Simple,
                        )?;
                    };
                    
                    compounded_value *= 1.0 + (rate + spread) * frac;
                 
                    calc_start_date = next_calc_date.clone();
                 
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
        let daycounter = DayCountConvention::Actual365Fixed;
        let tenor = "1D".to_string();
        let compounding_tenor = Some("1D".to_string());
        let fixing_days = 7;
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
            fixing_days,
            calendar.clone(),
            currency,
            code,
            name,
        )?;

        let curve_data = VectorData::new(
            array![0.03, 0.03],
            None,
            Some(array![0.1, 0.2]),
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

        let calc_end_date = dt + Duration::days(90);
        let base_schedule = BaseSchedule::new(
            calendar.adjust(&(dt.clone() - Duration::days(7)), &BusinessDayConvention::Preceding),
            dt.clone(),
            calc_end_date.clone(),
            calc_end_date.clone(),
            None,
        );

        let mut history_map = HashMap::new();
        //history_map.insert(datetime!(2023-12-29 16:30:00 -05:00), 0.06);
        history_map.insert(datetime!(2023-12-29 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-28 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-27 16:30:00 -05:00), 0.03);
        history_map.insert(datetime!(2023-12-26 16:30:00 -05:00), 0.03);
        
        let close_date = CloseData::new(
            history_map,
            "SOFR1D".to_string(),
            "SOFR1D".to_string(),
        );

        let rate = rate_index.get_coupon_amount(
            &base_schedule,
            None,
            &zero_curve,
            &close_date,
            evaluation_date.clone(),
        )?;

        let expected_rate: Real =  0.03010;

        let yearly_rate: Real = rate * 365.0 / 90.0;
        println!("expected_rate = {}, yearly_rate = {}", expected_rate, yearly_rate);
        assert!(
            (yearly_rate - expected_rate).abs() < 1e-4,
            "rate = {}, expected_rate = {}",
            yearly_rate,
            expected_rate
        );
        Ok(())
    }
}