use crate::types::{
    isin_code::IsinCode,
    venue::Venue,
};
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Mock {
    ising_code: IsinCode,
    venue: Venue,
    //
    unit_amount: u32,
    //
    last_trade_date: OffsetDateTime,
    last_trade_date_unix_nano: u64,
    //
    pub price_precision: u8,
    pub price_length: u8,
    //
    pub quantity_precision: u8,
    pub quantity_length: u8,
    //
}