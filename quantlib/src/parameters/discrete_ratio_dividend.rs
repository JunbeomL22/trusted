use crate::definitions::{Integer, Real, Time, EX_DIVIDEND_TIME, MARKING_DATE};
use time::{OffsetDateTime, UtcOffset};
use time;
use crate::data::observable::Observable;
use crate::data::vector_data::VectorData;
use crate::evaluation_date::EvaluationDate;
use crate::math::interpolators::stepwise_interpolatior::{StepwiseInterpolator1D, ConstantInterpolator1D};
use crate::math::interpolator::Interpolator1D;
use std::rc::Rc;
use std::cell::RefCell;
use crate::time::calendar::{NullCalendar, Calendar};
use crate::parameter::Parameter;
use ndarray::Array1;
use crate::util::to_yyyymmdd_int;

#[derive(Clone, Debug)]
enum DividendInterpolator {
    Stepwise(StepwiseInterpolator1D<Integer>),
    Constant(ConstantInterpolator1D),
}

#[derive(Clone, Debug)]
pub struct DiscreteRatioDividend {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    ex_dividend_dates: Vec<OffsetDateTime>,
    time_calculator: NullCalendar,
    //ex_dividend_times: Array1<Time>,
    date_integers: Vec<Integer>,
    dividend_yields: Array1<Real>,
    deduction_interpolator: DividendInterpolator,
    spot: Real,
    name: String,
}

impl DiscreteRatioDividend {
    /// evaluation_date: Rc<RefCell<EvaluationDate>>,
    /// data: Rc<RefCell<VectorData>>, // dividend amount (not yield)
    /// data is used to make an inner interpolator of accumulated dividend ratio deduction
    /// data is not an attribute of DiscreteRatioDividen, but an observable variable
    /// 
    /// data: VectorData have two attributes: times and dates. VectorData::dates allows to be None
    /// But in the case of DiscreteRatioDividend, dates can not be None.
    /// This choice is made becasue dividend falls on a specific date not on a specific time.
    /// To be precise on the dividend deduction on ex-dividend date, the domain of inner interpolator is Integer, not Real.
    /// 
    /// The integer is calculated from the days from 1970-01-01 +16:00:00 offset, e.g., if it is listed in KRX
    /// marking_offset = UtcOffset::hours(9)
    /// The interpolator is made from the integer domain, and the range is Real. 
    ///
    /// The ex-dividend-time is 00:00:00, and the closing-time is 16:00:00
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        data: &VectorData, // dividend amount
        spot: Real,
        name: String,
    ) -> DiscreteRatioDividend {
        // Begining of the function
        let time_calculator = NullCalendar {};

        let ex_dividend_dates: Vec<OffsetDateTime> = data.get_dates_clone().unwrap();
        let dividend_amount: Array1<Real> = data.get_value_clone();
        let dividend_yields: Array1<Real> = dividend_amount / spot;

        let mut date_integers: Vec<Integer> = vec![0; ex_dividend_dates.len()];
        //let mut ex_dividend_times: Array1<Time> = Array1::zeros(ex_dividend_dates.len());

        for (i, date) in ex_dividend_dates.iter().enumerate() {
            date_integers[i] = to_yyyymmdd_int(date);
            //let time = time_calculator.get_time_difference(&marking_offsetdatetime, date);
            //ex_dividend_times[i] = time;
        };
        // drop data of ex-dividend date and dividend amount before the evaluation-date
        let eval_dt = evaluation_date.to_owned().borrow().get_date_clone(); 
        let mut ex_dividend_dates_for_interpolator = ex_dividend_dates.clone();
        let mut div_yields_vec = dividend_yields.to_vec();
        let mut date_integers_for_interpolator = date_integers.clone();

        let mut i = 0;
        let mut checker = 0;
        while i < ex_dividend_dates.len() {
            if ex_dividend_dates[checker] < eval_dt {
                ex_dividend_dates_for_interpolator.remove(i);
                div_yields_vec.remove(i);
                date_integers_for_interpolator.remove(i);
                checker += 1;
            } else {
                i += 1;
            }
        }

        let dividend_yields_for_interpolator = Array1::from(div_yields_vec);
        let mut incremental_deduction_ratio = Array1::zeros(dividend_yields_for_interpolator.len());
        let mut temp = 1.0;
        for (i, &yield_value) in (&dividend_yields_for_interpolator).iter().enumerate() {
            temp *= 1.0 - yield_value;
            incremental_deduction_ratio[i] = temp;
        }

        let deduction_interpolator;
        if incremental_deduction_ratio.len() == 0 {
            deduction_interpolator = DividendInterpolator::Constant(ConstantInterpolator1D::new(1.0)); 
        } else {
            let right_extrapolation_value = Some(incremental_deduction_ratio[incremental_deduction_ratio.len() - 1]);
            let _interp = StepwiseInterpolator1D::new(
                Array1::from_vec(date_integers_for_interpolator),
                incremental_deduction_ratio,
                true,
                Some(1.0),
                right_extrapolation_value,
            );
            deduction_interpolator = DividendInterpolator::Stepwise(_interp);
        }
        
        DiscreteRatioDividend {
            evaluation_date: evaluation_date.clone(),
            ex_dividend_dates,
            time_calculator,
            //ex_dividend_times,
            date_integers,
            dividend_yields,
            deduction_interpolator,
            spot,
            name,
        }
    }

    pub fn get_deduction_ratio(&self, date: &OffsetDateTime) -> Real {
        let date_int = to_yyyymmdd_int(date);
        let ratio = match self.deduction_interpolator {
            DividendInterpolator::Constant(ref interp) => interp.interpolate(date_int),
            DividendInterpolator::Stepwise(ref interp) => interp.interpolate(date_int),
        };
        ratio
    }

    pub fn get_vectorized_deduction_ratio_for_sorted_datetime(&self, dates: &Vec<OffsetDateTime>) -> Array1<Real> {
        let length = dates.len();
        let mut result = Array1::zeros(length);
        for i in 0..length {
            result[i] = self.get_deduction_ratio(&dates[i]);
        }
        result
    }

    pub fn get_evaluation_date_clone(&self) -> Rc<RefCell<EvaluationDate>> {
        self.evaluation_date.clone()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_dividend(&self) -> Vec<(OffsetDateTime, Real)> {
        self.ex_dividend_dates
        .iter()
        .zip(self.dividend_yields.iter())
        .map(|(date, yield_value)| (*date, self.spot * (*yield_value))).collect()
    }
}

impl Parameter for DiscreteRatioDividend {
    fn update(&mut self, data: &dyn Observable) {
        let data = data.as_any().downcast_ref::<VectorData>().expect("error: cannot downcast to VectorData in ZeroCurve::update");

        self.ex_dividend_dates = data.get_dates_clone().unwrap();
        let dividend_amount: Array1<Real> = data.get_value_clone();
        self.dividend_yields = dividend_amount / self.spot;

        self.date_integers = vec![0; self.ex_dividend_dates.len()];
        //self.ex_dividend_times = Array1::zeros(self.ex_dividend_dates.len());

        for (i, date) in self.ex_dividend_dates.iter().enumerate() {
            let result = *date - self.marking_offsetdatetime;
            let days = result.whole_days() as Integer;
            self.date_integers[i] = days;
            let time = self.time_calculator.get_time_difference(&self.marking_offsetdatetime, date);
            self.ex_dividend_times[i] = time;
        };

        // drop data of ex-dividend date and dividend amount before the evaluation-date
        let eval_dt = self.evaluation_date.to_owned().borrow().get_date_clone(); 
        let mut ex_dividend_dates_for_interpolator = self.ex_dividend_dates.clone();
        let mut div_yields_vec = self.dividend_yields.to_vec();
        let mut date_integers_for_interpolator = self.date_integers.clone();

        let mut i = 0;
        let mut checker = 0;
        while i < self.ex_dividend_dates.len() {
            if self.ex_dividend_dates[checker] < eval_dt {
                ex_dividend_dates_for_interpolator.remove(i);
                div_yields_vec.remove(i);
                date_integers_for_interpolator.remove(i);
                checker += 1;
            } else {
                i += 1;
            }
        }

        let dividend_yields_for_interpolator = Array1::from(div_yields_vec);
        let mut incremental_deduction_ratio = Array1::zeros(dividend_yields_for_interpolator.len());
        let mut temp = 1.0;
        for (i, &yield_value) in (&dividend_yields_for_interpolator).iter().enumerate() {
            temp *= 1.0 - yield_value;
            incremental_deduction_ratio[i] = temp;
        }

        if incremental_deduction_ratio.len() == 0 {
            self.deduction_interpolator = DividendInterpolator::Constant(ConstantInterpolator1D::new(1.0)); 
        } else {
            let right_extrapolation_value = Some(incremental_deduction_ratio[incremental_deduction_ratio.len() - 1]);
            let deduction_interpolator = StepwiseInterpolator1D::new(
                Array1::from_vec(date_integers_for_interpolator),
                incremental_deduction_ratio,
                true,
                Some(1.0),
                right_extrapolation_value,
            );
            self.deduction_interpolator = DividendInterpolator::Stepwise(deduction_interpolator);
        }
    }

    /// this does not change the original data such as
    /// self.evalaution_date, self.ex_dividend_dates, self.dividend_yields
    /// but only change the dividend_deduction interpolator
    fn update_evaluation_date(&mut self, date: &EvaluationDate) {
        let eval_dt: OffsetDateTime = date.get_date_clone();

        let mut ex_dividend_dates_for_interpolator = self.ex_dividend_dates.clone();
        let mut div_yields_vec = self.dividend_yields.to_vec();
        let mut date_integers_for_interpolator = self.date_integers.clone();

        let mut i = 0;
        let mut checker = 0;
        while i < self.ex_dividend_dates.len() {
            if self.ex_dividend_dates[checker] < eval_dt {
                ex_dividend_dates_for_interpolator.remove(i);
                div_yields_vec.remove(i);
                date_integers_for_interpolator.remove(i);
                checker += 1;
            } else {
                i += 1;
            }
        }

        let dividend_yields_for_interpolator = Array1::from(div_yields_vec);
        let mut incremental_deduction_ratio = Array1::zeros(dividend_yields_for_interpolator.len());
        let mut temp = 1.0;
        for (i, &yield_value) in (&dividend_yields_for_interpolator).iter().enumerate() {
            temp *= 1.0 - yield_value;
            incremental_deduction_ratio[i] = temp;
        }

        if incremental_deduction_ratio.len() == 0 {
            self.deduction_interpolator = DividendInterpolator::Constant(ConstantInterpolator1D::new(1.0)); 
        } else {
            let right_extrapolation_value = Some(incremental_deduction_ratio[incremental_deduction_ratio.len() - 1]);
            let deduction_interpolator = StepwiseInterpolator1D::new(
                Array1::from_vec(date_integers_for_interpolator),
                incremental_deduction_ratio,
                true,
                Some(1.0),
                right_extrapolation_value,
            );
            self.deduction_interpolator = DividendInterpolator::Stepwise(deduction_interpolator);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vector_data::VectorData;
    use crate::definitions::CLOSING_TIME;
    use crate::evaluation_date::EvaluationDate;
    use time::macros::{date, datetime};
    use time::UtcOffset;
    use ndarray::array;

    #[test]
    fn test_deduction_ratio() {
        let evaluation_date = Rc::new(
            RefCell::new(
                EvaluationDate::new(
                    OffsetDateTime::new_in_offset(
                        date!(2021-01-01),
                        CLOSING_TIME,
                        UtcOffset::from_hms(9, 0, 0).unwrap(),
                    ) 
                )
            )
        );
        let dates = vec![
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2021-01-03 00:00:00 +09:00),
            datetime!(2021-01-06 00:00:00 +09:00),
            datetime!(2021-01-08 00:00:00 +09:00),
            datetime!(2021-01-11 00:00:00 +09:00),
        ];

        let times = None;
        let values = array![0.1, 0.2, 0.3, 0.4, 0.5];
        let mut data = VectorData::new(
            values,
            Some(dates), 
            times, 
            datetime!(2021-01-01 17:30:00 +09:00),
            "test".to_string()
        );

        let marking_offset = UtcOffset::from_hms(9, 0, 0).unwrap();
        let spot = 1.0;
        let name = "test".to_string();
        let discrete_ratio_dividend = DiscreteRatioDividend::new(
            evaluation_date.clone(),
            &data,
            //marking_offset,
            spot,
            name,
        );

        let dividend = Rc::new(RefCell::new(discrete_ratio_dividend));
        data.add_observer(dividend.clone());
        evaluation_date.borrow_mut().add_observer(dividend.clone());

        let test_dates = vec![
            datetime!(2021-01-01 10:00:00 +09:00),
            datetime!(2021-01-02 10:00:00 +09:00),
            datetime!(2021-01-03 10:00:00 +09:00), // ex-dividend date
            datetime!(2021-01-04 10:00:00 +09:00),
            datetime!(2021-01-05 10:00:00 +09:00),
            datetime!(2021-01-06 10:00:00 +09:00), // ex-dividend date
            datetime!(2021-01-07 10:00:00 +09:00),
            datetime!(2021-01-08 10:00:00 +09:00), // ex-dividend date
            datetime!(2021-01-09 10:00:00 +09:00),
            datetime!(2021-01-10 10:00:00 +09:00),
            datetime!(2021-01-11 10:00:00 +09:00), // ex-dividend date
            datetime!(2021-01-12 10:00:00 +09:00),
        ];

        let test_values: Vec<Real> = vec![
            1.0, // evaluation_date is before the first ex-dividend date
            1.0,    
            0.9,
            0.9,
            0.9,
            0.9 * 0.8,
            0.9 * 0.8,
            0.9 * 0.8 * 0.7,
            0.9 * 0.8 * 0.7,
            0.9 * 0.8 * 0.7,
            0.9 * 0.8 * 0.7 * 0.6,
            0.9 * 0.8 * 0.7 * 0.6,
        ];

        for (date, val) in test_dates.iter().zip(test_values.iter()) {
            let ratio = dividend.borrow().get_deduction_ratio(&date);
            assert!(
                (ratio - val) < 1.0e-10,
                "date: {:?}, val: {:?}, ratio: {}, expected: {}",
                date,
                val,
                ratio,
                val
            );
        }

        // bump all values by 0.1
        data += 0.1; // notified to the dividend
        let test_values: Vec<Real> = vec![
            1.0, // evaluation_date is before the first ex-dividend date
            1.0,    
            0.8,
            0.8,
            0.8,
            0.8 * 0.7,
            0.8 * 0.7,
            0.8 * 0.7 * 0.6,
            0.8 * 0.7 * 0.6,
            0.8 * 0.7 * 0.6,
            0.8 * 0.7 * 0.6 * 0.5,
            0.8 * 0.7 * 0.6 * 0.5,
        ];

        for (date, val) in test_dates.iter().zip(test_values.iter()) {
            let ratio = dividend.borrow().get_deduction_ratio(&date);
            assert!(
                (ratio - val) < 1.0e-10,
                "(after bumped) date: {:?}, val: {:?}, ratio: {}, expected: {}",
                date,
                val,
                ratio,
                val
            );
        }

        // drop the the first two ex-dividend dates by evaluation_date += "2D"
        *evaluation_date.borrow_mut() += "2D";

        let test_values: Vec<Real> = vec![
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0 * 0.7,
            1.0 * 0.7,
            1.0 * 0.7 * 0.6,
            1.0 * 0.7 * 0.6,
            1.0 * 0.7 * 0.6,
            1.0 * 0.7 * 0.6 * 0.5,
            1.0 * 0.7 * 0.6 * 0.5,
        ];

        for (date, val) in test_dates.iter().zip(test_values.iter()) {
            let ratio = dividend.borrow().get_deduction_ratio(&date);
            assert!(
                (ratio - val) < 1.0e-10,
                "(after add 2D from evaluation_date) date: {:?}, val: {:?}, ratio: {}, expected: {}",
                date,
                val,
                ratio,
                val
            );
        }

        // now recover again by shift evaluation_date -= "2D"
        *evaluation_date.borrow_mut() -= "2D";

        let test_values: Vec<Real> = vec![
            1.0, // evaluation_date is before the first ex-dividend date
            1.0,    
            0.8,
            0.8,
            0.8,
            0.8 * 0.7,
            0.8 * 0.7,
            0.8 * 0.7 * 0.6,
            0.8 * 0.7 * 0.6,
            0.8 * 0.7 * 0.6,
            0.8 * 0.7 * 0.6 * 0.5,
            0.8 * 0.7 * 0.6 * 0.5,
        ];

        for (date, val) in test_dates.iter().zip(test_values.iter()) {
            let ratio = dividend.borrow().get_deduction_ratio(&date);
            assert!(
                (ratio - val) < 1.0e-10,
                "(after add 2D and then subtract 2D from evaluation_date) date: {:?}, val: {:?}, ratio: {}, expected: {}",
                date,
                val,
                ratio,
                val
            );
        }
            
    }
}


        