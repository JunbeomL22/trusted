use crate::parameters::zero_curve::ZeroCurve;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::data::value_data::ValueData;
//
use std::rc::Rc;
use std::cell::RefCell;

pub struct FuturesEngine {
    collateral_curve: ZeroCurve, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    borrowing_curve: ZeroCurve, // or repo
    dividend: DiscreteRatioDividend,
    value_data: Rc<RefCell<ValueData>>,
    name: String,
}
