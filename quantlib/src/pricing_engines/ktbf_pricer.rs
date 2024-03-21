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
        
        let bond_pricer = FixedCouponBondPricer
        let cashflow = instrument.get_coupon_cashflow(
            Some(&pricing_date),
            None,
            None,
        ).context("Failed to get coupon cashflow in calculating FixedCouponBond::npv")?;

        for (payment_date, amount) in cashflow.iter() {
            if payment_date.date() > pricing_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(payment_date)?;
                res += amount * disc_factor;    
            }
        }

        if !instrument.is_coupon_strip()? {
            let maturity = instrument.get_maturity().unwrap();

            disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(maturity)?;
            res += disc_factor * instrument.get_redemption()?;
        }

        Ok(res)
    }
}