use crate::types::{
    base::{BookPrice, BookQuantity, OrderBase},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeQuoteData {
    //data_code: LocalStr, // this will be sent to another thread anyway
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    pub timestamp: u64,      // HHMMSSuuuuuu
    pub trade_price: BookPrice,
    pub trade_quantity: BookQuantity,
    pub trade_type: Option<TradeType>,
    pub ask_quote_data: Vec<OrderBase>,
    pub bid_quote_data: Vec<OrderBase>,
    pub quote_level_cut: usize, // this value indicates how many levels of order data are actually used. This can be less than the length of ask_order_data and bid_order_data
}

impl Default for TradeQuoteData {
    fn default() -> Self {
        TradeQuoteData {
            //data_code: LocalStr::from(""),
            venue: Venue::default(),
            isin_code: IsinCode::default(),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            ask_quote_data: Vec::new(),
            bid_quote_data: Vec::new(),
            quote_level_cut: 0,
        }
    }
}

impl TradeQuoteData {
    pub fn with_quote_level(level: usize) -> Self {
        TradeQuoteData {
            venue: Venue::default(),
            isin_code: IsinCode::default(),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            ask_quote_data: vec![OrderBase::default(); level],
            bid_quote_data: vec![OrderBase::default(); level],
            quote_level_cut: level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::memory_investigation::print_struct_info;

    #[test]
    fn show_me_the_memory() {
        let trade_quote_data = TradeQuoteData::default();
        print_struct_info(trade_quote_data);

        let trade_quote_data = TradeQuoteData::with_quote_level(5);
        print_struct_info(trade_quote_data);
        assert_eq!(1, 1);
    }
}
