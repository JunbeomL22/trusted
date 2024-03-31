use crate::time::calendar_trait::CalendarTrait;
use crate::time::conventions::{BusinessDayConvention, DayCountConvention};
use crate::enums::RateIndexCode;
use crate::instruments::schedule::BaseSchedule;
use crate::parameters::zero_curve::ZeroCurve;
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::assets::currency::Currency;
use crate::time::jointcalendar::JointCalendar;
use crate::utils::string_arithmetic::add_period;
use crate::enums::Compounding;
use crate::util::min_offsetdatetime;
//
use serde::{Deserialize, Serialize};
use anyhow::Result;
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
    curve_tenor: String,
    currency: Currency,
    name: String, // USD LIBOR 3M, EURIBOR 6M, CD91, etc
    code: RateIndexCode, 
}

impl RateIndex {
    pub fn new(
        curve_tenor: String,
        currency: Currency,
        code: RateIndexCode,
        name: String,
    ) -> Result<RateIndex> {

        Ok(RateIndex {
            curve_tenor,
            currency,
            code,
            name,
        })
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

    pub fn get_curve_tenor(&self) -> &String {
        &self.curve_tenor
    }

    /// base_schedule (BaseSchedule): fixing_date, calc_start_date, calc_end_date, payment_date, amount (Option)
    /// spread (Option<Real>): spread to be added to the rate. None means zero spread
    /// forward_curve (Rc<RefCell<ZeroCurve>>): forward curve
    /// close_data (Rc<CloseData>): historical data which is used when the fixing date is before the evaluation date
    /// pricing_date (OffsetDateTime): evaluation date
    /// compound_tenor (Option<&String>): compounding tenor. This is optional and None means that it is not a overnight type index. 
    /// For example, if the floatin part is CD91, Libor3M, etc, it is None, but in case of SOFR1D, it is Some(String::from("1D"))
    pub fn get_coupon_amount(
        &self,
        base_schedule: &BaseSchedule,
        spread: Option<Real>,
        forward_curve: Rc<RefCell<ZeroCurve>>,
        close_data: Rc<CloseData>,
        pricing_date: &OffsetDateTime,
        compound_tenor: Option<&String>,
        calendar: &JointCalendar,
        daycounter: &DayCountConvention,
        fixing_days: i64,
    ) -> Result<Real> {
        let spread = spread.unwrap_or(0.0);
        match compound_tenor.as_ref() {
            None => {// None means that it is not a overnight type index
                let fixing_date = base_schedule.get_fixing_date();
                let curve_end_date = add_period(
                    fixing_date, self.curve_tenor.as_str());
                let res: Real;
                if fixing_date < pricing_date {
                    res = match close_data.get(fixing_date) {
                        Some(rate) => *rate,
                        None => {
                            println!(
                                "Warning! ({}:{}) fixing_date = {:?} is before the evaluation date = {:?}, \
                                but there is no rate in the fixing date, Thus, spot rate at the evalaution date is used",
                                file!(), line!(), fixing_date, pricing_date
                            );

                            forward_curve.borrow().get_forward_rate_from_evaluation_date(
                                &curve_end_date, Compounding::Simple,
                            )?
                        },
                    };                    
                } else { // fixing_date >= eval_dt
                    res = forward_curve.borrow().get_forward_rate_between_dates(
                        &fixing_date, 
                        &curve_end_date, 
                        Compounding::Simple,
                    )?;
                }
                    
                let frac = calendar.year_fraction(
                    base_schedule.get_calc_start_date(),
                    base_schedule.get_calc_end_date(),
                    daycounter,
                )?;
                
                Ok((res + spread) * frac)
            },
            Some(comp_tenor) => {//some means it is an ovenight type index
                let fixing_date = base_schedule.get_fixing_date();
                if fixing_date >= pricing_date {
                    // if the fixing date is after the evaluation date, the rate is calculated by the forward curve
                    // the value is taken as the average of the first and last fixing for performance
                    let curve_end_date = add_period(fixing_date, &self.curve_tenor.as_str());
                        
                    let first_rate = forward_curve.borrow().get_forward_rate_between_dates(
                        &fixing_date,
                        &curve_end_date,
                        Compounding::Simple,
                    )?;

                    let last_fixing_date = base_schedule.get_calc_end_date().clone() - Duration::days(fixing_days);
                    let last_rate = forward_curve.borrow().get_forward_rate_between_dates(
                        &last_fixing_date,
                        &add_period(&last_fixing_date, &self.curve_tenor.as_str()),
                        Compounding::Simple,
                    )?;

                    let rate = (first_rate + last_rate) / 2.0;

                    let frac = calendar.year_fraction(
                        base_schedule.get_calc_start_date(),
                        base_schedule.get_calc_end_date(),
                        daycounter,
                    )?;
                    return Ok((rate + spread) * frac);
                }
                let curve_end_date_from_eval_date = add_period(pricing_date, &comp_tenor.as_str());
                let mut calc_start_date = base_schedule.get_calc_start_date().clone();
                let mut next_calc_date: OffsetDateTime;
                let calc_end_date = base_schedule.get_calc_end_date().clone();
                
                let spot_rate = forward_curve.borrow().get_forward_rate_between_dates(
                    pricing_date,
                    &curve_end_date_from_eval_date,
                    Compounding::Simple,
                )?;

                let mut compounded_value: Real = 1.0;
                let mut rate: Real;
                let mut fixing_date: OffsetDateTime;
                let mut frac: Real;
                
                while calc_start_date < calc_end_date {
                    fixing_date = calendar.adjust(
                        &(calc_start_date - Duration::days(fixing_days)),
                        &BusinessDayConvention::Preceding,
                    )?;

                    next_calc_date = add_period(&calc_start_date, &comp_tenor.as_str());
                    next_calc_date = min_offsetdatetime(&next_calc_date, &calc_end_date);

                    frac = calendar.year_fraction(
                        &calc_start_date,
                        &next_calc_date,
                        &daycounter
                    )?;

                    if &fixing_date < pricing_date {
                        rate = match close_data.get(&fixing_date) {
                            Some(rate) => *rate,
                            None => {
                                println!(
                                    "{}:{} fixing_date = {:?} is before the evaluation date = {:?}, \
                                    but there is no rate in the fixing date, thus spot rate is taken",
                                    file!(), line!(), fixing_date.date(), pricing_date.date()
                                );
                                spot_rate
                            },
                        }
                    } else {
                        rate = forward_curve.borrow().get_forward_rate_between_dates(
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
        calendars::unitedstates::{UnitedStates, UnitedStatesType},
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
    use crate::evaluation_date::EvaluationDate;
    use crate::instruments::schedule::BaseSchedule;
    //
    use time::{
        macros::datetime,
        Duration,
    };
    use anyhow::Result;
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
        let compound_tenor = Some("1D".to_string());
        let fixing_days = 7;
        let calendar = JointCalendar::new(
            vec![
                Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Sofr)),
                ]
            )?;
        let currency = Currency::USD;
        let code = RateIndexCode::SOFR;
        let name = "SOFR1D".to_string();
        let rate_index = RateIndex::new(
            "1D".to_string(), // "1D" means "1D" forward curve calculation period
            currency,
            code,
            name,
        )?;

        let curve_data = VectorData::new(
            array![0.03, 0.03],
            None,
            Some(array![0.1, 0.2]),
            Some(dt.clone()),
            Currency::USD,
            "USDOIS".to_string(),
        )?;
        
        let zero_curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &curve_data,
                "USDOIS".to_string(),
                "USDOIS".to_string(),
            )?
        ));

        let calc_end_date = dt + Duration::days(90);
        let base_schedule = BaseSchedule::new(
            calendar.adjust(&(dt.clone() - Duration::days(7)), &BusinessDayConvention::Preceding)?,
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
        
        let close_data = Rc::new(CloseData::new(
            history_map,
            "SOFR1D".to_string(),
            "SOFR1D".to_string(),
        ));

        let rate = rate_index.get_coupon_amount(
            &base_schedule,
            None,
            zero_curve.clone(),
            close_data.clone(),
            evaluation_date.borrow().get_date(),
            compound_tenor.as_ref(),
            &calendar,
            &daycounter,
            fixing_days,
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