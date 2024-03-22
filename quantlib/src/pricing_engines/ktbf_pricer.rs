use crate::instruments::schedule::*;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{
    npv_result::NpvResult, 
    pricer::PricerTrait,
    krx_yield_pricer::KrxYieldPricer,
    fixed_coupon_bond_pricer::FixedCouponBondPricer,
};
use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::time::calendars::{
    calendar_trait::CalendarTrait,
    null_calendar::NullCalendar,
};
use crate::instruments::bonds::fixed_coupon_bond::FixedCouponBond;
use crate::time::conventions::DayCountConvention;
//
use anyhow::{anyhow, Context, Result};
use std::{
    rc::Rc, 
    cell::RefCell,
    collections::HashMap,
};
use time::OffsetDateTime;

pub struct KtbfPricer {
    discount_curve: Rc<RefCell<ZeroCurve>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    time_calculator: NullCalendar,
}

impl KtbfPricer {
    pub fn new(
        discount_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> KtbfPricer {
        KtbfPricer {
            discount_curve,
            evaluation_date,
            time_calculator: NullCalendar::new(),
        }
    }
}

impl PricerTrait for KtbfPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let mut res: Real = 0.0;
        let mut disc_factor: Real;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let pricing_date = instrument.get_maturity().unwrap();
        
        let bond_pricer = BondPricer::new(
            self.discount_curve.clone(),
            None,
            self.evaluation_date.clone(),
            None,
        );
        
        let mut bond_yields = Vec::new();
        let underlying_bonds = instrument.get_underlying_bonds();

        let krx_yield_pricer = KrxYieldPricer::new(
            0.0,
            None,
            self.evaluation_date.clone(),
            None,
        );

        let init_guess = self.discount_curve.borrow().get_forward_rate_from_evaluation_date(
            underlying_bonds[0].get_maturity().unwrap(),
        );

        for bond in underlying_bonds.iter_mut() {
            bond.set_pricing_date(Some(pricing_date.clone()));
            let npv = bond_pricer.npv(Instrument::Bond(bond))?;
            let yield_ = krx_yield_pricer.find_yield(
                Instrument::Bond(bond), 
                npv,
                init_guess
            )?;

            bond_yields.push(yield_);
        }

        let average_yield = bond_yields.iter().sum::<Real>() / bond_yields.len() as Real;

        let ktbf_price = instrument.get_virtual_bond_npv(average_yield)?;

        Ok(ktbf_price)
    }
}