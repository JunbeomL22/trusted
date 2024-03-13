use crate::instruments::schedule::Schedule;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{npv_result::NpvResult, pricer::PricerTrait};
use std::collections::HashMap;
use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::calendars::nullcalendar::NullCalendar;
use crate::instruments::bond::fixed_coupon_bond::FixedCouponBond;
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

    pub fn make_cashflow(
        &self, 
        schedule: Schedule,
        daycounter: DayCountConvention,
        coupon_rate: Real,
        include_evaluation_date: bool,
    ) -> HashMap<OffsetDateTime, Real> {
    }
}

impl PricerTrait for FixedCouponBondPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        match instrument {
            Instrument::FixedCouponBond(bond) => {
                let mut npv = 0.0;
                let discount_curve = self.discount_curve.borrow();
                let evaluation_date = self.evaluation_date.borrow();
                let calendar = self.time_calculator.as_trait();
                let start_date = evaluation_date.get_date();
                let end_date = bond.get_maturity_date();
                let coupon_dates = bond.get_coupon_dates();
                let coupon_amounts = bond.get_coupon_amounts();
                let face_value = bond.get_face_value();
                let coupon_rate = bond.get_coupon_rate();
                let frequency = bond.get_coupon_frequency();
                let day_count = bond.get_day_count();
                let coupon_payment_probability = bond.get_coupon_payment_probability();
                let discount_factor = discount_curve.get_discount_factor(start_date, end_date, calendar)?;
                for (i, coupon_date) in coupon_dates.iter().enumerate() {
                    let coupon_amount = coupon_amounts[i];
                    let coupon_payment_date = coupon_date;
                    let coupon_payment_probability = coupon_payment_probability[i];
                    let coupon_discount_factor = discount_curve.get_discount_factor(start_date, *coupon_payment_date, calendar)?;
                    npv += coupon_amount * coupon_discount_factor * coupon_payment_probability;
                }
                npv += face_value * discount_factor;
                Ok(npv)
            },
            _ => {

                Err(anyhow!(
                    "Invalid instrument type for FixedCouponBondPricer\n\
                    name: {}, code = {}",
                    instrument.as_trait().get_name(),
                    instrument.as_trait().get_code(),
                ))
            },
    }

    fn fx_exposure(&self, instrument: &Instrument, npv: Real) -> Result<Real> {
        Ok(npv)
    }
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        
}