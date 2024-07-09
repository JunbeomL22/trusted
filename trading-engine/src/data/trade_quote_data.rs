use crate::types::{
    isin_code::IsinCode,
    base::{
        BookPrice,
        BookQuantity,
        OrderData,
    },
    enums::TradeType,
    venue::Venue,
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TradeQuoteData {
    //data_code: LocalStr, // this will be sent to another thread anyway
    venue: Venue,
    isin_code: IsinCode, // this can be spread product
    timestamp: u64, // HHMMSSuuuuuu
    trade_price: BookPrice,
    trade_quantity: BookQuantity,
    trade_type: Option<TradeType>,
    // in case of spread product
    is_spread_product: bool,
    near_month_trade_price: BookPrice, // far month trade price is not necessary
    //far_month_trade_price: Option<BookPrice>,
    //
    ask_order_data: Vec<OrderData>,
    bid_order_data: Vec<OrderData>,
}

impl TradeQuoteData {
    pub fn with_capacity(n: usize) -> Self {
        TradeQuoteData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::from(""),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            //prev_trade_price: None,
            trade_type: None,
            is_spread_product: false,
            near_month_trade_price: 0,
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
