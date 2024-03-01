use crate::instrument::Instrument;
use crate::definitions::Real;

pub trait PricerTrait {
    // Code -> NPV
    fn npv(&self, instrument: &Instrument) -> Real;
    fn fx_exposure(&self, instrument: &Instrument) -> Real;
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