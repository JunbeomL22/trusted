use crate::types::{
    precision::Precision,
    book_price::BookPrice,
    book_quantity::BookQuantity,
    types::{OrderId, TraderId},
};
use crate::types::enums::OrderSide;
//
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookOrder<T: Precision + Clone> {
    pub price: BookPrice<T>,
    pub quantity: BookQuantity<T>,
    pub order_side: OrderSide, // should I keep this? book also has its side
    pub order_id: OrderId,
    pub trader_id: Option<TraderId>, // only L3 provides this
}

