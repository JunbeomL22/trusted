use time::{OffsetDateTime, UtcOffset};
use time;
use crate::data::vector_data::VectorData;
use crate::definitions::{Time, Real, Integer};
use crate::evaluation_date::EvaluationDate;
use crate::math::interpolators::stepwise_interpolatior::StepwiseInterpolator1D;
use std::rc::Rc;
use std::cell::RefCell;
use crate::time::calendar::{NullCalendar, Calendar};
//use crate::data::observable::Observable;
use ndarray::Array1;

#[derive(Clone, Debug)]
pub struct DiscreteRatioDividend {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    ex_dividend_dates: Vec<OffsetDateTime>,
    marking_offsetdatetime: OffsetDateTime,
    date_serial_numbers: Array1<Integer>, // days from 1970-01-01
    time_calculator: NullCalendar,
    ex_dividend_times: Array1<Time>,
    dividend_yields: Array1<Real>,
    deduction_interpolator: StepwiseInterpolator1D<Integer>,
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
    /// The integer is calculated from the days from 1970-01-01 +17::30::00 offset, e.g., if it is listed in KRX
    /// marking_offset = UtcOffset::hours(9)
    /// The interpolator is made from the integer domain, and the range is Real. 
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        data: &Rc<RefCell<VectorData>>, // dividend amount
        marking_offset: UtcOffset,
        spot: Real,
        name: String,
    ) -> DiscreteRatioDividend {
        // Begining of the function
        let time_calcualtor = NullCalendar {};

        let ex_dividend_dates: Vec<OffsetDateTime> = data.borrow().get_dates_clone().unwrap();

        let dividend_amount: Array1<Real> = data.borrow().get_value_clone();
        let dividend_yields: Array1<Real> = dividend_amount / spot;

        let mut incremental_deduction_ratio = Array1::zeros(dividend_yields.len());

        let mut temp = 1.0;
        for (i, &yield_value) in (&dividend_yields).iter().enumerate() {
            temp *= 1.0 - yield_value;
            incremental_deduction_ratio[i] = temp;
        }

        let marking_offsetdatetime = OffsetDateTime::new_in_offset(
            time::macros::date!(2021-01-01),
            time::macros::time!(17:30:00),
            marking_offset,
        );

        let mut date_serial_numbers: Array1<Integer> = Array1::zeros(ex_dividend_dates.len());

        let mut ex_dividend_times: Array1<Time> = Array1::zeros(ex_dividend_dates.len());

        for (i, date) in ex_dividend_dates.iter().enumerate() {
            let result = *date - marking_offsetdatetime;
            let days = result.whole_days() as Integer;
            date_serial_numbers[i] = days;

            let time = time_calcualtor.get_time_difference(&marking_offsetdatetime, date);
            ex_dividend_times[i] = time;
        };

        let right_extrapolation_value = Some(incremental_deduction_ratio[incremental_deduction_ratio.len()-1]);
        let deduction_interpolator = StepwiseInterpolator1D::new(
            date_serial_numbers.clone(),
            incremental_deduction_ratio,
            false,
            Some(1.0),
            right_extrapolation_value,
        );

        DiscreteRatioDividend {
            evaluation_date,
            ex_dividend_dates,
            marking_offsetdatetime,
            date_serial_numbers,
            time_calculator: time_calcualtor,
            ex_dividend_times,
            dividend_yields,
            deduction_interpolator,
            name,
        }
    }
}


        