use crate::types::{
    base::{BookPrice, BookQuantity},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeData {
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    pub timestamp: u64,      // HHMMSSuuuuuu
    pub trade_price: BookPrice,
    pub trade_quantity: BookQuantity,
    pub trade_type: Option<TradeType>,
    //
    pub cumulative_trade_quantity: Option<BookQuantity>,
}

impl Default for TradeData {
    fn default() -> Self {
        TradeData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::default(),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            //
            cumulative_trade_quantity: None,
        }
    }
}

impl TradeData {
    pub fn with_capacity(_n: usize) -> Self {
        TradeData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: unsafe { IsinCode::from_u8_unchecked(&[0; 12]) },
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            //prev_trade_price: None,
            trade_type: None,
            //
            cumulative_trade_quantity: None,
        }
    }
}
