use crate::data::order::{
    LimitOrder,
    MarketOrder,
};
use crate::data::{
    quote::QuoteSnapshot,
    trade_quote::TradeQuoteSnapshot,
};
use crate::orderbook::level::Level;
use crate::types::{
    base::{BookPrice, OrderId, BookQuantity, TradeHistory, LevelSnapshot},
    enums::OrderSide,
};
use crate::topics::LogTopic;
//
use anyhow::{Result, anyhow};
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
    /// it cleans levels and cache then
    /// LevelSnapshot is used as a single quote with fake orderid
    pub fn update_by_level_snapshot(&mut self, level_snapshot: &Vec<LevelSnapshot>) {
        self.levels.clear();
        self.cache.clear();
        self.total_quantity = 0;
        self.best_price = if self.order_side == OrderSide::Ask {
            BookPrice::MAX
        } else {
            BookPrice::MIN
        };
        
        let mut order_id_generator = 0;
        
        for level in level_snapshot {
            let price = level.book_price;
            let quantity = level.book_quantity;
            
            let mut level = Level::initialize(price);
            level.total_quantity = quantity;
            let order_id = order_id_generator;
            order_id_generator += 1;

            level.orders.push_back((order_id, quantity));

            self.cache.insert(order_id, price);
            self.levels.insert(price, level);
            self.total_quantity += quantity;
            self.best_price = match self.order_side {
                OrderSide::Ask => self.best_price.min(price),
                OrderSide::Bid => self.best_price.max(price),
            };
        }
    }

    pub fn to_string_upto_depth(&self, depth: Option<usize>) -> String {
        match self.order_side {
            OrderSide::Ask => self.ask_half_book_string(depth),
            OrderSide::Bid => self.bid_half_book_string(depth),
        }
    }
    
    pub fn ask_half_book_string(&self, depth: Option<usize>) -> String {
        let depth = depth.unwrap_or(self.len());
        let half_book = self;
        let mut output = String::new();
        let header = format!(
            "{:<15} | {:<8} | {:<12}\n", "Price", "Ask Cnt", "Ask Qty");
        output.push_str(&header);
        output.push_str(&"-".repeat(header.len()));
        output.push('\n');
    
        for (price, level) in half_book.levels.iter().rev().take(depth) {
            output.push_str(&format!(
                "{:<15} | {:<8} | {:<12}\n",
                price, level.orders.len(), level.total_quantity
            ));
        }
    
        output
    }
    
    pub fn get_order(&self, order_id: OrderId) -> Option<LimitOrder> {
        self.cache.get(&order_id).map(|price| {
            let level = self.levels.get(price).unwrap();
            level.get_order(order_id, self.order_side).unwrap()
        })
    }

    pub fn bid_half_book_string(&self, depth: Option<usize>) -> String {
        let depth = depth.unwrap_or(self.len());
        let half_book = self;
        let mut output = String::new();
        let header = format!("{:>12} | {:>8} | {:>15}\n", "Bid Qty", "Bid Cnt", "Price");
        output.push_str(&header);
        output.push_str(&"-".repeat(header.len()));
        output.push('\n');
    
        for (price, level) in half_book.levels.iter().take(depth) {
            output.push_str(&format!(
                "{:>12} | {:>8} | {:>15}\n",
                level.total_quantity, level.orders.len(), price
            ));
        }
    
        output
    }

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
                self.best_price = *self.levels.iter().next().unwrap().0;
            } else {
                self.best_price = *self.levels.iter().next_back().unwrap().0;
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
                if let Some(original_qty) = level.change_quantity(order_id, new_quantity) {
                    self.total_quantity -= original_qty;
                    self.total_quantity += new_quantity;
                    if level.total_quantity == 0 {
                        self.levels.remove(price);
                        if *price == self.best_price {
                            self.reinitialize_best_price();
                        }
                    }
                    return Some(())
                }
                return None
            }
            None
        } else {
            let msg = format!(
                "Order not found in ChangeQuantity id = {}, quantity = {}", 
                order_id, 
                new_quantity
            );

            crate::log_warn!(LogTopic::OrderNotFound, message = msg);
            None
        }
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

    /// trade from the best_price and returns the trade history and remaining quantity
    /// which is a vector of (traded_price, traded_quantity). The last element of the vector is the last trade.
    /// None means no trade, the case happens when the order is the same side with the half_book
    /// or the half_book is empty
    #[must_use]
    #[inline]
    pub fn trade_market_order(
        &mut self, 
        market_order: &MarketOrder,
    ) -> Option<(TradeHistory, BookQuantity)> {
        if self.is_empty() || market_order.order_side == self.order_side {
            return None
        }

        let qty = market_order.quantity;

        let mut remaining = qty;
        let mut remove_prices: Vec<BookPrice> = vec![];
        let mut remove_ids: Vec<OrderId> = vec![];
        let trade_history = match self.order_side {
            OrderSide::Ask => self.ask_trade_market_order(&mut remaining, &mut remove_prices, &mut remove_ids),
            OrderSide::Bid => self.bid_trade_market_order(&mut remaining, &mut remove_prices, &mut remove_ids),
        };
        self.cleanup_after_trade(&remove_prices, &remove_ids);
        Some((trade_history, remaining))
    }

    #[inline]
    pub fn get_best_level_mut(&mut self) -> Option<&mut Level> {
        self.levels.get_mut(&self.best_price)
    }

    /// Returns a Vector of traded_price and traded_quantity
    fn ask_trade_market_order(
        &mut self,
        remaining: &mut BookQuantity,
        remove_prices: &mut Vec<BookPrice>, 
        remove_ids: &mut Vec<OrderId>, 
    ) -> TradeHistory {
        let mut traded_res = TradeHistory::default();
        for (price, level) in self.levels.iter_mut() {
            let (rm_ids, rem, level_rem) = level.trade(*remaining);
            remove_ids.extend(rm_ids);
            let traded_qty = (*remaining).saturating_sub(rem);
            if traded_qty > 0 {
                traded_res.push(*price, traded_qty);
                self.total_quantity -= traded_qty;
            }
            if level_rem == 0 { remove_prices.push(*price) }
            *remaining = rem;
            
            if *remaining == 0 {
                return traded_res
            }
        }
        traded_res
    }

    /// Returns a Vector of traded_price and traded_quantity
    fn bid_trade_market_order(
        &mut self,
        remaining: &mut BookQuantity,
        remove_prices: &mut Vec<BookPrice>, 
        remove_ids: &mut Vec<OrderId>, 
    ) -> TradeHistory {
        let mut traded_res = TradeHistory::default();
        for (price, level) in self.levels.iter_mut().rev() {
            let (rm_ids, rem, level_rem) = level.trade(*remaining);
            remove_ids.extend(rm_ids);
            let traded_qty = (*remaining).saturating_sub(rem);
            if traded_qty > 0 {
                traded_res.push(*price, traded_qty);
                self.total_quantity -= traded_qty;
            }

            if level_rem == 0 { remove_prices.push(*price) }
            *remaining = rem;

            if *remaining == 0 {
                return traded_res
            }
        }
        traded_res
    }

    /// Returns a Vector of traded_price and traded_quantity
    fn ask_trade_limit_order(
        &mut self, 
        order_price: BookPrice,
        remaining: &mut BookQuantity,
        remove_prices: &mut Vec<BookPrice>, 
        remove_ids: &mut Vec<OrderId>, 
    ) -> TradeHistory {
        let mut traded_res = TradeHistory::default();
        for (price, level) in self.levels.iter_mut() {
            if *price > order_price || *remaining == 0 {
                return traded_res
            } else {
                let (rm_ids, rem, level_rem) = level.trade(*remaining);
                remove_ids.extend(rm_ids);
                let traded_qty = (*remaining).saturating_sub(rem);
                if traded_qty > 0 {
                    traded_res.push(*price, traded_qty);
                    self.total_quantity -= traded_qty;
                }
                
                if level_rem == 0 { remove_prices.push(*price) }       
                *remaining = rem;
            }
        }   
        traded_res
    }

    /// Returns a Vector of traded_price and traded_quantity
    fn bid_trade_limit_order(
        &mut self, 
        order_price: BookPrice,
        remaining: &mut BookQuantity,
        remove_prices: &mut Vec<BookPrice>, 
        remove_ids: &mut Vec<OrderId>, 
    ) -> TradeHistory {
        let mut traded_res = TradeHistory::default();
        for (price, level) in self.levels.iter_mut().rev() {
            if *price < order_price || *remaining == 0 {
                return traded_res
            } else {
                let (rm_ids, rem, level_rem) = level.trade(*remaining);
                remove_ids.extend(rm_ids);
                let traded_qty = (*remaining).saturating_sub(rem);
                if traded_qty > 0 {
                    traded_res.push(*price, traded_qty);
                    self.total_quantity -= traded_qty;
                }
                
                if level_rem == 0 { remove_prices.push(*price) }
                *remaining = rem;
            }
        }
        traded_res
    }

    fn cleanup_after_trade(&mut self, remove_price: &[BookPrice], remove_id: &[OrderId]) {
        for id in remove_id {
            self.cache.remove(id);
        }
        for price in remove_price {
            self.levels.remove(price);
        }
        self.reinitialize_best_price();
    }

    /// trade from the best_price and returns remaining quantit
    /// If the order is worse than the best price, it will return None directly.
    /// Otherwise, it will trade from the best price and return the traded history which is 
    /// a vector of (traded_price, traded_quantity). The last element of the vector is the last trade.
    /// If the order is the same side with the half_book, it will return None.
    #[must_use]
    #[inline]
    pub fn trade_limit_order(&mut self, order: &LimitOrder) -> Option<(TradeHistory, BookQuantity)> {
        // nothinf to trade in the following cases
        if (self.order_side == order.order_side) || // same side
        self.is_empty() || // empty book
        (self.order_side == OrderSide::Ask && order.price < self.best_price) || // ask book and price is lower than the best price
        (self.order_side == OrderSide::Bid && order.price > self.best_price) {  // bid book and price is higher than the best price
            return None
        }

        let mut remaining = order.quantity;
        let order_price = order.price;
        let mut remove_prices: Vec<BookPrice> = vec![];
        let mut remove_ids: Vec<OrderId> = vec![];
        
        let trade_history = match self.order_side {
            OrderSide::Ask => self.ask_trade_limit_order(order_price, &mut remaining, &mut remove_prices, &mut remove_ids),
            OrderSide::Bid => self.bid_trade_limit_order(order_price, &mut remaining, &mut remove_prices, &mut remove_ids),
        };

        self.cleanup_after_trade(&remove_prices, &remove_ids);
        Some((trade_history, remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::OrderSide;
    use crate::data::order::LimitOrder;

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
        //
        assert_eq!(half_book.cache.len(), 2);
        assert_eq!(half_book.levels.len(), 2);

        half_book.cancel_order(2);
        //
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
            quantity: 3,
            order_side: OrderSide::Bid,
        };
        assert_eq!(half_book.best_price, 100);

        let (trades, remaining) = half_book.trade_limit_order(&limit_order).unwrap();
        assert_eq!(trades.get_last_trade(), Some((100, 3)));
        assert_eq!(remaining, 0);

        assert_eq!(half_book.aggregate_quantity(), 22);
        assert_eq!(half_book.best_price, 100);
        
        let market_order = MarketOrder {
            order_id: 5,
            quantity: 1,
            order_side: OrderSide::Bid,
        };

        let (trades, remaining) = half_book.trade_market_order(&market_order).unwrap();
        assert_eq!(trades.get_last_trade(), Some((100, 1)));
        assert_eq!(
            (trades.average_trade_price() - 100.0).abs() < 0.000001, 
            true
        );

        assert_eq!(remaining, 0);
        assert_eq!(half_book.aggregate_quantity(), 21);

        let market_order = MarketOrder {
            order_id: 6,
            quantity: 12,
            order_side: OrderSide::Bid,
        };

        let (trades, remaining) = half_book.trade_market_order(&market_order).unwrap();
        assert_eq!(trades.get_last_trade(), Some((101, 1)));
        let ave = (101.0 * 1.0 + 100.0 * 11.0) / 12.0;
        assert!((trades.average_trade_price() - ave).abs() < 0.000001);
        assert_eq!(remaining, 0);    

        let display = half_book.to_string_upto_depth(Some(3));
        println!("{}", display);

        Ok(())
    }
}