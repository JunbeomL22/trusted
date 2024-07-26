use crate::data::order::{
    LimitOrder,
    MarketOrder,
    OrderRequest,
};
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
use std::collections::{
    btree_map::{BTreeMap, Entry as BtmEntry},
    hash_map::Entry as HmEntry,
};
    
#[derive(Debug, Clone, Default)]
pub struct HalfBook {
    pub order_side: OrderSide,
    // wow! this can be gigantic
    // where cancel or trade, it must be removed
    pub levels: BTreeMap<BookPrice, Level>, 
    pub cache: FxHashMap<OrderId, BookPrice>, 
    pub best_price: BookPrice,
    total_quantity: BookQuantity,
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
            levels: BTreeMap::default(),
            cache: FxHashMap::default(),
            best_price,
            total_quantity: 0,
        }
    }

    #[must_use]
    pub fn initialize_with_limit_order(order: LimitOrder) -> Self {
        let mut levels = BTreeMap::default();
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
            total_quantity: order.quantity,
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
            OrderSide::Ask => {
                self.best_price = self.best_price.min(price);
            }
            _ => {
                self.best_price = self.best_price.max(price);
            }
        }
    }

    #[inline]
    pub fn reinitialize_best_price(&mut self) {
        if self.levels.is_empty() {
            if self.order_side == OrderSide::Ask {
                self.best_price = BookPrice::MAX;
            } else {
                self.best_price = BookPrice::MIN;
            }
        } else {
            if self.order_side == OrderSide::Ask {
                self.best_price = *self.levels.first_key_value().unwrap().0;
            } else {
                self.best_price = *self.levels.last_key_value().unwrap().0;
            }
        }
    }

    #[inline]
    pub fn add_limit_order(&mut self, order: LimitOrder) -> Result<()> {
        self.update_best_price(order.price);

        self.cache.insert(order.order_id, order.price);

        match self.levels.get_mut(&order.price) {
            Some(level) => {
                self.total_quantity += order.quantity;
                level.add_limit_order(order)?;
            },
            None => {
                self.total_quantity += order.quantity;
                let price = order.price;
                let level = Level::initialize_with_order(order);
                self.levels.insert(price, level);
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
        match self.cache.entry(order_id) {
            HmEntry::Occupied(entry) => {
                let price = *entry.get();
                entry.remove();
                if let Some(level) = self.levels.get_mut(&price) {
                    if let Some((removed, remained)) = level.cancel_order(order_id) {
                        self.total_quantity -= removed;
                        if remained == 0 {
                            self.levels.remove(&price);
                            
                            if price == self.best_price {
                                if self.levels.is_empty() {
                                    if self.order_side == OrderSide::Ask {
                                        self.best_price = BookPrice::MAX;
                                    } else {
                                        self.best_price = BookPrice::MIN;
                                    }
                                } else {
                                    if self.order_side == OrderSide::Ask {
                                        self.best_price = *self.levels.first_key_value().unwrap().0;
                                    } else {
                                        self.best_price = *self.levels.last_key_value().unwrap().0;
                                    }
                                }
                            }
                        }
                        return Some(())
                    } else {
                        return None
                    }
                } else {
                    return None
                }
            }
            HmEntry::Vacant(_) => {
                return None
            }
        }
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
        match self.cache.entry(order_id) {
            HmEntry::Occupied(mut entry) => { // cache has the order
                let price = *entry.get();
                *entry.get_mut() = new_price;
                match self.levels.entry(price) {
                    BtmEntry::Occupied(mut entry) => { // level for the price exists
                        let level = entry.get_mut();
                        if let Some((removed, remained)) = level.cancel_order(order_id) {
                            self.total_quantity -= removed;
                            if remained == 0 { // there is no order left in the level
                                entry.remove();
                            } 
                            self.add_limit_order(LimitOrder {
                                order_id,
                                price: new_price,
                                quantity: removed,
                                order_side: self.order_side,
                            }).unwrap();
                            //
                            self.reinitialize_best_price();
                            return Some(())
                        } else {
                            
                            return None
                        }
                    },
                    BtmEntry::Vacant(_) => { // level for the price does not exist
                        
                        return None
                    },
                }
            },
            HmEntry::Vacant(_) => { // cache does not have the order
                
                return None
            }
        }
    }

    /// check if the order is in the cache before
    pub fn change_quantity(
        &mut self, 
        order_id: OrderId, 
        new_quantity: BookQuantity,
    ) -> Option<()> {
        if let Some(price) = self.cache.get(&order_id) {
            if let Some(level) = self.levels.get_mut(price) {
                for (idx, (oid, q)) in level.orders.iter().enumerate() {
                    if *oid == order_id {
                        if q > &new_quantity {
                            let diff = q - new_quantity;
                            level.total_quantity -= diff;
                            self.total_quantity -= diff;
                        } else {
                            let diff = new_quantity - q;
                            level.total_quantity += diff;
                            self.total_quantity += diff;
                        }

                        level.orders[idx].1 = new_quantity;

                        if level.total_quantity == 0 {
                            self.levels.remove(price);
                            if *price == self.best_price {
                                self.reinitialize_best_price();
                            }
                        } 
                        return Some(())
                    }
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
        self.total_quantity
    }

    /// trade from the best_price and returns (last_trade_price, remaining quantity)
    #[must_use]
    #[inline]
    pub fn trade_market_order(&mut self, qty: BookQuantity) -> Option<(BookPrice, BookQuantity)> {
        if self.is_empty() {
            return None
        }
        let mut remaining = qty;
        let mut best_price = self.best_price; 
        while remaining > 0 {
            if self.is_empty() {
                return Some((best_price, remaining))
            }

            best_price = self.best_price;
            let level = self.levels.get_mut(&best_price).unwrap();
            let (remove_ids, rem, level_total) = level.trade(remaining);
            self.total_quantity -= remaining - rem;

            if level_total == 0 {
                self.levels.remove(&best_price);
                self.reinitialize_best_price();
            }
            remaining = rem;
            for id in remove_ids {
                self.cache.remove(&id);
            }
        }
        Some((best_price, remaining))
    }

    #[inline]
    pub fn get_best_level_mut(&mut self) -> Option<&mut Level> {
        self.levels.get_mut(&self.best_price)
    }

    /// trade from the best_price and returns remaining quantit
    /// If the order is worse than the best price, it will return None directly.
    /// Otherwise, it will trade from the best price and return the remaining quantity.
    /// If the order is the same side with the half_book, it will return None.
    #[must_use]
    #[inline]
    pub fn trade_limit_order(&mut self, order: &LimitOrder) -> Option<BookQuantity> {
        // nothinf to trade in the following cases
        if (self.order_side == order.order_side) || // same side
        self.is_empty() || // empty book
        (self.order_side == OrderSide::Ask && order.price < self.best_price) || // ask book and price is lower than the best price
        (self.order_side == OrderSide::Bid && order.price > self.best_price) { // bid book and price is higher than the best price
            return None
        }

        let mut remaining = order.quantity;
        while remaining > 0 {
            if self.order_side == OrderSide::Ask {
                let mut remove_price: Vec<BookPrice> = vec![];
                
                for (price, level) in self.levels.iter_mut() {
                    if *price > order.price {
                        return Some(remaining)
                    } else {
                        let (remove_ids, rem, level_rem) = level.trade(remaining);
                        self.total_quantity -= remaining - rem;
                        if level_rem == 0 { remove_price.push(*price) }
                        remaining = rem;
                        for id in remove_ids {
                            self.cache.remove(&id);
                        }
                    }
                }

                for price in remove_price {
                    self.levels.remove(&price);
                }
                self.reinitialize_best_price();

            } else if self.order_side == OrderSide::Bid {
                let mut remove_price: Vec<BookPrice> = vec![];

                for (price, level) in self.levels.iter_mut().rev() {
                    if *price < order.price {
                        return Some(remaining)
                    } else {
                        let (remove_ids, rem, level_rem) = level.trade(remaining);
                        self.total_quantity -= remaining - rem;
                        if level_rem == 0 { remove_price.push(*price) }
                        remaining = rem;

                        for id in remove_ids {
                            self.cache.remove(&id);
                        }
                    }
                }

                for price in remove_price {
                    self.levels.remove(&price);
                }
                self.reinitialize_best_price();
            }
        }

        Some(remaining)
            
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::OrderSide;
    use crate::data::order::{
        LimitOrder,
        MarketOrder,
    };

    #[test]
    fn test_ask_half_book() -> Result<()> {
        let mut half_book = HalfBook::initialize(OrderSide::Ask);
        assert!(half_book.is_empty());
        let order = LimitOrder {
            order_id: 1,
            price: 100,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(order)?;

        assert_eq!(half_book.len(), 1);

        let order = LimitOrder {
            order_id: 2,
            price: 100,
            quantity: 5,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(order)?;
        assert_eq!(half_book.len(), 1);
        assert_eq!(half_book.aggregate_quantity(), 15);

        let order = LimitOrder {
            order_id: 3,
            price: 101,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(order)?;

        assert_eq!(half_book.len(), 2);
        assert_eq!(half_book.aggregate_quantity(), 25);
        
        // dbg!(half_book.clone()); total_quantity = 25

        half_book.change_price(1, 99);
        assert_eq!(half_book.best_price, 99);

        // dbg!(half_book.clone()); wrong total quantity = 35 not 25

        half_book.change_quantity(2, 10);
        assert_eq!(half_book.aggregate_quantity_at_price(99), 10);
        assert_eq!(half_book.aggregate_quantity(), 30);
        assert_eq!(half_book.best_price, 99);

        half_book.cancel_order(1);
        assert_eq!(half_book.aggregate_quantity(), 20);
        assert_eq!(half_book.best_price, 100);
    
        assert_eq!(half_book.cache.len(), 2);
        assert_eq!(half_book.levels.len(), 2);

        half_book.cancel_order(2);
        assert_eq!(half_book.aggregate_quantity(), 10);
        assert_eq!(half_book.best_price, 101);

        // dbg!(half_book.clone()); ok!
        half_book.change_price(3, 99);
        assert_eq!(half_book.best_price, 99);
        let stress_test = half_book.change_price(4, 99);
        assert_eq!(stress_test, None);

        half_book.change_quantity(3, 5);
        
        assert_eq!(half_book.aggregate_quantity(), 5);
        let x = half_book.change_quantity(4, 0);

        assert_eq!(x, None);
        Ok(())
    }

    #[test]
    fn test_bid_half_book() -> Result<()> {
        let mut half_book = HalfBook::initialize(OrderSide::Bid);
        assert!(half_book.is_empty());
        let order = LimitOrder {
            order_id: 1,
            price: 100,
            quantity: 10,
            order_side: OrderSide::Bid,
        };

        half_book.add_limit_order(order)?;

        assert_eq!(half_book.len(), 1);

        let order = LimitOrder {
            order_id: 2,
            price: 100,
            quantity: 5,
            order_side: OrderSide::Bid,
        };

        half_book.add_limit_order(order)?;
        assert_eq!(half_book.len(), 1);
        assert_eq!(half_book.aggregate_quantity(), 15);
        assert_eq!(half_book.best_price, 100);

        let order = LimitOrder {
            order_id: 3,
            price: 101,
            quantity: 10,
            order_side: OrderSide::Bid,
        };

        half_book.add_limit_order(order)?;

        assert_eq!(half_book.len(), 2);
        assert_eq!(half_book.aggregate_quantity(), 25);
        assert_eq!(half_book.best_price, 101);
        
        // dbg!(half_book.clone()); total_quantity = 25

        half_book.change_price(1, 99);
        assert_eq!(half_book.best_price, 101);

        // dbg!(half_book.clone()); wrong total quantity = 35 not 25

        half_book.change_quantity(2, 10);
        assert_eq!(half_book.aggregate_quantity(), 30);
        assert_eq!(half_book.best_price, 101);

        half_book.cancel_order(3);
        assert_eq!(half_book.aggregate_quantity(), 20);
        assert_eq!(half_book.best_price, 100);
        
        Ok(())
    }

    #[test]
    fn test_limit_trade() -> Result<()> {
        let mut half_book = HalfBook::initialize(OrderSide::Ask);
        assert!(half_book.is_empty());
        let order = LimitOrder {
            order_id: 1,
            price: 100,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(order)?;

        assert_eq!(half_book.len(), 1);

        let order = LimitOrder {
            order_id: 2,
            price: 100,
            quantity: 5,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(order)?;
        assert_eq!(half_book.len(), 1);
        assert_eq!(half_book.aggregate_quantity(), 15);

        let order = LimitOrder { 
            order_id: 3,
            price: 101,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(order)?;
        

        let limit_order = LimitOrder {
            order_id: 4,
            price: 101,
            quantity: 20,
            order_side: OrderSide::Bid,
        };

        dbg!(half_book.clone());
        let x = half_book.trade_limit_order(&limit_order);
        
        println!("{:?}", x);
        dbg!(half_book.clone());

        Ok(())
    }
}