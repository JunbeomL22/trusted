use crate::types::{
    base::OrderBase,
    isin_code::IsinCode,
    venue::Venue,
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct QuoteData {
    venue: Venue,
    isin_code: IsinCode, // this can be spread product
    timestamp: u64,      // HHMMSSuuuuuu
    ask_order_data: Vec<OrderBase>,
    bid_order_data: Vec<OrderBase>,
}

impl QuoteData {
    pub fn with_capacity(n: usize) -> Self {
        QuoteData {
            venue: Venue::KRX,
            isin_code: IsinCode::from_u8_unchecked([0; 12]),
            timestamp: 0,
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
        let trade_quote_data = QuoteData::with_capacity(5);
        print_struct_info(trade_quote_data);
        assert_eq!(1, 1);
    }
}
