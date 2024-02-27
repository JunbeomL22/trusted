use crate::definitions::Real;
use crate::assets::currency::Currency;
use time::OffsetDateTime;

pub trait InstrumentTriat {
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
    fn get_unit_notional(&self) -> Real;
    fn get_type_name(&self) -> &'static str;
    fn get_maturity(&self) -> &Option<OffsetDateTime>;
    fn get_underlying_names(&self) -> &Vec<String>;
}
pub enum Instrument {
    StockFutures(Box<dyn InstrumentTriat>),
    FixedCouponBond(Box<dyn InstrumentTriat>),
}

impl Instrument {
    pub fn as_trait(&self) -> &(dyn InstrumentTriat) {
        match self {
            Instrument::StockFutures(instrument) => &**instrument,
            Instrument::FixedCouponBond(instrument) => &**instrument,
            // Other variants...
        }
    }

    pub fn get_underlying_names(&self) -> Vec<String> {
        match self {
            Instrument::StockFutures(instrument) => instrument.get_underlying_names().clone(),
            _ => Vec::<String>::new()
        }
    }
}

pub struct Instruments {
    instruments: Vec<Instrument>,
}