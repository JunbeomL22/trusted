use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::data::history_data::CloseData;
use crate::pricing_engines::pricer::PricerTrait;
use crate::instrument::{
    Instrument,
    InstrumentTrait,
};
use crate::definitions::Real;
// 
use std::{
    cell::RefCell,
    rc::Rc,
};
use anyhow::Result;

pub struct IrsPricer {
    discount_curve: Rc<RefCell<ZeroCurve>>,
    forward_curve: Rc<RefCell<ZeroCurve>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    past_fixing_data: Option<Rc<CloseData>>,
}

impl IrsPricer {
    pub fn new(
        discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        past_fixing_data: Option<Rc<CloseData>>,
    ) -> IrsPricer {
        IrsPricer {
            discount_curve,
            forward_curve,
            evaluation_date,
            past_fixing_data,
        }
    }
}
