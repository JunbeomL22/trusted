use crate::instruments::mock_instrument::MockInstrument;
use crate::types::{
    base::NumReprCfg,
    isin_code::IsinCode,
    venue::Venue,
};
use serde::{Serialize, Deserialize};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[enum_dispatch(InstrumentTrait)]
pub enum Instrument {
    Mock(MockInstrument)
}

#[enum_dispatch]
pub trait InstrumentTrait {
    fn get_price_num_repr_cfg(&self) -> NumReprCfg;
    fn get_quantity_num_repr_cfg(&self) -> NumReprCfg;
    fn get_price_precision(&self) -> u8;
    fn get_price_length(&self) -> u8;
    fn get_quantity_precision(&self) -> u8;
    fn get_quantity_length(&self) -> u8;
    fn get_isin_code(&self) -> &IsinCode;
    fn get_venue(&self) -> Venue;
}
