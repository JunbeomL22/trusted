use crate::instruments::schedule::*;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{npv_result::NpvResult, pricer::PricerTrait};
use std::collections::HashMap;
use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::calendars::nullcalendar::NullCalendar;
use crate::instruments::bonds::fixed_coupon_bond::FixedCouponBond;
use time::OffsetDateTime;
use crate::time::conventions::DayCountConvention;

//
use anyhow::{anyhow, Context, Result};
use std::{rc::Rc, cell::RefCell};


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