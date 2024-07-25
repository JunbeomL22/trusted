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
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct HalfBook {
    pub order_side: OrderSide,
    // wow! this can be gigantic
    // where cancel or trade, it must be removed
    pub levels: FxHashMap<BookPrice, Level>, 
    pub cache: FxHashMap<OrderId, BookPrice>, 
    pub best_price: BookPrice,
}

impl HalfBook {
    #[must_use]
    pub fn initialize(order_side: OrderSide) -> Self {
        let best_price = if order_side == OrderSide::Ask {
            BookPrice::MAX
        } else {
            BookPrice::MIN
        };
        HalfBook {
            order_side,
            levels: FxHashMap::default(),
            cache: FxHashMap::default(),
            best_price,
        }
    }

    #[must_use]
    pub fn initialize_with_order(order: BookOrder) -> Self {
        let mut levels = FxHashMap::default();
        let price = order.price;
        let level = Level::initialize_with_order(order.clone());
        levels.insert(price, level);
        let mut cache = FxHashMap::default();
        cache.insert(order.order_id, price);
        HalfBook {
            order_side: order.order_side,
            levels,
            cache,
            best_price: price,
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
    pub fn update_best_price(&mut self, price: BookPrice) {
        match self.order_side {
            OrderSide::NoSide => {}
            OrderSide::Ask => {
                self.best_price = self.best_price.min(price);
            }
            _ => {
                self.best_price = self.best_price.max(price);
            }
        }
    }

    #[inline]
    pub fn add_order(&mut self, order: BookOrder) -> Result<()> {
        self.update_best_price(order.price);

        self.cache.insert(order.order_id, order.price);

        match self.levels.get_mut(&order.price) {
            Some(level) => {
                level.add_order(order)?;
            },
            None => {
                let level = Level::initialize_with_order(order.clone());
                self.levels.insert(order.price, level);
            },
        }

        Ok(())
    }

    /// preprocess like exceptions must be handled before calling this function
    /// For exmaple, cache must be checked before calling this function
    /// these actions are not Result type because the program should continue even if the order is not found
    #[inline]
    pub fn cancel_order(
        &mut self, 
        order_id: OrderId,
    ) -> Option<()> {
        let price = if let Some(price) = self.cache.get(&order_id) {
            *price
        } else {
            let msg = format!("Order not found in CancelOrder id = {}", order_id);
            crate::log_warn!(LogTopic::OrderNotFound, message = msg);
            return None
        };

        if let Some(level) = self.levels.get_mut(&price) {
            if let Some(_) = level.cancel_order(order_id) {
                self.cache.remove(&order_id);
                if level.is_empty() {
                    self.levels.remove(&price);
                    if self.best_price == price {
                        if self.order_side == OrderSide::Ask {
                            self.best_price = *self.levels.keys().min().unwrap_or(&BookPrice::MAX);
                        } else {
                            self.best_price = *self.levels.keys().max().unwrap_or(&BookPrice::MIN);
                        }
                    }
                }
                return Some(())
            }
        }

        let msg = format!("Order not found in CancelOrder id = {}", order_id);
        crate::log_warn!(LogTopic::OrderNotFound, message = msg);
        None
    }
    #[inline]
    pub fn get_price(&self, order_id: OrderId) -> Option<BookPrice> {
        self.cache.get(&order_id).copied()
    }

    /// preprocess like exceptions must be handled befor calling this function
    #[inline]
    pub fn change_price(
        &mut self, 
        order_id: OrderId, 
        new_price: BookPrice,
    ) -> Option<()> {    
        let prev_price = if let Some(price) = self.cache.get(&order_id) {
            *price
        } else {
            crate::log_warn!(
                LogTopic::OrderNotFound, 
                message = format!(
                    "OrderId is not in HalfBook.cache.\n\
                    Thus, the order will be ignored.
                    id = {}, price = {}", 
                    order_id, 
                    new_price,
                )
            );
            return None
        };

        if let Some(level) = self.levels.get_mut(&prev_price) {
            if let Some(order) = level.cancel_order(order_id) {
                self.cache.insert(order_id, new_price);

                if level.is_empty() {
                    self.levels.remove(&prev_price);
                }

                self.levels.entry(new_price)
                    .or_insert_with(Level::default)
                    .orders.push_back(order);

                if self.order_side == OrderSide::Ask {
                    self.best_price = *self.levels.keys().min().unwrap_or(&BookPrice::MAX);
                } else {
                    self.best_price = *self.levels.keys().max().unwrap_or(&BookPrice::MIN);
                }


                return Some(());
            }
        }

        crate::log_warn!(
            LogTopic::OrderNotFound, 
            message = format!(
                "OrderId is in HalfBook.cache but not in the levels.\n\
                Thus, the order will be ignored.
                id = {}, price = {}", 
                order_id, 
                new_price,
            )
        );

        None
    }

    /// check if the order is in the cache before
    pub fn change_quantity(
        &mut self, 
        order_id: OrderId, 
        new_quantity: BookQuantity,
    ) -> Option<()> {
        let price = if let Some(price) = self.cache.get(&order_id) {
            *price
        } else {
            let msg = format!(
                "Order not found in ChangeQuantity id = {}, quantity = {}", 
                order_id, 
                new_quantity
            );

            crate::log_warn!(LogTopic::OrderNotFound, message = msg);
            return None
        };
        if let Some(level) = self.levels.get_mut(&price) {
            // find and change the quantity only. do not cancel
            for (k, v) in level.orders.iter_mut() {
                if *k == order_id {
                    *v = new_quantity;
                    return Some(())
                }
            }
        }

        let msg = format!(
            "Order not found in ChangeQuantity id = {}, quantity = {}", 
            order_id, 
            new_quantity
        );

        crate::log_warn!(LogTopic::OrderNotFound, message = msg);
        None
    }

    #[must_use]
    #[inline]
    pub fn aggregate_quantity_at_price(&self, price: BookPrice) -> BookQuantity {
        self.levels.get(&price).map_or(0, |level| level.aggregate_quantity())
    }

    #[must_use]
    #[inline]
    pub fn aggregate_quantity(&self) -> BookQuantity {
        self.levels.iter().map(|(_, level)| level.aggregate_quantity()).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::OrderSide;
    use crate::data::book_order::BookOrder;

    #[test]
    fn test_half_book() -> Result<()> {
        let mut half_book = HalfBook::initialize(OrderSide::Ask);
        assert!(half_book.is_empty());
        let order = BookOrder {
            order_id: 1,
            price: 100,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_order(order)?;

        assert_eq!(half_book.len(), 1);

        let order = BookOrder {
            order_id: 2,
            price: 100,
            quantity: 5,
            order_side: OrderSide::Ask,
        };

        half_book.add_order(order)?;
        assert_eq!(half_book.len(), 1);
        assert_eq!(half_book.aggregate_quantity(), 15);

        let order = BookOrder {
            order_id: 3,
            price: 101,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_order(order)?;

        assert_eq!(half_book.len(), 2);
        assert_eq!(half_book.aggregate_quantity(), 25);
        

        half_book.change_price(1, 99);
        assert_eq!(half_book.best_price, 99);

        half_book.change_quantity(2, 10);
        assert_eq!(half_book.aggregate_quantity_at_price(99), 10);
        assert_eq!(half_book.aggregate_quantity(), 30);
        assert_eq!(half_book.best_price, 99);

        half_book.cancel_order(1);
        assert_eq!(half_book.aggregate_quantity(), 20);
        assert_eq!(half_book.best_price, 100);
    
        dbg!(half_book.clone());



        Ok(())
    }
}