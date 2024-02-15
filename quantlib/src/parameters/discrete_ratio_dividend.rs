use time::OffsetDateTime;
use crate::definitions::{Time, Real, Integer};
use crate::parameter::Parameter;
use crate::data::vector_data::VectorData;
use crate::evaluation_date::EvaluationDate;
use crate::math::interpolators::stepwise_interpolatior::StepWiseInterpolator1D;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone, Debug)]
pub struct DiscreteRatioDividend {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    data: Rc<RefCell<VectorData>>,
    dividend_dates: Vec<OffsetDateTime>,
    date_serial_numbers: Vec<Integer>,
    dividend_yields: Vec<Real>,
    deduction_interpolator: StepWiseInterpolator1D<Real>,
    name: String,
}
