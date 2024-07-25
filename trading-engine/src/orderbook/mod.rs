pub mod half_book;
pub mod level;

use crate::orderbook::half_book::HalfBook;
use crate::types::{
    isin_code::IsinCode,
    venue::Venue,
};

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: HalfBook,
    pub asks: HalfBook,
    isin_code: IsinCode,
    venue: Venue,
}