use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::npv_result::NpvResult;
use crate::pricing_engines::pricer::PricerTrait;
//
use anyhow::Result;
pub struct CashPricer {}

impl CashPricer {
    pub fn new() -> CashPricer {
        CashPricer {}
    }
}

impl Default for CashPricer {
    fn default() -> CashPricer {
        CashPricer::new()
    }
}

impl PricerTrait for CashPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }

    fn npv(&self, _instrument: &Instrument) -> Result<Real> {
        Ok(1.0)
    }
}
