use crate::types::{
    base::{
        LevelSnapshot,
        BookQuantity,
    },
    isin_code::IsinCode, 
    venue::Venue,
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct QuoteSnapshot {
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    pub timestamp: u64,      // HHMMSSuuuuuu
    pub ask_quote_data: Vec<LevelSnapshot>,
    pub bid_quote_data: Vec<LevelSnapshot>,
    pub quote_level_cut: usize, // this value indicates how many levels of order data are actually used. This can be less than the length of ask_order_data and bid_order_data
    //
    pub lp_ask_quote_data: Vec<LevelSnapshot>,
    pub lp_bid_quote_data: Vec<LevelSnapshot>,
    pub lp_quote_level_cut: usize,
    pub lp_holdings: Option<BookQuantity>,
}

impl QuoteSnapshot {
    pub fn with_quote_level(level: usize) -> Self {
        QuoteSnapshot {
            venue: Venue::KRX,
            isin_code: IsinCode::default(),
            timestamp: 0,
            ask_quote_data: vec![LevelSnapshot::default(); level],
            bid_quote_data: vec![LevelSnapshot::default(); level],
            quote_level_cut: level,
            //
            lp_ask_quote_data: vec![],
            lp_bid_quote_data: vec![],
            lp_quote_level_cut: 0,
            lp_holdings: None,
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

    #[inline]
    #[must_use]
    pub fn effective_lp_bid_data(&self) -> &[LevelSnapshot] {
        &self.lp_bid_quote_data
    }

    #[inline]
    #[must_use]
    pub fn effective_lp_ask_data(&self) -> &[LevelSnapshot] {
        &self.lp_ask_quote_data
    }

    #[inline]
    #[must_use]
    pub fn effective_lp_holdings(&self) -> Option<BookQuantity> {
        self.lp_holdings
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
