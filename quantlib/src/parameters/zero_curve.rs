use time::OffsetDateTime;
use crate::parameters::enums::{ZeroCurveCode, Compounding};
use crate::evaluation_date::EvaluationDate;
use crate::data::vector_data::VectorData;
use crate::definitions::{Real, Time};
use crate::parameter::Parameter;
use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
use crate::math::interpolator::InterpolatorReal1D;
use crate::math::interpolator::ExtraPolationType;
use crate::time::calendar::{NullCalendar, Calendar};
use crate::utils::string_arithmetic::add_period;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

/// ZeroCurve is a curve of zero rates which implements Parameter (Observer) trait.
/// Input is a vector of dates and a vector of zero rates of Data (observable) type.
/// when the zero rates are updated, the zero curve will be updated.
#[derive(Clone, Debug)]
pub struct ZeroCurve {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    data: Rc<RefCell<VectorData>>,
    rate_interpolator: LinearInterpolator1D,
    discount_times: Vec<Time>,
    discount_factors: Vec<Real>,
    discount_interpolator: LinearInterpolator1D,
    time_calculator: NullCalendar,
    code: ZeroCurveCode,
    name: String,
}

impl ZeroCurve {
    /// Create a new ZeroCurve
    /// For performance reasons, zero curve caches discount and then interpolate the discount factor
    /// To reproduce the linear interest rate linear interpolation as much as possible, 
    /// the discount factors are cached by the interpolated rate in between the times of the input data.
    /// The interpolated tenors of the given rate are:
    /// "0D", "1D", 
    /// "1W", "2W", 
    /// "1M", "2M", "3M", "4M", "5M", "6M", "7M", "8M", "9M", "10M", "11M", "1Y", 
    /// "1Y6M", "2Y", "2Y6M", "3Y", 
    /// "4Y", "5Y", "6Y", "7Y", "8Y", "9Y", "10Y",
    /// "12Y", "15Y", "17Y", "20Y",
    ///"25Y", "30Y", "40Y", "50Y", "70Y", "100Y"
    pub fn new(evaluation_date: Rc<RefCell<EvaluationDate>>, data: Rc<RefCell<VectorData>>, code: ZeroCurveCode, name: String) -> ZeroCurve {
        let rate_times = data.get_times();
        let zero_rates = data.get_value();
        let time_calculator =  NullCalendar {};

        let rate_interpolator = LinearInterpolator1D::new(rate_times.clone(), zero_rates.clone(), ExtraPolationType::Flat, true);
        let period_leteral = vec![
            "0D", "1D", 
            "1W", "2W", 
            "1M", "2M", "3M", "4M", "5M", "6M", "7M", "8M", "9M", "10M", "11M", "1Y", 
            "1Y6M", "2Y", "2Y6M", "3Y", 
            "4Y", "5Y", "6Y", "7Y", "8Y", "9Y", "10Y",
            "12Y", "15Y", "17Y", "20Y",
            "25Y", "30Y", "40Y", "50Y", "70Y", "100Y"
            ];

        let discount_times: Vec<Time> = period_leteral.iter().map(|period| add_period(&evaluation_date.get_date(), period))
            .map(|date| time_calculator.get_time_difference(&evaluation_date.get_date(), &date)).collect();
        let interpolated_rates = rate_interpolator.vectorized_interpolate_for_sorted_input(&discount_times);
        let discount_factors: Vec<Real> = interpolated_rates.iter().zip(&discount_times).map(|(rate, time)| (-rate * time).exp()).collect();
        
        let discount_interpolator = LinearInterpolator1D::new(discount_times.clone(), discount_factors.clone(), ExtraPolationType::None, false);
        
        ZeroCurve {
            evaluation_date,
            data,
            rate_interpolator,
            discount_times,
            discount_factors,
            discount_interpolator,
            time_calculator,
            code,
            name,
        }
    }
    pub fn get_discount_factor(&self, time: Time) -> Real {
        self.discount_interpolator.interpolate(time)
    }

    pub fn get_vectorized_discount_factor_for_sorted_time(&self, times: &Vec<Time>) -> Vec<Real> {
        self.discount_interpolator.vectorized_interpolate_for_sorted_input(times)
    }

    pub fn get_discount_factor_at_date(&self, date: &OffsetDateTime) -> Real {
        self.get_discount_factor(self.time_calculator.get_time_difference(&self.evaluation_date.get_date(), date))
    }

    pub fn get_vectorized_discount_factor_for_sorted_dates(&self, dates: &Vec<OffsetDateTime>) -> Vec<Real> {
        dates.iter().map(|date| self.get_discount_factor_at_date(date)).collect()
    }

    pub fn get_discount_factor_between_times(&self, t1: Time, t2: Time) -> Real {
        assert!(
            t1 <= t2, 
            "(error: t1 > t2) occured in get_discount_factor_between_times(t1: {}, t2: {})",
            t1, 
            t2
            );
        let res = self.get_discount_factor(t2) / self.get_discount_factor(t1);
        res
    }

    pub fn get_forward_rate_between_times(&self, t1: Time, t2: Time, compounding: Compounding) -> Real {
        assert!(
            t1 <= t2, 
            "(error: t1 > t2) occured in get_forward_rate_between_times(t1: {}, t2: {}, compounding: {:?})",
            t1, 
            t2,
            compounding
            );

        let tau = t2 - t1; // tau must not be negligibly small
        assert!(
            tau > 1e-8,
            "(error: tau is negligibly small) occured in get_forward_rate_between_times(t1: {}, t2: {}, compounding: {:?})",
            t1, 
            t2,
            compounding
        );

        let disc = self.get_discount_factor_between_times(t1, t2);

        
        match compounding {
            Compounding::Simple => (1.0 - disc) / tau,
            Compounding::Continuous => -disc.ln() / tau,
        }
    }

    pub fn get_forward_rate_between_dates(&self, date1: &OffsetDateTime, date2: &OffsetDateTime, compounding: Compounding) -> Real {
        assert!(
            date1 <= date2,
            "(error: date1 > date2) occured in get_forward_rate_between_dates(date1: {:?}, date2: {:?}, compounding: {:?})",
            date1,
            date2,
            compounding
            );

        let t1 = self.time_calculator.get_time_difference(&self.evaluation_date.get_date(), date1);
        let t2 = self.time_calculator.get_time_difference(&self.evaluation_date.get_date(), date2);
        self.get_forward_rate_between_times(t1, t2, compounding)
    }

    pub fn get_short_rate_from_time(&self, time: Time) -> Real {
        self.get_forward_rate_between_times(time, time + 0.002737, Compounding::Simple)
    }

    pub fn get_vectorized_short_rate_for_sorted_times(&self, times: &Vec<Time>) -> Vec<Real> {
        let mut res = vec![0.0; times.len()];
        for i in 0..times.len() {
            res[i] = self.get_short_rate_from_time(times[i]);
        }
        res
    }
    pub fn get_instantaneous_forward_rate_from_date(&self, date: &OffsetDateTime) -> Real {
        let time = self.time_calculator.get_time_difference(&self.evaluation_date.get_date(), date);
        self.get_short_rate_from_time(time)
    }

    pub fn get_cached_discount_factors(&self) -> Vec<Real> {
        self.discount_factors.clone()
    }

    pub fn get_cached_discount_times(&self) -> Vec<Time> {
        self.discount_times.clone()
    }

    pub fn get_code(&self) -> ZeroCurveCode {
        self.code
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_data(&self) -> &VectorData {
        &self.data
    }

    pub fn get_evaluation_date(&self) -> &EvaluationDate {
        &self.evaluation_date
    }
    pub fn set_data(&mut self, data: Rc<VectorData>) {
        self.data = data;
        self.update();
    }
}

impl Parameter for ZeroCurve {
    fn update(&mut self) {
        let rate_times = self.data.get_times();
        let zero_rates = self.data.get_value();

        self.rate_interpolator = LinearInterpolator1D::new(rate_times, zero_rates, ExtraPolationType::Flat, true);

        let interpolated_rates = self.rate_interpolator.vectorized_interpolate_for_sorted_input(&self.discount_times);
        self.discount_factors = interpolated_rates.iter().zip(&self.discount_times).map(|(rate, time)| (-rate * time).exp()).collect();
        
        self.discount_interpolator = LinearInterpolator1D::new(self.discount_times.clone(), self.discount_factors.clone(), ExtraPolationType::None, false);
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluation_date::EvaluationDate;
    use super::*;
    use time::macros::datetime;
    use std::rc::Rc;
    use crate::time::calendar::{Calendar, NullCalendar};

    #[test]
    fn test_zero_curve() {
        let eval_dt = datetime!(2021-01-01 00:00:00 UTC);
        let evaluation_date = EvaluationDate::new(eval_dt);

        let param_dt = datetime!(2020-01-01 00:00:00 UTC);
        let dates = vec![
            param_dt, 
            add_period(&param_dt, "1M"), 
            add_period(&param_dt, "1Y"),
            add_period(&param_dt, "2Y"),
            add_period(&param_dt, "3Y"),
            add_period(&param_dt, "5Y")
            ];

        let data = VectorData::new(
            vec![0.02, 0.02, 0.025, 0.03, 0.035, 0.04],
            Some(dates.clone()), 
            None, 
            param_dt, 
            "vector data in test_zero_curve".to_string()
        );

        let zero_curve = ZeroCurve::new(Rc::new(evaluation_date), Rc::new(data), ZeroCurveCode::Undefined, "test".to_string());

        let cal = NullCalendar {};
        let times: Vec<Time> = dates
                                .iter()
                                .map(|&date| cal.get_time_difference(&param_dt, &date))
                                .collect();

        let expected_discount_factors = vec![
            1.0, 
            (-0.02 * times[1]).exp(), 
            (-0.025 * times[2]).exp(), 
            (-0.03 * times[3]).exp(), 
            (-0.035 * times[4]).exp(), 
            (-0.04 * times[5]).exp()
            ];

        let allow_error = 1e-6;
        for i in 0..times.len() {
            assert!(
                (zero_curve.get_discount_factor(times[i]) - expected_discount_factors[i]) < allow_error,
                "i: {}, zero_curve.get_discount_factor(times[i]): {}, expected_discount_factors[i]: {}",
                i,
                zero_curve.get_discount_factor(times[i]),
                expected_discount_factors[i]
                );
        }
    }
}
