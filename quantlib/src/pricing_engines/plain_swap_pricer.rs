use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::data::history_data::CloseData;
use crate::pricing_engines::{
    pricer::PricerTrait,
    npv_result::NpvResult,
};
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
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fixed_side_discount_curve: Rc<RefCell<ZeroCurve>>,
    floating_side_discount_curve: Rc<RefCell<ZeroCurve>>,
    forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
    past_fixing_data: Option<Rc<CloseData>>,
    fxs: Option<HashMap<String, Real>>,
}

impl PlainSwapPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fixed_side_discount_curve: Rc<RefCell<ZeroCurve>>,
        floating_side_discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<CloseData>>,
        fxs: Option<HashMap<String, Real>>,
    ) -> Result<PlainSwapPricer> {
        Ok(PlainSwapPricer {
            evaluation_date,
            fixed_side_discount_curve,
            floating_side_discount_curve,
            forward_curve,
            past_fixing_data,
            fxs,
        })
    }
}

impl PricerTrait for PlainSwapPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new(npv))
    }

    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let eval_date = self.evaluation_date.borrow().get_date();
        let fixed_cashflows = instrument.get_fixed_cashflows(eval_date)?;
        let floating_cashflows = instrument.get_floating_cashflows(eval_date)?;
    }
}