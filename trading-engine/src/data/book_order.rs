use crate::types::base::{
    OrderId,
    BookPrice,
    BookQuantity,
};
use crate::types::enums::OrderSide;
//
use serde::{Deserialize, Serialize};
use std::fmt::Debug;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookOrder {
    pub price: BookPrice,
    pub quantity: BookQuantity,
    pub order_side: OrderSide, // should I keep this? book also has its side
    pub order_id: OrderId,
}

impl BookOrder {
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



#[cfg(test)]
mod tests {

    #[test]
    fn test_book_order() {
        use crate::types::enums::OrderSide;
        use crate::types::base::{
            OrderId,
            BookPrice,
            BookQuantity,
        };
        use super::*;

        let price: BookPrice = 100;
        let quantity: BookQuantity = 100;
        let order_side: OrderSide = OrderSide::Bid;
        let order_id: OrderId = 1;

        let book_order = BookOrder::new(price, quantity, order_side, order_id);

        assert_eq!(book_order.price, price);
        assert_eq!(book_order.quantity, quantity);
        assert_eq!(book_order.order_side, order_side);
        assert_eq!(book_order.order_id, order_id);
    }
}