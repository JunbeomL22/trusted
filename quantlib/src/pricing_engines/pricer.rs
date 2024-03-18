use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::pricing_engines::npv_result::NpvResult;
use anyhow::Result;

pub trait PricerTrait {
    // Code -> NPV
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult>;
    fn npv(&self, instrument: &Instrument) -> Result<Real>;
    fn fx_exposure(&self, _instrument: &Instrument, npv: Real) -> Result<Real> { Ok(npv) }
    /*
    fn coupons(
        &self, 
        instrument: &Instrument,
        start_date: &OffsetDateTime,
        end_date: &OffsetDateTime,
    ) -> Result<HashMap<OffsetDateTime, Real>, MyError>;
    */
}

pub enum Pricer {
    StockFuturesPricer(Box<dyn PricerTrait>),
    FixedCouponBondPricer(Box<dyn PricerTrait>),
}

impl Pricer {
    pub fn as_trait(&self) -> &(dyn PricerTrait) {
        match self {
            Pricer::StockFuturesPricer(pricer) => &**pricer,
            Pricer::FixedCouponBondPricer(pricer) => &**pricer,
        }
    }
}