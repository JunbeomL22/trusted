use crate::instruments::mock_instrument::MockInstrument;
use crate::types::{id::isin_code::IsinCode, venue::Venue};
use crate::utils::numeric_converter::NumReprCfg;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
#[enum_dispatch]
pub trait InstrumentTrait {
    fn get_venue(&self) -> Venue;
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[enum_dispatch(InstrumentTrait)]
pub enum Instrument {
    Mock(MockInstrument),
}


