use crate::types::{
    isin_code::IsinCode,
    base::{
        BookPrice,
        BookQuantity,
    },
    venue::Venue,
};

use ustr::Ustr;

pub enum TradeType {
    Buy,
    Sell,
}

pub struct TradeTick {
    tr: Ustr,
    venue: Venue,
    isin_code: IsinCode,
    data_timestamp: u64, // HHMMSSuuuuuu
    trade_price: BookPrice,
    trade_quantity: BookQuantity,
    trade_type: Option<TradeType>,
    ask_price: Vec<BookPrice>,
    ask_quantity: Vec<BookQuantity>,
    ask_count: Vec<u32>,
    bid_price: Vec<BookPrice>,
    bid_quantity: Vec<BookQuantity>,
    bid_count: Vec<u32>,
}