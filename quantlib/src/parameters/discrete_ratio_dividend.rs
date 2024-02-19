use time::{OffsetDateTime, UtcOffset};
use time;
use crate::data::vector_data::VectorData;
use crate::definitions::{Time, Real, Integer};
use crate::evaluation_date::EvaluationDate;
use crate::math::interpolators::stepwise_interpolatior::StepwiseInterpolator1D;
use std::rc::Rc;
use std::cell::RefCell;
use crate::time::calendar::NullCalendar;
use crate::data::observable::Observable;
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
    pub fn new(
        evaluation_date: &Rc<RefCell<EvaluationDate>>,
        data: &VectorData, // dividend amount
        marking_offset: UtcOffset,
        spot: Real,
        name: String,
    ) -> DiscreteRatioDividend {
        let marking_offsetdatetime = OffsetDateTime::new_in_offset(
            time::Date::from_ymd(1970, 1, 1),
            time::Time::from_hms(18, 0, 0), 
            &marking_offset
        );
        
        let eval_date = evaluation_date.clone();
        let ex_dividend_dates = data.get_dates_clone();

        let dt_clone = eval_date.borrow().get_date_clone();

        if (ex_dividend_dates[0] - dt_clone).abs() > time::Duration::days(1) {
            ex_dividend_dates.insert(0, dt_clone);  
        }
        
        let date_serial_numbers: Array1<Integer> = Array1::from_vec(
            ex_dividend_dates
            .iter()
            .map(|x| x.num_days_from_ce())
            .collect());
        let time_calculator = NullCalendar::new();
        let ex_dividend_times: Array1<Time> = Array1::from_vec(
            ex_dividend_dates
            .iter()
            .map(|x| time_calculator.get_time_difference(eval_date.borrow().get_date(), *x))
            .collect());
        let dividend_yields = data.get_values().map(|x| x / spot).collect();

    }


/*
impl DiscreteRatioDividend {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        data: Rc<RefCell<VectorData>>, // dividend amount
        marking_offset: UtcOffset,
        spot: Real,
        name: String,
    ) -> DiscreteRatioDividend {
        let marking_offsetdatetime = OffsetDateTime::new_in_offset(
            time::Date::from_ymd(1970, 1, 1),
            time::Time::from_hms(18, 0, 0), 
            &marking_offset
        );
        let ex_dividend_dates = data.get_dates();
        let eval_date = evaluation_date.borrow().get_date();
        let date_serial_numbers: Vec<Integer> = ex_dividend_dates.iter().map(|x| x.num_days_from_ce()).collect();
        let time_calculator = NullCalendar::new();
        let ex_dividend_times: Vec<Time> = ex_dividend_dates.iter().map(
            |x| time_calculator.get_time_difference(eval_date, *x))
            .collect();


        let dividend_yields = data.get_values().map(|x| x / spot).collect();
        if ex_dividend_times[0] > 1e-8 {
            ex_dividend_times.insert(0, 0.0);
            dividend_yields.insert(0, 0.0);
        }
        
        let deduction_product = dividend_yields.iter().fold(1.0, |acc, x| acc * (1.0 - x));
        let deduction_interpolator = StepWiseInterpolator1D::new(
            ex_dividend_times.clone(),
            dividend_yields.clone(),
        );
        let deduction_interpolator = StepWiseInterpolator1D::new(
            ex_dividend_times.clone(),
            dividend_yields.clone(),
        );
        DiscreteRatioDividend {
            evaluation_date,
            dividend_dates,
            marking_offsetdatetime,
            date_serial_numbers,
            time_calculator,
            dividend_times,
            dividend_yields,
            deduction_interpolator,
            name,
        }
    }
}
*/