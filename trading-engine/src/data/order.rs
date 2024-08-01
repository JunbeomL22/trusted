use crate::types::base::{BookPrice, BookQuantity, OrderId};
use crate::types::isin_code::IsinCode;
use crate::types::enums::OrderSide;
use crate::types::venue::Venue;
use crate::types::timestamp::TimeStamp;

//
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait OrderRequest {}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LimitOrder {
    pub price: BookPrice,
    pub quantity: BookQuantity,
    pub order_side: OrderSide, // should I keep this? book also has its side
    pub order_id: OrderId,
}

impl LimitOrder {
    #[inline]
    pub fn new(
        price: BookPrice,
        quantity: BookQuantity,
        order_side: OrderSide,
        order_id: OrderId,
    ) -> Self {
        Self {
            price,
            quantity,
            order_side,
            order_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketOrder {
    pub quantity: BookQuantity,
    pub order_side: OrderSide,
    pub order_id: OrderId,
}

impl MarketOrder {
    #[inline]
    pub fn new(quantity: BookQuantity, order_side: OrderSide, order_id: OrderId) -> Self {
        Self {
            quantity,
            order_side,
            order_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CancelOrder {
    pub order_id: OrderId,
}

impl CancelOrder {
    #[inline]
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModifyOrder {
    pub order_id: OrderId,
    pub price: BookPrice,
    pub quantity: BookQuantity,
}

impl ModifyOrder {
    #[inline]
    pub fn new(order_id: OrderId, price: BookPrice, quantity: BookQuantity) -> Self {
        Self {
            order_id,
            price,
            quantity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// This is a result of reverse engineering from L2 order book to order request
/// When it comes to order book, we don't need to know the order_id. 
/// It just removes an order from the last order at each price level.
pub struct RemoveAnyOrder {
    pub price: BookPrice,
    pub quantity: BookQuantity,
    pub order_side: OrderSide,
}

impl RemoveAnyOrder {
    #[inline]
    pub fn new(price: BookPrice, quantity: BookQuantity, order_side: OrderSide) -> Self {
        Self {
            price,
            quantity,
            order_side,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderEnum {
    LimitOrder(LimitOrder),
    RemoveAnyOrder(RemoveAnyOrder),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecomposedOrder {
    pub order: OrderEnum,
    pub timestamp: TimeStamp,
    pub isin_code: IsinCode,
    pub venue: Venue,
}

impl OrderRequest for LimitOrder {}
impl OrderRequest for MarketOrder {}
impl OrderRequest for CancelOrder {}
impl OrderRequest for ModifyOrder {}
impl OrderRequest for RemoveAnyOrder {}


#[cfg(test)]
mod tests {

    #[test]
    fn test_book_order() {
        use super::*;
        use crate::types::base::{BookPrice, BookQuantity, OrderId};
        use crate::types::enums::OrderSide;

        let price: BookPrice = 100;
        let quantity: BookQuantity = 100;
        let order_side: OrderSide = OrderSide::Bid;
        let order_id: OrderId = 1;

        let book_order = LimitOrder::new(price, quantity, order_side, order_id);

        assert_eq!(book_order.price, price);
        assert_eq!(book_order.quantity, quantity);
        assert_eq!(book_order.order_side, order_side);
        assert_eq!(book_order.order_id, order_id);
    }
}
