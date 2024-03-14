use crate::instruments::schedule::*;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{npv_result::NpvResult, pricer::PricerTrait};
use std::collections::HashMap;
use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::calendars::nullcalendar::NullCalendar;
use crate::instruments::bond::fixed_coupon_bond::FixedCouponBond;
use time::OffsetDateTime;
use crate::time::conventions::DayCountConvention;
//
use anyhow::{anyhow, Context, Result};
use std::{rc::Rc, cell::RefCell};

pub struct FixedCouponBondPricer{
    discount_curve: Rc<RefCell<ZeroCurve>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    time_calculator: NullCalendar,
}

impl FixedCouponBondPricer {
    pub fn new(
        discount_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> FixedCouponBondPricer {
        FixedCouponBondPricer {
            discount_curve,
            evaluation_date,
            time_calculator: NullCalendar::new(),
        }
    }

    /// generate coupon-cashflow after evaluation date for bonds
    /// if include_evaluation_date is true, it will include the evaluation date
    pub fn get_coupon_cashflow(
        &self, 
        schedule: &Schedule,
        daycounter: &DayCountConvention,
        coupon_rate: Real,
        include_evaluation_date: bool,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let mut coupon_amount: Real;

        for base_schedule in schedule.iter() {
            let start_date = base_schedule.get_calc_start_date();
            let end_date = base_schedule.get_calc_end_date();
            let payment_date = base_schedule.get_payment_date();
            let amount = base_schedule.get_amount();

            if (include_evaluation_date && payment_date.date() >= eval_dt.date()) ||
                (!include_evaluation_date && payment_date.date() > eval_dt.date()) {
                match amount {
                    Some(amount) => {res.insert(payment_date.clone(), amount);},
                    None => {
                        coupon_amount = coupon_rate * self.time_calculator.year_fraction(
                            start_date, 
                            end_date,
                            daycounter
                        )?;
    
                        res.insert(payment_date.clone(), coupon_amount);
                    }
                }
            }
        }
        Ok(res)
    }
}

impl PricerTrait for FixedCouponBondPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let bond = instrument.as_fixed_coupon_bond()?;
        let mut res: Real = 0.0;
        let mut disc_factor: Real;
        let schedule = bond.get_schedule();
        let daycounter = bond.get_daycounter();
        let coupon_rate = bond.get_coupon_rate();
        let cashflow = self.get_coupon_cashflow(
            schedule, 
            daycounter, 
            coupon_rate, 
            false
        ).context("Failed to get coupon cashflow in calculating FixedCouponBond::npv")?;

        for (payment_date, amount) in cashflow.iter() {
            disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(payment_date)?;
            res += amount * disc_factor;
        }

        if !bond.is_coupon_strip() {
            let maturity = instrument.as_trait().get_maturity().unwrap();

            disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(maturity)?;
            res += disc_factor;
        }

        Ok(res)

    }

    fn fx_exposure(&self, _instrument: &Instrument, npv: Real) -> Result<Real> {
        Ok(npv)
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let bond = instrument.as_fixed_coupon_bond()?;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let mut npv: Real = 0.0;
        let mut coupon_amounts: HashMap<usize, (OffsetDateTime, Real)> = HashMap::new();
        let mut coupon_payment_probability: HashMap<usize, (OffsetDateTime, Real)> = HashMap::new();

        let mut disc_factor: Real;
        let schedule = bond.get_schedule();
        let daycounter = bond.get_daycounter();
        let coupon_rate = bond.get_coupon_rate();
        let cashflow = self.get_coupon_cashflow(
            schedule, 
            daycounter, 
            coupon_rate, 
            true
        ).context("Failed to get coupon cashflow in calculating FixedCouponBond::npv_result")?; // include evaluation date

        for (i, (payment_date, amount)) in cashflow.iter().enumerate() {
            if eval_dt.date() < payment_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(payment_date)?;
                npv += amount * disc_factor;    
            }

            if eval_dt.date() <= payment_date.date() {
                coupon_amounts.insert(i as usize, (payment_date.clone(), *amount));
                coupon_payment_probability.insert(i, (payment_date.clone(), 1.0));
            }
        }

        if !bond.is_coupon_strip() {
            let maturity = instrument.as_trait().get_maturity().unwrap();
            disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(maturity)?;
            npv += disc_factor;
        }

        let res = NpvResult::new(
            npv,
            coupon_amounts,
            coupon_payment_probability,
        );

        Ok(res)
    }       
}