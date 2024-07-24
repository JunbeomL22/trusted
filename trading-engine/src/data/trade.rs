use crate::types::{
    base::{
        BookPrice, 
        BookQuantity,
        Real,
    },
    enums::{
        TradeType,
        TimeStampType,
    },
    isin_code::IsinCode,
    venue::Venue,
};
use crate::utils::numeric_converter::{
    OrderConverter,
    CumQntConverter,
    TimeStampConverter,
};

use crate::data::krx::krx_converter::{
    get_krx_base_bond_order_converter,
    get_krx_timestamp_converter,
    get_krx_base_cum_qnt_converter,
};

#[derive(Debug, Clone)]
pub struct TradeData {
    pub venue: Venue,
    pub isin_code: IsinCode, // this can be spread product
    //
    pub timestamp_type: TimeStampType,
    pub timestamp: u64,      // HHMMSSuuuuuu
    pub trade_price: BookPrice,
    pub trade_quantity: BookQuantity,
    pub trade_type: Option<TradeType>,
    //
    pub cumulative_trade_quantity: Option<BookQuantity>,
    //
    pub order_converter: &'static OrderConverter,
    pub cumulative_qnt_converter: &'static CumQntConverter,
    pub timestamp_converter: &'static TimeStampConverter,

}

impl Default for TradeData {
    fn default() -> Self {
        TradeData {
            //data_code: LocalStr::from(""),
            venue: Venue::KRX,
            isin_code: IsinCode::default(),
            timestamp_type: TimeStampType::HHMMSSuuuuuu,
            timestamp: 0,
            trade_price: 0,
            trade_quantity: 0,
            trade_type: None,
            //
            cumulative_trade_quantity: None,
            //
            order_converter: get_krx_base_bond_order_converter(),
            cumulative_qnt_converter: get_krx_base_cum_qnt_converter(),
            timestamp_converter: get_krx_timestamp_converter(),
        }
    }
}

impl TradeData {
    pub fn with_capacity(_n: usize) -> Self {
        TradeData {
            venue: Venue::KRX,
            isin_code: IsinCode::default(),
            timestamp_type: TimeStampType::HHMMSSuuuuuu,
            timestamp: 09_00_00_000000,
            //
            trade_price: 0,
            trade_quantity: 0,
            //prev_trade_price: None,
            trade_type: None,
            //
            cumulative_trade_quantity: None,
            //
            order_converter: get_krx_base_bond_order_converter(),
            cumulative_qnt_converter: get_krx_base_cum_qnt_converter(),
            timestamp_converter: get_krx_timestamp_converter(),
        }
    }

    #[inline]
    #[must_use]
    pub fn to_normalized_real(&self) -> Real {
        self.order_converter.price.normalized_real_from_i64(self.trade_price)
    }
}
