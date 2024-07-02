use crate::types::{
    precision::Precision,
    book_price::BookPrice,
    book_quantity::BookQuantity,
    venue::OrderId,
};
use crate::types::enums::OrderSide;
//
use serde::{Deserialize, Serialize};
use std::fmt::Debug;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookOrder
<T: Precision + Clone + Debug, S: Precision + Clone + Debug> {
    pub price: BookPrice<T>,
    pub quantity: BookQuantity<S>,
    pub order_side: OrderSide, // should I keep this? book also has its side
    pub order_id: OrderId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::precision::{
        Prec3,
        Prec0,
    };
    use crate::types::book_price::BookPrice;
    use crate::types::book_quantity::BookQuantity;
    use crate::types::{
        venue::OrderId,
        venues::mock_exchange::MockOrderId,
    };
    use crate::types::enums::OrderSide;
    use ustr::Ustr;

    #[test]
    fn test_book_order() {
        let price = BookPrice::<Prec3>::new(100.1234).unwrap();
        let quantity = BookQuantity::<Prec0>::new(100.0).unwrap();
        let _order_id = MockOrderId::new(1);
        let order_id = OrderId::MockOrderId(_order_id);
        let order_side = OrderSide::Buy;

        let book_order = BookOrder {
            price,
            quantity,
            order_side,
            order_id,
        };
        assert_eq!(book_order.price.as_f64(), 100.123);
        assert_eq!(book_order.quantity.as_f64(), 100.0);
        assert_eq!(book_order.order_side, OrderSide::Buy);
        assert_eq!(book_order.order_id, 1);
    }
}