use crate::pricing_engines::pricer::PricerTrait;
use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::market_price::MarketPrice;
use crate::pricing_engines::npv_result::NpvResult;
//
use anyhow::Result;
use std::{
    rc::Rc,
    cell::RefCell,
};

pub struct NullPricer {
    market_price: Rc<RefCell<MarketPrice>>,
}

impl NullPricer {
    pub fn new(market_price: Rc<RefCell<MarketPrice>>) -> NullPricer {
        NullPricer {
            market_price,
        }
    }
}

impl PricerTrait for NullPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        Ok(self.market_price.borrow().get_value())
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        Ok(NpvResult::new_from_npv(self.npv(instrument)?))
    }
}