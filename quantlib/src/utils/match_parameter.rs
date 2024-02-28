use chrono::format::Fixed;

use crate::instrument::{self, Instrument};
use crate::assets::currency::Currency;
use crate::parameters::zero_curve::ZeroCurve;
use std::collections::HashMap;
use crate::enums::CreditRating;
use crate::enums::IssuerType;

pub struct MatchPrameter {
    repo_curve_map: HashMap<&'static str, ZeroCurve>, // 
    collaterl_curve_map: HashMap<&'static str, ZeroCurve>, // currency -> collateral curve name
}

impl MatchPrameter {
    pub fn get_discount_curve_name(&self, instrument: &Instrument) -> &'static str {
        match instrument {
            Instrument::FixedCouponBond(instrument) |
            Instrument::FloatingRateNote(instrument) => {
                self.get_bond_discount_curve(
                    instrument.get_currency(),
                    instrument.get_credit_rating(),
                    instrument.get_issuer_type(),
                )
            },
            Instrument::IRS(instrument) => {
                instrument.get_rate_forward_curve_name()
            },
            Instrument::StockFutures(_) |
            Instrument::BondFutures(_) |
            Instrument::KTBF(_) => "StockFutures is not discounted",
        }
    }

    pub fn get_bond_discount_curve(
        &self, 
        currency: &Currency,
        credit_rating: Option<&CreditRating>,
        issuer_type: Option<&IssuerType>,
    ) -> &'static str {
        &"not yet implemented"
    }

    pub fn get_collateral_curve_name(&self, instrument: &Instrument) -> &'static str {
        match instrument {
            Instrument::StockFutures(instrument) |
            Instrument::BondFutures(instrument) |
            Instrument::KTBF(instrument) => {
                self.collaterl_curve_map.get(instrument.get_currency())
            },
            Instrument::FixedCouponBond(_) |
            Instrument::FloatingRateNote(_) |
            Instrument::IRS(_)=> {
                "collateral curve is not needed for bonds"
            },
        }
    }
    }

    pub fn get_repo_curve(&self, instrument: &Instrument) -> String {
        match instrument {
            Instrument::FixedCouponBond(instrument) => {"Bond (BondFutures my need) does not need repo".to_string()},
            Instrument::StockFutures(instrument) => {
                match instrument.get_currency() {
                    Currency::KRW => "KRW".to_string(),
                    _ => "Undefined".to_string(),
            }
        }
    }
    }
}