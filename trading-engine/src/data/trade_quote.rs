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
    // in case of spread product
    // is_spread_product: bool,
    // near_month_trade_price: BookPrice, // far month trade price is not necessary
    // far_month_trade_price: Option<BookPrice>,
    //
    pub ask_order_data: Vec<OrderBase>,
    pub bid_order_data: Vec<OrderBase>,
}

impl Default for TradeQuoteData {
    fn default() -> Self {
        TradeQuoteData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::from_u8_unchecked([0; 12]),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            ask_order_data: Vec::new(),
            bid_order_data: Vec::new(),
        }
    }
}

impl TradeQuoteData {
    pub fn with_capacity(n: usize) -> Self {
        TradeQuoteData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::from_u8_unchecked([0; 12]),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            //prev_trade_price: None,
            trade_type: None,
            //is_spread_product: false,
            //near_month_trade_price: 0,
            ask_order_data: Vec::with_capacity(n),
            bid_order_data: Vec::with_capacity(n),
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
        assert_eq!(1, 1);
    }
}
