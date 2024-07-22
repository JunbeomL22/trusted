use crate::types::base::{
    Real,
    MilliTimeStamp,
};
use crate::data::{
    trade_quote::TradeQuoteSnapshot,
    trade::TradeData,
};
use crate::utils::numeric_converter::IntegerConverter;

#[derive(Debug, Clone)]
pub struct TradePrice {
    value: Real,
    timestamp: MilliTimeStamp,
    converter: IntegerConverter,
}

impl Default for TradePrice {
    fn default() -> Self {
        Self {
            value: 0.0,
            timestamp: MilliTimeStamp { stamp: 0 },
            converter: IntegerConverter::default(),
        }
    }
}

impl TradePrice {
    pub fn from_trade_data(data: &TradeData) -> Self {
        Self {
            value: data.to_normalized_real(),
            timestamp: data.timestamp,
        }
    }
}