use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::parameters::market_price::MarketPrice;
use crate::pricing_engines::npv_result::NpvResult;
use crate::pricing_engines::pricer::PricerTrait;
//
use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

pub struct IdentityPricer {
    market_price: Rc<RefCell<MarketPrice>>,
}

impl IdentityPricer {
    pub fn new(market_price: Rc<RefCell<MarketPrice>>) -> IdentityPricer {
        IdentityPricer { market_price }
    }
}

impl PricerTrait for IdentityPricer {
    fn npv(&self, _instrument: &Instrument) -> Result<Real> {
        Ok(self.market_price.borrow().get_value())
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        Ok(NpvResult::new_from_npv(self.npv(instrument)?))
    }
}
