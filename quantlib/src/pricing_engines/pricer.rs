use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::utils::myerror::MyError;
use time::OffsetDateTime;
use std::collections::HashMap;

pub trait PricerTrait<'a> {
    // Code -> NPV
    fn npv(&self, instrument: &'a Instrument<'a>) -> Result<Real, MyError>;
    fn fx_exposure(&self, instrument: &'a Instrument<'a>) -> Result<Real, MyError>;
    fn coupons(
        &self, 
        instrument: &'a Instrument<'a>,
        start_date: &OffsetDateTime,
        end_date: &OffsetDateTime,
    ) -> Result<HashMap<OffsetDateTime, Real>, MyError>;
}

pub enum Pricer<'a> {
    StockFuturesPricer(Box<dyn PricerTrait<'a>>),
}

impl<'a> Pricer<'a> {
    pub fn as_trait(&self) -> &(dyn PricerTrait<'a>) {
        match self {
            Pricer::StockFuturesPricer(pricer) => &**pricer,
        }
    }
}