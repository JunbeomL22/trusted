use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::utils::myerror::MyError;
//use time::OffsetDateTime;
//use std::collections::HashMap;
use crate::pricing_engines::npv_result::NpvResult;

pub trait PricerTrait {
    // Code -> NPV
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult, MyError>;
    fn npv(&self, instrument: &Instrument) -> Result<Real, MyError>;
    fn fx_exposure(&self, instrument: &Instrument) -> Result<Real, MyError>;
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
}

impl Pricer {
    pub fn as_trait(&self) -> &(dyn PricerTrait) {
        match self {
            Pricer::StockFuturesPricer(pricer) => &**pricer,
        }
    }
}