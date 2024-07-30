use crate::types::{
    base::{
        LevelSnapshot,
        BookQuantity,
    },
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
    TimeStampConverter,
};

use crate::data::krx::krx_converter::{
    get_krx_base_bond_order_converter,
    get_krx_base_order_counter,
    get_krx_timestamp_converter,
};
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct QuoteSnapshot {
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    //
    pub timestamp_type: TimeStampType,
    pub timestamp: TimeStamp,
    //
    pub ask_quote_data: Vec<LevelSnapshot>,
    pub bid_quote_data: Vec<LevelSnapshot>,
    pub quote_level_cut: usize, // this value indicates how many levels of order data are actually used. This can be less than the length of ask_order_data and bid_order_data
    //
    pub all_lp_holdings: Option<BookQuantity>,
    //
    #[serde(skip)]
    pub order_counter: &'static OrderCounter,
    #[serde(skip)]
    pub order_converter: &'static OrderConverter,
    #[serde(skip)]
    pub timestamp_converter: &'static TimeStampConverter,
}

impl Default for QuoteSnapshot {
    fn default() -> Self {
        QuoteSnapshot {
            venue: Venue::default(),
            isin_code: IsinCode::default(),
            timestamp_type: TimeStampType::HHMMSSuuuuuu,
            timestamp: TimeStamp::new(0),
            ask_quote_data: Vec::new(),
            bid_quote_data: Vec::new(),
            quote_level_cut: 0,
            //
            all_lp_holdings: None,
            //
            order_counter: get_krx_base_order_counter(),
            order_converter: get_krx_base_bond_order_converter(),
            timestamp_converter: get_krx_timestamp_converter(),
        }
    }

}

impl QuoteSnapshot {
    pub fn with_quote_level(level: usize) -> Self {
        QuoteSnapshot {
            venue: Venue::KRX,
            isin_code: IsinCode::default(),
            //
            timestamp_type: TimeStampType::HHMMSSuuuuuu,
            timestamp: TimeStamp::new(19700101),
            ask_quote_data: vec![LevelSnapshot::default(); level],
            bid_quote_data: vec![LevelSnapshot::default(); level],
            quote_level_cut: level,
            //
            all_lp_holdings: None,
            //
            order_counter: get_krx_base_order_counter(),
            order_converter: get_krx_base_bond_order_converter(),
            timestamp_converter: get_krx_timestamp_converter(),
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
        let trade_quote_data = QuoteSnapshot::with_quote_level(5);
        print_struct_info(trade_quote_data);
        assert_eq!(1, 1);
    }
}
