use crate::types::{
    isin_code::IsinCode,
    venue::Venue,
};
use crate::instruments::instrument::InstrumentTrait;
use crate::types::base::NumberRepresentationConfigurations;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct MockInstrument {
    ising_code: IsinCode,
    venue: Venue,
    //
    unit_amount: u32,
    //
    last_trade_date: OffsetDateTime,
    last_trade_date_unix_nano: u64,
    //
    price_precision_config: NumberRepresentationConfigurations,
    //
    quantity_precision_config: NumberRepresentationConfigurations,
    //
}

impl InstrumentTrait for MockInstrument {
    #[inline]
    #[must_use]
    fn get_price_precision(&self) -> u8 { self.price_precision_config.precision }
        
    #[inline]
    #[must_use]
    fn get_price_length(&self) -> u8 { self.price_precision_config.length }

    #[inline]
    #[must_use]
    fn get_quantity_precision(&self) -> u8 { self.quantity_precision_config.precision }

    #[inline]
    #[must_use]
    fn get_quantity_length(&self) -> u8 { self.quantity_precision_config.length }

    #[inline]
    #[must_use]
    fn get_isin_code(&self) -> &IsinCode { &self.ising_code }

    #[inline]
    #[must_use]
    fn get_venue(&self) -> Venue { self.venue }
}
