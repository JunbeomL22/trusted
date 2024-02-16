use time::{OffsetDateTime, UtcOffset};
use time;
use crate::definitions::{Time, Real, Integer};
use crate::evaluation_date::EvaluationDate;
use crate::math::interpolators::stepwise_interpolatior::StepWiseInterpolator1D;
use std::rc::Rc;
use std::cell::RefCell;
use crate::time::calendar::NullCalendar;
use crate::data::data::vector_data::VectorData;

#[derive(Clone, Debug)]
pub struct DiscreteRatioDividend {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    ex_dividend_dates: Vec<OffsetDateTime>,
    marking_offsetdatetime: OffsetDateTime,
    date_serial_numbers: Vec<Integer>, // days from 1970-01-01
    time_calculator: NullCalendar,
    ex_dividend_times: Vec<Time>,
    dividend_yields: Vec<Real>,
    deduction_interpolator: StepWiseInterpolator1D<Real>,
    name: String,
}

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
            dividend_times.clone(),
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
