use crate::types::{
    base::{BookPrice, BookQuantity, LevelSnapshot},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};
use crate::data::krx::krx_converter::{
    get_krx_cum_qnt_converter,
    get_krx_trade_price_converter,
    get_krx_trade_qnt_converter,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TradeQuoteSnapshot {
    //data_code: LocalStr, // this will be sent to another thread anyway
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    pub timestamp: u64,      // HHMMSSuuuuuu
    pub trade_price: BookPrice,
    pub trade_quantity: BookQuantity,
    pub trade_type: Option<TradeType>,
    pub ask_quote_data: Vec<LevelSnapshot>,
    pub bid_quote_data: Vec<LevelSnapshot>,
    pub quote_level_cut: usize, // this value indicates how many levels of order data are actually used. This can be less than the length of ask_order_data and bid_order_data
    //
    pub all_lp_holdings: Option<BookQuantity>,
    //
    pub cumulative_trade_quantity: Option<BookQuantity>,
}

impl TradeQuoteSnapshot {
    pub fn with_quote_level(level: usize) -> Self {
        TradeQuoteSnapshot {
            venue: Venue::default(),
            isin_code: IsinCode::default(),
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            ask_quote_data: vec![LevelSnapshot::default(); level],
            bid_quote_data: vec![LevelSnapshot::default(); level],
            quote_level_cut: level,
            //
            all_lp_holdings: None,
            //
            cumulative_trade_quantity: None,
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
