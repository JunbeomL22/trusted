use crate::types::{
    base::{
        BookPrice, 
        BookQuantity, 
        LevelSnapshot,
    },
    enums::TradeType,
    timestamp::{
        TimeStampType,
        TimeStamp,
    },
    isin_code::IsinCode,
    venue::Venue,
};
use crate::utils::numeric_converter::{
    OrderConverter,
    OrderCounter,
    CumQntConverter,
    TimeStampConverter,
};

use crate::data::krx::krx_converter::{
    get_krx_base_bond_order_converter,
    get_krx_base_order_counter,
    get_krx_timestamp_converter,
    get_krx_base_cum_qnt_converter,
};
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
//Serialize // this struct will not be in database directly. It will be converted to another struct
pub struct TradeQuoteSnapshot {
    //data_code: LocalStr, // this will be sent to another thread anyway
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    //
    pub timestamp_type: TimeStampType,
    pub timestamp: TimeStamp,
    //
    pub trade_price: BookPrice,
    pub trade_quantity: BookQuantity,
    pub trade_type: Option<TradeType>,
    pub ask_quote_data: Vec<LevelSnapshot>,
    pub bid_quote_data: Vec<LevelSnapshot>,
    pub quote_level_cut: usize, // this value indicates how many levels of order data are actually used. This can be less than the length of ask_order_data and bid_order_data
    //
    pub cumulative_trade_quantity: Option<BookQuantity>,
    //
    pub all_lp_holdings: Option<BookQuantity>,
    //
    #[serde(skip)]
    pub order_counter: &'static OrderCounter,
    #[serde(skip)]
    pub order_converter: &'static OrderConverter,
    #[serde(skip)]
    pub timestamp_converter: &'static TimeStampConverter,
    #[serde(skip)]
    pub cumulative_qnt_converter: &'static CumQntConverter,
}



impl Default for TradeQuoteSnapshot {
    fn default() -> Self {
        TradeQuoteSnapshot {
            venue: Venue::default(),
            isin_code: IsinCode::default(),
            //
            timestamp_type: TimeStampType::HHMMSSuuuuuu,
            timestamp: TimeStamp::new(09_00_00_000000),
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            ask_quote_data: Vec::new(),
            bid_quote_data: Vec::new(),
            quote_level_cut: 0,
            //
            cumulative_trade_quantity: None,
            //
            all_lp_holdings: None,
            //
            order_counter: get_krx_base_order_counter(),
            order_converter: get_krx_base_bond_order_converter(),
            timestamp_converter: get_krx_timestamp_converter(),
            cumulative_qnt_converter: get_krx_base_cum_qnt_converter(),
        }
    }
}

impl TradeQuoteSnapshot {
    pub fn with_quote_level(level: usize) -> Self {
        TradeQuoteSnapshot {
            venue: Venue::default(),
            isin_code: IsinCode::default(),
            //
            timestamp_type: TimeStampType::HHMMSSuuuuuu,
            timestamp: TimeStamp::new(09_00_00_000000),
            //
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            ask_quote_data: vec![LevelSnapshot::default(); level],
            bid_quote_data: vec![LevelSnapshot::default(); level],
            quote_level_cut: level,
            //
            cumulative_trade_quantity: None,
            //
            all_lp_holdings: None,
            //
            order_counter: get_krx_base_order_counter(),
            order_converter: get_krx_base_bond_order_converter(),
            timestamp_converter: get_krx_timestamp_converter(),
            cumulative_qnt_converter: get_krx_base_cum_qnt_converter(),
        }
    }

    #[inline]
    #[must_use]
    pub fn effective_bid_data(&self) -> &[LevelSnapshot] {
        &self.bid_quote_data[..self.quote_level_cut]
    }
    
    #[inline]
    #[must_use]
    pub fn effective_ask_data(&self) -> &[LevelSnapshot] {
        &self.ask_quote_data[..self.quote_level_cut]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::memory_investigation::print_struct_info;

    #[test]
    fn show_me_the_memory() {
        let trade_quote_data = TradeQuoteSnapshot::default();
        print_struct_info(trade_quote_data);

        let trade_quote_data = TradeQuoteSnapshot::with_quote_level(5);
        print_struct_info(trade_quote_data);
        assert_eq!(1, 1);
    }
}
