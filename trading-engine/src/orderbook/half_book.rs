use crate::data::book_order::BookOrder;
use crate::orderbook::level::Level;
use crate::types::{
    base::{BookPrice, OrderId, BookQuantity},
    enums::OrderSide,
};
use crate::topics::LogTopic;
//
use anyhow::Result;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct HalfBook {
    pub order_side: OrderSide,
    // wow! this can be gigantic
    // where cancel or trade, it must be removed
    pub levels: BTreeMap<BookPrice, Level>, 
    pub cache: FxHashMap<OrderId, BookPrice>, 
}

impl HalfBook {
    #[must_use]
    pub fn initialize(order_side: OrderSide) -> Self {
        HalfBook {
            order_side,
            levels: BTreeMap::new(),
            cache: FxHashMap::default(),
        }
    }

    #[must_use]
    pub fn initialize_with_order(order: BookOrder) -> Self {
        let mut levels = BTreeMap::new();
        let price = order.price;
        let level = Level::initialize_with_order(order.clone());
        levels.insert(price, level);
        let mut cache = FxHashMap::default();
        cache.insert(order.order_id, price);
        HalfBook {
            order_side: order.order_side,
            levels,
            cache,
        }
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.levels.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.levels.clear();
        self.cache.clear();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    #[inline]
    pub fn add_order(&mut self, order: BookOrder) -> Result<()> {
        self.cache.insert(order.order_id, order.price);

        match self.levels.get_mut(&order.price) {
            Some(level) => {
                level.add_order(order)?;
            }
            None => {
                let level = Level::initialize_with_order(order.clone());
                self.levels.insert(order.price, level);
            }
        }

        Ok(())
    }

    /// these actions are not Result type because the program should continue even if the order is not found
    #[inline]
    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<()> {
        if let Some(price) = self.cache.remove(&order_id) {
            if let Some(level) = self.levels.get_mut(&price) {
                if let Some(_) = level.cancel_order(order_id) {
                    self.cache.remove(&order_id);
                    return Some(())
                }
            }
        }
        let msg = format!("Order not found in CancelOrder id = {}", order_id);
        crate::log_warn!(LogTopic::OrderNotFound, message = msg);
        None
    }

    #[inline]
    pub fn change_price(&mut self, order_id: OrderId, new_price: BookPrice) -> Option<()> {
        /* fill up */

        let msg = format!("Order not found in ChangePrice id = {}, price = {}", order_id, new_price);
        crate::log_warn!(LogTopic::OrderNotFound, message = msg);
        None
    }


    pub fn change_quantity(&mut self, order_id: OrderId, new_quantity: BookQuantity) -> Option<()> {
        if let Some(price) = self.cache.get(&order_id) {
            if let Some(level) = self.levels.get_mut(&price) {
                // find and change the quantity only. do not cancel
                for (k, v) in level.orders.iter_mut() {
                    if *k == order_id {
                        *v = new_quantity;
                        return Some(())
                    }
                }
            }
        }

        let msg = format!("Order not found in ChangeQuantity id = {}, quantity = {}", order_id, new_quantity);
        crate::log_warn!(LogTopic::OrderNotFound, message = msg);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::OrderSide;
    use crate::data::book_order::BookOrder;
    use crate::types::base::BookQuantity;

    #[test]
    fn test_half_book() {
        let mut half_book = HalfBook::initialize(OrderSide::Bid);
        let order1 = BookOrder {
            order_id: 1,
            price: 100,
            quantity: 10,
            order_side: OrderSide::Bid,
        };
        let order2 = BookOrder {
            order_id: 2,
            price: 100,
            quantity: 20,
            order_side: OrderSide::Bid,
        };
        let order3 = BookOrder {
            order_id: 3,
            price: 101,
            quantity: 30,
            order_side: OrderSide::Bid,
        };
        let order4 = BookOrder {
            order_id: 4,
            price: 102,
            quantity: 40,
            order_side: OrderSide::Bid,
        };
        let order5 = BookOrder {
            order_id: 5,
            price: 103,
            quantity: 50,
            order_side: OrderSide::Bid,
        };
        let order6 = BookOrder {
            order_id: 6,
            price: 104,
            quantity: 60,
            order_side: OrderSide::Bid,
        };
        let order7 = BookOrder {
            order_id: 7,
            price: 105,
            quantity: 70,
            order_side: OrderSide::Bid,
        };
        let order8 = BookOrder {
            order_id: 8,
            price: 106,
            quantity: 80,
            order_side: OrderSide::Bid,
        };
        let order9 = BookOrder {
            order_id: 9,
            price: 107,
            quantity: 90,
            order_side: OrderSide::Bid,
        };
        let order10 = BookOrder {
            order_id: 10,
            price: 108,
            quantity: 100,
            order_side: OrderSide::Bid,
        };
        half_book.add_order(order1).unwrap();
        half_book.add_order(order2).unwrap();
        half_book.add_order(order3).unwrap();
        half_book.add_order(order4).unwrap();
        half_book.add_order(order5).unwrap();
    }
}

