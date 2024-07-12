use crate::types::{
    base::{BookPrice, BookQuantity},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeData {
    //data_code: LocalStr, // this will be sent to another thread anyway
    venue: Venue,
    isin_code: IsinCode, // this can be spread product
    timestamp: u64,      // HHMMSSuuuuuu
    trade_price: BookPrice,
    trade_quantity: BookQuantity,
    trade_type: Option<TradeType>,
}

impl Default for TradeData {
    fn default() -> Self {
        TradeData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::from_u8_unchecked([0; 12]),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
        }
    }
}

impl TradeData {
    pub fn with_capacity(_n: usize) -> Self {
        TradeData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::from_u8_unchecked([0; 12]),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            //prev_trade_price: None,
            trade_type: None,
        }
    }
}
