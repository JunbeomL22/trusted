use crate::types::base::{
    BookPrice,
    BookQuantity,
    OrderId,
    Real,
};
use crate::types::timestamp::TimeStamp;
use crate::data::order::LimitOrder;
//
use rustc_hash::FxHashMap;

/// SimpleTaker enters a position and then acts only either to take profit or to stop loss.
/// If the bid price is higher than the bid_upper, it will sell.
/// If the ask price is lower than the ask_lower, it will buy.
/// If timeoout is reached, it will cancel all orders.
#[derive(Debug, Clone, Default)]
pub struct SimpleTaker {
    pub enter_price: FxHashMap<OrderId, BookPrice>,
    pub bid_upper: BookPrice,
    pub ask_lower: BookPrice,
    pub timestamp: TimeStamp,
    pub timeout_milli: Real,
    pub requested_orders: Vec<LimitOrder>,
    pub confirmed_orders: Vec<OrderId>,
    pub trade_amount: FxHashMap<OrderId, BookQuantity>
} 