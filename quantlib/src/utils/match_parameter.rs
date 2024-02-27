use crate::instrument::Instrument;
use crate::assets::currency::Currency;
use crate::parameters::zero_curve::ZeroCurve;
use std::collections::HashMap;

pub struct MatchPrameter {
    repo_curve: HashMap<String, ZeroCurve>, // 
}

impl MatchPrameter {
    pub fn get_discount_curve_name(&self, instrument: &Instrument) -> String {
        match instrument {
            Instrument::FixedCouponBond(instrument) => {
                instrument.get_currency().as_str().to_string() + "GOV"
            },
            Instrument::StockFutures(_) => "StockFutures is not discounted".to_string(),
        }
    }

    pub fn get_collateral_curve(&self, instrument: &Instrument) -> String {
        match instrument {
            Instrument::FixedCouponBond(instrument) => {"Bond (BondFutures may need) does not need collateral".to_string()},
            Instrument::StockFutures(instrument) => {
                match instrument.get_currency() {
                    Currency::KRW => "KSD".to_string(),
                    _ => "Undefined".to_string(),
            }
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