use crate::pricing_engines::pricer::PricerTrait;
use crate::instrument::Instrument;
use crate::pricing_engines::npv_result::NpvResult;
use crate::definitions::Real;
//
use anyhow::Result;
pub struct CashPricer {}

impl CashPricer {
    pub fn new() -> CashPricer {
        CashPricer {}
    }
}

impl PricerTrait for CashPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }

    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        Ok(1.0)
    }
}