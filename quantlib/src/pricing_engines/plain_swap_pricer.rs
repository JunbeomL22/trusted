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
    collections::HashMap,
};
use anyhow::Result;

pub struct PlainSwapPricer {
    discount_curve: Rc<RefCell<ZeroCurve>>,
    forward_curve: Rc<RefCell<ZeroCurve>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    past_fixing_data: Option<Rc<CloseData>>,
    fxs: Option<HashMap<String, Real>>,
}

impl PlainSwapPricer {
    pub fn new(
        discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        past_fixing_data: Option<Rc<CloseData>>,
        fxs: Option<HashMap<String, Real>>,
    ) -> PlainSwapPricer {
        PlainSwapPricer {
            discount_curve,
            forward_curve,
            evaluation_date,
            past_fixing_data,
            fxs,
        }
    }
}
