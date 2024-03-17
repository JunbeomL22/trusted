use crate::instruments::bonds::{
    fixed_coupon_bond::FixedCouponBond,
    floating_rate_note::FloatingRateNote,
};
//
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bond {
    FixedCouponBond(FixedCouponBond),
    FloatingRateNote(FloatingRateNote),
}