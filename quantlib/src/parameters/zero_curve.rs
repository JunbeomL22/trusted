use crate::currency::Currency;
use time::{OffsetDateTime, macros::datetime};
use crate::enums::Compounding;
use crate::evaluation_date::EvaluationDate;
use crate::data::{vector_data::VectorData, observable::Observable};
use crate::definitions::{Real, Time};
use crate::parameter::Parameter;
use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
use crate::math::interpolator::InterpolatorReal1D;
use crate::math::interpolator::Interpolator1D;
use crate::math::interpolators::stepwise_interpolatior::ConstantInterpolator1D;
use crate::math::interpolator::ExtraPolationType;
use crate::time::{
    calendars::nullcalendar::NullCalendar, 
    calendar_trait::CalendarTrait,
};
use crate::utils::string_arithmetic::add_period;
//
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use ndarray::{Array1, array};
use anyhow::{Result, Context, anyhow};

#[derive(Clone, Debug)]
enum ZeroCurveInterpolator {
    Constant(ConstantInterpolator1D),
    Linear(LinearInterpolator1D),
}

/// ZeroCurve is a curve of zero rates which implements Parameter (Observer) trait.
/// Input is a vector of dates and a vector of zero rates of Data (observable) type.
/// when the zero rates are updated, the zero curve will be updated.
#[derive(Clone, Debug)]
pub struct ZeroCurve {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    rate_interpolator: ZeroCurveInterpolator,
    interpolated_rates: Array1<Real>,
    discount_times: Array1<Time>,
    discount_factors: Array1<Real>,
    discount_interpolator: LinearInterpolator1D,
    time_calculator: NullCalendar,
    name: String,
    code: String,
}

impl ZeroCurve {
    /// Create a new ZeroCurve
    /// For performance reasons, zero curve caches discount and then interpolate the discount factor
    /// To reproduce the linear interest rate linear interpolation as much as possible, 
    /// the discount factors are cached by the interpolated rate in between the times of the input data.
    /// The interpolated tenors of the given rate are:\n
    /// 
    /// ["0D", "1D", 
    /// "1W", "2W", 
    /// "1M", "2M", "3M", "4M", "5M", "6M", "9M", "1Y", 
    /// "1Y6M", "2Y", "2Y6M", "3Y", 
    /// "4Y", "5Y", "6Y", "7Y", "8Y", "9Y", "10Y",
    /// "12Y", "15Y", "20Y", "30Y", "50Y", "100Y"]
    /// 
    /// This setup is chosen for afety and clean code but it is not the most efficient way. 
    /// I leave the optimization for later.
    
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>, 
        data: &VectorData,
        name: String,
        code: String,
    ) -> Result<ZeroCurve> {
        let rate_times = data.get_times_clone();
        let zero_rates = data.get_value_clone();
        let time_calculator =  NullCalendar::default();
        
        if rate_times.len() != zero_rates.len() {
            let error = anyhow!(
                "({}:{}) input data length mismatch\n\
                name = {}\n\
                data = {:?}\n\
                zero_rates = {:?}\n\
                rate_times = {:?}", 
                file!(), line!(),
                name, data, zero_rates, rate_times);
            return Err(error)
        }
        
        if zero_rates.len() < 1 {
            let error = anyhow!(
                "({}:{}) name = {} zero_rates = {:?} data = {:?}", 
                file!(), line!(),
                name, zero_rates, data);
            return Err(error)
        }

        let rate_interpolator: ZeroCurveInterpolator;

        if zero_rates.len() == 1 {
            rate_interpolator = ZeroCurveInterpolator::Constant(
                ConstantInterpolator1D::new(zero_rates[0])?
            );
        } else {
            rate_interpolator = ZeroCurveInterpolator::Linear(
                LinearInterpolator1D::new(
                    rate_times.clone(),
                    zero_rates.clone(), 
                    ExtraPolationType::Flat, 
                    true)?
            );
        }
        
        let period_leteral = vec![
            "0D", "1D", 
            "1W", "2W", 
            "1M", "2M", "3M", "4M", "5M", "6M", "9M", "1Y", 
            "1Y6M", "2Y", "2Y6M", "3Y", 
            "4Y", "5Y", "6Y", "7Y", "8Y", "9Y", "10Y",
            "12Y", "15Y", "20Y", "30Y", "50Y", "100Y"
            ];

        let eval_date = evaluation_date.clone();
        let mut discount_times: Array1<Time> = Array1::zeros(period_leteral.len());
        //discount_times[0] = - 0.0001;
        for (i, period) in period_leteral.iter().enumerate() {
            discount_times[i] = time_calculator.get_time_difference(
                &eval_date.borrow().get_date_clone(), 
                &add_period(&eval_date.borrow().get_date_clone(), period)
            );
        };

        let interpolated_rates = match &rate_interpolator {
            ZeroCurveInterpolator::Constant(c) =>  c.vectorized_interpolate_for_sorted_ndarray(&discount_times)?, 
            ZeroCurveInterpolator::Linear(l) => l.vectorized_interpolate_for_sorted_ndarray(&discount_times)?,
        };
        
        let discount_factors: Array1<Real> = (&interpolated_rates * &discount_times).mapv(|x| (-x).exp());
        
        let discount_interpolator = LinearInterpolator1D::new(
            discount_times.clone(), 
            discount_factors.clone(), 
            ExtraPolationType::None, 
            false
        )?;
        
        let res = ZeroCurve {
            evaluation_date: evaluation_date.clone(),
            rate_interpolator,
            interpolated_rates,
            discount_times,
            discount_factors,
            discount_interpolator,
            time_calculator,
            name,
            code,
        };
        Ok(res)
    }

    /// For self.interpolated_rates in the time_interval (date1 < date <= date2)
    /// bump self.interpolated_rates by bump_val
    /// then reset 
    /// self.rate_interpolator, self.discount_factors, and self.discount_interpolator
    pub fn bump_date_interval(
        &mut self, 
        date1: Option<&OffsetDateTime>, 
        date2: Option<&OffsetDateTime>,
        bump_val: Real
    ) -> Result<()> {
        let dt = &self.evaluation_date.borrow().get_date_clone();

        let t1 = match date1 {
            Some(d) => self.time_calculator.get_time_difference(dt, d),
            None => -99999999.0
        };

        let t2 = match date2 {
            Some(d) => self.time_calculator.get_time_difference(dt, d),
            None => 99999999.0
        };

        self.bump_time_interval(Some(t1), Some(t2), bump_val)
    }

    /// For self.interpolated_rates in the time_interval (t1 < t <= t2)
    /// bump self.interpolated_rates by bump_val
    /// then reset 
    /// self.rate_interpolator, self.discount_factors, and self.discount_interpolator
    pub fn bump_time_interval(
        &mut self, 
        time1: Option<Time>, 
        time2: Option<Time>,
        bump_val: Real
    ) -> Result<()> {
        let t1 = match time1 {
            Some(t) => t,
            None => -99999999.0
        };
        let t2 = match time2 {
            Some(t) => t,
            None => 99999999.0
        };
        //sanity check
        if self.interpolated_rates.len() != self.discount_times.len() {
            return Err(anyhow!(
                "self.interpolated_rates = {:?} but\nself.discount_times = {:?}",
                self.interpolated_rates, self.discount_times
            ));
        }

        if t1 > t2 {
            return Err(anyhow!("t1 = {} > t2 = {} in ZeroCurve::bump_time_interval", t1, t2));
        }
        // sanity check is done

        // bump rates
        let mask = self.discount_times.mapv(
            |x| if (x > t1) & (x <= t2) {1.0} else {0.0});

        self.interpolated_rates = &self.interpolated_rates + mask * bump_val;
        // reset self.rate_interpolator
        if self.interpolated_rates.len() == 1 {
            self.rate_interpolator = ZeroCurveInterpolator::Constant(
                ConstantInterpolator1D::new(self.interpolated_rates[0])?
            );
        } else {
            self.rate_interpolator = ZeroCurveInterpolator::Linear(
                LinearInterpolator1D::new(
                    self.discount_times.clone(),
                    self.interpolated_rates.clone(), 
                    ExtraPolationType::Flat, 
                    true)?
            );
        }
        // reset self.discount_factors
        self.discount_factors = (&self.interpolated_rates * &self.discount_times).mapv(|x| (-x).exp());
        // reset self.discount_interpolator
        self.discount_interpolator = LinearInterpolator1D::new(
            self.discount_times.clone(), 
            self.discount_factors.clone(), 
            ExtraPolationType::None, 
            false
        )?;
        Ok(())
    }

    pub fn get_interpolated_rates(&self) -> Array1<Real> {
        self.interpolated_rates.clone()
    }

    pub fn dummy_curve() -> Result<ZeroCurve> {
        let dt = EvaluationDate::new(datetime!(1970-01-01 00:00:00 UTC));
        let evaluation_date = Rc::new(RefCell::new(dt));
        let data = VectorData::new(
            array![0.0],
            Some(vec![datetime!(2080-01-01 00:00:00 UTC)]), // dummy date
            None, 
            Some(evaluation_date.borrow().get_date_clone()),
            Currency::NIL,
            "dummy curve in ZeroCurve::null_curve".to_string()
        ).with_context(|| "error in ZeroCurve::dummy_curve")?;

        ZeroCurve::new(
            evaluation_date,
            &data,
            String::from("Dummy"),
            String::from("Dummy"),
        )
    }
    pub fn get_discount_factor(&self, time: Time) -> Result<Real> {
        self.discount_interpolator.interpolate(time)
    }

    pub fn get_vectorized_discount_factor_for_sorted_time(&self, times: &Array1<Time>) -> Result<Array1<Real>> {
        self.discount_interpolator.vectorized_interpolate_for_sorted_ndarray(times)
    }

    pub fn get_discount_factor_at_date(&self, date: &OffsetDateTime) -> Result<Real> {
        let t = self.time_calculator.get_time_difference(&self.evaluation_date.borrow().get_date_clone(), date);
        if t < 0.0 {
            Err(anyhow!(
                "(ZeroCurve::get_discount_factor_at_date)\n\
                date = {:?} > evaluation date = {:?}.\n\
                An action on negative time is not defined.\n\
                If it is intentional, check {}:{}",
                date, self.evaluation_date.borrow().get_date_clone(),
                file!(), line!()))
        } else {
            self.get_discount_factor(t)
        }
    }

    pub fn get_vectorized_discount_factor_for_sorted_dates(&self, dates: &Vec<OffsetDateTime>) -> Result<Vec<Real>> {
        dates.iter().map(|date| self.get_discount_factor_at_date(date)).collect()
    }

    pub fn get_discount_factor_between_times(&self, t1: Time, t2: Time) -> Result<Real> {
        match t1 <= t2 {
            true => Ok(self.get_discount_factor(t2)? / self.get_discount_factor(t1)?),
            false => {
                let error = anyhow!(
                    "{} > {} in ZeroCurve::get_discount_factor_between_times. name = {}", 
                    t1, t2, self.name
                );
                Err(error)
            }
        }
    }

    pub fn get_forward_rate_between_times(&self, t1: Time, t2: Time, compounding: Compounding) -> Result<Real> {
        match t1 <= t2 {
            true => {
                let tau = t2 - t1;
        
                let disc: Real;
                if tau.abs() > 1e-6 {
                    disc = match self.get_discount_factor_between_times(t1, t2) {
                        Ok(d) => d,
                        Err(e) => return Err(e)
                    }
                } else {
                    disc = match self.get_discount_factor_between_times(t1, t2 + 1e-5) {
                        Ok(d) => d,
                        Err(e) => return Err(e)
                    }
                }
                
                match compounding {
                    Compounding::Simple => Ok((1.0 - disc) / tau),
                    Compounding::Continuous => Ok(-disc.ln() / tau),
                }
            }
            false => {
                let error = anyhow!(
                    "({}:{}) {} > {} in ZeroCurve::get_forward_rate_between_times", 
                    file!(), line!(),
                    t1, t2
                );
                Err(error)
            }
        }
    }

    pub fn get_forward_rate_between_dates(
        &self, 
        date1: &OffsetDateTime, 
        date2: &OffsetDateTime, 
        compounding: Compounding
    ) -> Result<Real> {
        if date1 > date2 {
            let error = anyhow!(
                "({}:{}) {:?} > {:?} in ZeroCurve::get_forward_rate_between_dates", 
                file!(), line!(),
                date1, date2);
            return Err(error)
        }

        let t1 = self.time_calculator.get_time_difference(&self.evaluation_date.borrow().get_date_clone(), date1);
        let t2 = self.time_calculator.get_time_difference(&self.evaluation_date.borrow().get_date_clone(), date2);
        return self.get_forward_rate_between_times(t1, t2, compounding)
    }

    pub fn get_forward_rate_from_evaluation_date(
        &self, 
        date: &OffsetDateTime, 
        compounding: Compounding
    ) -> Result<Real> {
        let dt = self.evaluation_date.borrow().get_date_clone();
        if date < &dt {
            let error = anyhow!(
                "({}:{}) date = {:?} < evaluation date = {:?} in ZeroCurve::get_forward_rate_from_evaluation_date", 
                file!(), line!(),
                date, dt);
            return Err(error)
        }
        self.get_forward_rate_between_dates(
            &dt,
            date, 
            compounding
        )
    }

    pub fn get_short_rate_at_time(&self, time: Time) -> Result<Real> {
        match self.get_forward_rate_between_times(time, time + 0.002737, Compounding::Simple) {
            Ok(rate) => Ok(rate),
            Err(e) => Err(e)
        }
    }

    pub fn get_vectorized_short_rate_for_sorted_times(&self, times: &Vec<Time>) -> Result<Vec<Real>> {
        let mut res = vec![0.0; times.len()];
        for i in 0..times.len() {
            res[i] = match self.get_short_rate_at_time(times[i]) {
                Ok(rate) => rate,
                Err(e) => return Err(e)
            
            }
        }
        Ok(res)
    }
    pub fn get_instantaneous_forward_rate_from_date(&self, date: &OffsetDateTime) -> Result<Real> {
        let time = self.time_calculator.get_time_difference(&self.evaluation_date.borrow().get_date_clone(), date);
        self.get_short_rate_at_time(time)
    }

    pub fn get_cached_discount_factors_clone(&self) -> Array1<Real> {
        self.discount_factors.clone()
    }

    pub fn get_cached_discount_times_clone(&self) -> Array1<Time> {
        self.discount_times.clone()
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_name_clone(&self) -> String {
        self.name.clone()
    }

    pub fn get_evaluation_date_clone(&self) -> Rc<RefCell<EvaluationDate>> {
        self.evaluation_date.clone()
    }

}

impl Parameter for ZeroCurve {
    fn get_address(&self) -> String {
        format!("{:p}", self)
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_type_name(&self) -> &'static str {
        "ZeroCurve"
    }

    fn update(&mut self, data: &dyn Observable) -> Result<()> {
        let data = data.as_any().downcast_ref::<VectorData>().expect("error: cannot downcast to VectorData in ZeroCurve::update");
        let rate_times = data.get_times_clone();
        let zero_rates = data.get_value_clone();

        if zero_rates.len() != rate_times.len() {
            let error = anyhow!(
                "update filed by input data length mismatch\n\
                name = {}\n\
                data = {:?}\n\
                zero_rates = {:?}\n\
                rate_times = {:?}", 
                self.name, data, zero_rates, rate_times);
            
            return Err(error)
        }

        if zero_rates.len() < 1 {
            let error = anyhow!("name = {} zero_rates = {:?} data = {:?}", self.name, zero_rates, data);
            return Err(error)
        }

        if zero_rates.len() == 1 {
            self.rate_interpolator = ZeroCurveInterpolator::Constant(
                ConstantInterpolator1D::new(zero_rates[0])?
            );
        } else {
            self.rate_interpolator = ZeroCurveInterpolator::Linear(
                LinearInterpolator1D::new(
                    rate_times.clone(),
                    zero_rates.clone(), 
                    ExtraPolationType::Flat, 
                    true
                )?
            );
        }

        let interpolated_rates = match &self.rate_interpolator {
            ZeroCurveInterpolator::Constant(c) =>  c.vectorized_interpolate_for_sorted_ndarray(&self.discount_times)?, 
            ZeroCurveInterpolator::Linear(l) => l.vectorized_interpolate_for_sorted_ndarray(&self.discount_times)?,
        };
        
        self.discount_factors = interpolated_rates.iter().zip(&self.discount_times).map(|(rate, time)| (-rate * time).exp()).collect();
        
        self.discount_interpolator = LinearInterpolator1D::new(self.discount_times.clone(), self.discount_factors.clone(), ExtraPolationType::None, false)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluation_date::EvaluationDate;
    use super::*;
    use anyhow::Ok;
    use time::macros::datetime;
    use std::rc::Rc;
    use crate::time::calendars::nullcalendar::NullCalendar;
    use ndarray::array;
    use crate::utils::string_arithmetic::add_period;

    #[test]
    fn test_zero_curve() -> Result<()> {
        let eval_dt = datetime!(2021-01-01 00:00:00 UTC);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(eval_dt)));

        let param_dt = datetime!(2020-01-01 00:00:00 UTC);
        let dates = vec![
            param_dt, 
            add_period(&param_dt, "1M"), 
            add_period(&param_dt, "1Y"),
            add_period(&param_dt, "2Y"),
            add_period(&param_dt, "3Y"),
            add_period(&param_dt, "5Y")
            ];

        let mut data = VectorData::new(
            array![0.02, 0.02, 0.025, 0.03, 0.035, 0.04],
            Some(dates.clone()), 
            None, 
            Some(param_dt), 
            Currency::NIL,
            "vector data in test_zero_curve".to_string()
        ).expect("error in test_zero_curve");

        let _zero_curve = ZeroCurve::new(
            evaluation_date,
            &data,
            String::from("test"),
            "test".to_string()
        ).expect("error in test_zero_curve");

        let zero_curve = Rc::new(RefCell::new(_zero_curve));
        
        data.add_observer(zero_curve.clone());

        let cal = NullCalendar::default();
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
                (zero_curve.borrow().get_discount_factor(times[i])? - expected_discount_factors[i]) < allow_error,
                "i: {}, zero_curve.get_discount_factor(times[i]): {}, expected_discount_factors[i]: {}",
                i,
                zero_curve.borrow().get_discount_factor(times[i])?,
                expected_discount_factors[i]
                );
        }

        Ok(())
    }
}
