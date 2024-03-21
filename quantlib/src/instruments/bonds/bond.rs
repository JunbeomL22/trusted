use crate::instruments::bonds::{
    fixed_coupon_bond::FixedCouponBond,
    floating_rate_note::FloatingRateNote,
};
use crate::parameters::zero_curve::ZeroCurve;
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::instrument::InstrumentTriat;
//
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};
use time::OffsetDateTime;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bond {
    FixedCouponBond(FixedCouponBond),
    FloatingRateNote(FloatingRateNote),
}

impl Bond {
    pub fn get_coupon_cashflow(
        &self, 
        pricing_date: Option<&OffsetDateTime>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_data: Option<&Rc<CloseData>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        match self {
            Bond::FixedCouponBond(bond) => {
                bond.get_coupon_cashflow(pricing_date, forward_curve, past_data)
            },
            Bond::FloatingRateNote(bond) => {
                bond.get_coupon_cashflow(pricing_date, forward_curve, past_data)
            },
        }
    }
}