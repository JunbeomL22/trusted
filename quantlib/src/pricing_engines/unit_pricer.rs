use crate::pricing_engines::pricer::PricerTrait;
use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::pricing_engines::npv_result::NpvResult;
//
use anyhow::Result;

#[derive(Default)]
pub struct UnitPricer {}

impl UnitPricer {
    pub fn new() -> UnitPricer {
        UnitPricer { }
    }
}

impl PricerTrait for UnitPricer {
    fn npv(&self, _instrument: &Instrument) -> Result<Real> {
        Ok(1.0)
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        Ok(NpvResult::new_from_npv(self.npv(instrument)?))
    }
}