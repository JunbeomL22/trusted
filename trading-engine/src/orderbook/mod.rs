pub mod half_book;
pub mod level;
//
//
use crate::types::enums::OrderSide;
use crate::orderbook::half_book::HalfBook;
use crate::data::order::{
    LimitOrder,
    MarketOrder,
    CancelOrder,
    ModifyOrder,
    RemoveAnyOrder,
    DecomposedOrder,
};
use crate::data::{
    quote::QuoteSnapshot,
    trade_quote::TradeQuoteSnapshot,
};
use crate::topics::LogTopic;
use crate::types::{
    id::InstId,
    base::{
        OrderId,
        BookPrice,
        BookQuantity,
        TradeHistory,
        VirtualOrderId,
        LevelSnapshot,
    },
    timestamp::TimeStamp,
};
//
use anyhow::{
    Result, 
    anyhow, 
    Context,
};

#[derive(Debug, Clone, Default)]
pub struct OrderBook {
    pub asks: HalfBook,
    pub bids: HalfBook,
    id: InstId,
    virtual_id_counter: VirtualOrderId,
    timestamp: TimeStamp,
    system_time: TimeStamp,
}

impl OrderBook {
    #[inline]
    #[must_use]
    pub fn bid_level_snapshot(&self) -> Vec<LevelSnapshot> {
        self.bids.to_level_snapshot()
    }

    #[inline]
    #[must_use]
    pub fn ask_level_snapshot(&self) -> Vec<LevelSnapshot> {
        self.asks.to_level_snapshot()
    }
    #[must_use]
    pub fn eq_level(&self, other: &Self) -> bool {
        self.asks.eq_level(&other.asks) && self.bids.eq_level(&other.bids)
    }

    #[must_use]
    pub fn decomposed_orders_with_update(
        &mut self,
        ask_level_snapshot: &Vec<LevelSnapshot>,
        bid_level_snapshot: &Vec<LevelSnapshot>,
    ) -> Result<Vec<DecomposedOrder>> {
        let mut decomposed_orders = Vec::<DecomposedOrder>::default();
        let ask_order_vec = self.asks.decomposed_orders_with_update(ask_level_snapshot, &mut self.virtual_id_counter)?;
        let bid_order_vec = self.bids.decomposed_orders_with_update(bid_level_snapshot, &mut self.virtual_id_counter)?;

        for order in ask_order_vec.into_iter() {
            let decompsed_order = DecomposedOrder {
                order,
                timestamp: self.timestamp,
                id: self.id,
            };
            decomposed_orders.push(decompsed_order);
        }

        for order in bid_order_vec.into_iter() {
            let decompsed_order = DecomposedOrder {
                order,
                timestamp: self.timestamp,
                id: self.id,
            };
            decomposed_orders.push(decompsed_order);
        }

        Ok(decomposed_orders)
    }
    #[inline]
    pub fn check_validity_quantity(&self) -> bool {
        self.asks.check_validity_quantity() && self.bids.check_validity_quantity()
    }

    fn check_id(&self, id: InstId) -> Result<()> {
        if self.id == id {
            Ok(())
        } else {
            let lazy_msg = || {
                anyhow!(
                    "Id mismatch\n\
                    orderbook id: {:?}\n\
                    input id: {:?}",
                    self.id,
                    id,
                )
            };
            Err(anyhow!(lazy_msg()))
        }
    }

    /// update by snapshot
    #[inline]
    pub fn update_from_quote_snapshot(&mut self, quote: &QuoteSnapshot) -> Result<()> {
        self.check_id(quote.id)?;
        self.asks.update_l2_snapshot(&quote.ask_quote_data, &mut self.virtual_id_counter)
            .context("Failed to update ask side by level snapshot")?;
        self.bids.update_l2_snapshot(&quote.bid_quote_data, &mut self.virtual_id_counter)
            .context("Failed to update bid side by level snapshot")?;

        Ok(())
    }

    #[inline]
    pub fn update_from_trade_quote_snapshot(&mut self, trade_quote: &TradeQuoteSnapshot) -> Result<()> {
        self.check_id(trade_quote.id)?;
        
        self.asks.update_l2_snapshot(&trade_quote.ask_quote_data, &mut self.virtual_id_counter)
            .context("Failed to update ask side by level snapshot")?;
        self.bids.update_l2_snapshot(&trade_quote.bid_quote_data, &mut self.virtual_id_counter)
            .context("Failed to update bid side by level snapshot")?;

        Ok(())
    }

    pub fn to_string(&self) -> String {
        
        let mut output = String::new();
        let header = format!(
            "{:<12} | {:<8} | {:<15} | {:<8} | {:<12}\n", 
            "Bid Qty", "Bid Cnt", "Price", "Ask Cnt", "Ask Qty"
        );

        output.push_str(&header);
        output.push_str(&"-".repeat(header.len()-1)); // cut by "-". "\n" not counted. 
        output.push('\n');
    
        
        for (price, level) in self.asks.levels.iter().rev() {
            output.push_str(&format!(
                "{:<12} | {:<8} | {:<15} | {:<8} | {:<12}\n",
                " ", " ", price, level.orders.len(), level.total_quantity
            ));
        }
    
        // cut by "-"
        output.push_str( &format!(
            "{:<12} | {:<8} | {:<15} | {:<8} | {:<12}\n", 
            "-".repeat(12), "-".repeat(8), "-".repeat(15), "-".repeat(8), "-".repeat(12)
        ));

        for (price, level) in self.bids.levels.iter().rev() {
            output.push_str(&format!(
                "{:<12} | {:<8} | {:<15} | {:<8} | {:<12}\n",
                level.total_quantity, level.orders.len(), price, " ", " "
            ));
        }
    
        output
    }

    #[inline]
    pub fn initialize_with_id(id: InstId) -> Self {
        OrderBook {
            asks: HalfBook::initialize(OrderSide::Ask),
            bids: HalfBook::initialize(OrderSide::Bid),
            id,
            virtual_id_counter: VirtualOrderId::new(0),
            timestamp: TimeStamp::default(),
            system_time: TimeStamp::default(),
        }
    }
    
    #[inline]
    pub fn add_limit_order(&mut self, order: LimitOrder) -> Result<()> {
        match order.order_side {
            OrderSide::Ask => self.asks.add_limit_order(order),
            OrderSide::Bid => self.bids.add_limit_order(order),
        }
    }

    /// None means order_id not found, either in cache or levels
    #[inline]
    pub fn cancel_order(&mut self, order: CancelOrder) -> Option<()> {
        let order_id = order.order_id;
        if self.asks.cancel_order(order_id).is_some() ||
        self.bids.cancel_order(order_id).is_some() {
            Some(())
        } else {
            let message = format!(
                "Order {} not found\n\
                id: {:?}",
                order_id, self.id
            );

            crate::log_warn!(LogTopic::OrderNotFound, message = message);
            None
        }
    }

    pub fn remove_order(&mut self, order: RemoveAnyOrder) -> Option<BookQuantity> {
        match order.order_side {
            OrderSide::Ask => self.asks.remove_order(order),
            OrderSide::Bid => self.bids.remove_order(order),
        }
    }
    
    /// None means that just change order not traded.
    pub fn modify_order(&mut self, order: ModifyOrder) -> Option<(TradeHistory, BookQuantity)> {
        let order_id = order.order_id;
        if let Some(price) = self.asks.cache.get(&order_id) {
            if price == &order.price { // same price so just change quantity
                self.asks.change_quantity(order_id, order.quantity);
                None
            } else { 
                let limit_order = LimitOrder {
                    order_id,
                    price: order.price,
                    quantity: order.quantity,
                    order_side: OrderSide::Ask,
                };
                self.asks.cancel_order(order_id);
                self.process_limit_order(limit_order)
            }
        } else if let Some(price) = self.bids.cache.get(&order_id) {
            if price == &order.price { // same price so just change quantity
                self.bids.change_quantity(order_id, order.quantity);
                None
            } else { 
                let limit_order = LimitOrder {
                    order_id,
                    price: order.price,
                    quantity: order.quantity,
                    order_side: OrderSide::Bid,
                };
                self.bids.cancel_order(order_id);
                self.process_limit_order(limit_order)
            }
        } else {
            let copied_id = self.id;
            let message = crate::LazyString::new(move || 
                format!(
                    "Order {} not found\n\
                    id: {:?}",
                    order_id, copied_id
                )
            );

            crate::log_warn!(LogTopic::OrderNotFound, message = message);
            None
        }
    }

    /// None means order_id not found, either in cache or levels
    #[inline]
    #[deprecated]
    fn change_price(&mut self, order_id: OrderId, new_price: BookPrice) -> Option<()> {
        if self.asks.change_price(order_id, new_price).is_some() {
            Some(())
        } else if self.bids.change_price(order_id, new_price).is_some() {
            Some(())
        } else {
            let copied_id = self.id;
            let message = crate::LazyString::new(move || 
                format!(
                    "Order {} not found\n\
                    id: {:?}",
                    order_id, copied_id
                )
            );

            crate::log_warn!(LogTopic::OrderNotFound, message = message);
            None
        }
    }

    /// None means order_id not found, either in cache or levels
    #[inline]
    pub fn change_quantity(&mut self, order_id: OrderId, new_quantity: BookQuantity) -> Option<()> {
        if self.asks.change_quantity(order_id, new_quantity).is_some() || 
        self.bids.change_quantity(order_id, new_quantity).is_some() {
            Some(())
        } else {
            let message = format!(
                "Order {} not found\n\
                id: {:?}",
                order_id, 
                self.id
            );

            crate::log_warn!(LogTopic::OrderNotFound, message = message);
            None
        }
    }

    /// It trades, registers, or both trades and registers the order from the best_price. 
    /// If the order is worse than the best (higher for best ask, lower for best bid),
    /// it will be added to the book.
    #[inline]
    pub fn process_limit_order(&mut self, mut order: LimitOrder) -> Option<(TradeHistory, BookQuantity)> {
        match order.order_side {
            OrderSide::Ask => {
                if order.price > self.bids.best_price { // ask order price is higher than the best bid price, so there is no trade
                    self.asks.add_limit_order(order).expect("Failed to add limit order to ask half book");
                    None
                } else {
                    if let Some((traded_history, rem)) = self.bids.trade_limit_order(&order) {
                        if rem > 0 {
                            order.quantity = rem;
                            self.asks.add_limit_order(order).expect("Failed to add limit order to ask half book");
                        }
                        Some((traded_history, rem))
                    } else {
                        None
                    }
                }
            },
            OrderSide::Bid => {
                if order.price < self.asks.best_price { // bid order price is lower than the best ask price, so there is no trade
                    self.bids.add_limit_order(order).expect("Failed to add limit order to bid half book");
                    None
                } else {
                    if let Some((traded_history, rem)) = self.asks.trade_limit_order(&order) {
                        if rem > 0 {
                            order.quantity = rem;
                            self.bids.add_limit_order(order).expect("Failed to add limit order to bid half book");
                        }
                        Some((traded_history, rem))
                    } else {
                        None
                    }
                }
            },
        }
    }

    /// None means there is nothing traded.
    /// Otherwise, it returns the trade history which is a vector of (price, quantity) tuples,
    /// and the remaining quantity of the order.
    /// The traded price may not be the unique. If the ask market-order is larger than the best price of the bid.
    /// Then the it is traded at the best bid, then the next best bid, and so on.
    #[inline]
    pub fn process_market_order(
        &mut self, 
        order: &MarketOrder,
    ) -> Option<(TradeHistory, BookQuantity)> {
        match order.order_side {
            OrderSide::Ask => self.bids.trade_market_order(order),
            OrderSide::Bid => self.asks.trade_market_order(order),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::id::{
        isin_code::IsinCode,
        Symbol,
    };
    use crate::types::venue::Venue;
    #[test]
    fn test_orderbook() {
        let mut order_counter = VirtualOrderId::new(0);
        let ask_levels = vec![102, 101, 100];
        let qtys = vec![1, 2, 3];
        let mut ask_order_vec: Vec<LimitOrder> = Vec::new();
        
        for i in 0..ask_levels.len() {
            for j in qtys.clone().into_iter() {
                ask_order_vec.push(LimitOrder {
                    order_id: order_counter.next_id(),
                    price: ask_levels[i],
                    quantity: j as u64,
                    order_side: OrderSide::Ask,
                });
            }
        }
    
        let bid_levels = vec![99, 98, 97];
        let qtys = vec![1, 2, 3];
        let mut bid_order_vec: Vec<LimitOrder> = Vec::new();
        for i in 0..bid_levels.len() {
            for j in qtys.clone().into_iter() {
                bid_order_vec.push(LimitOrder {
                    order_id: order_counter.next_id(),
                    price: bid_levels[i],
                    quantity: j as u64,
                    order_side: OrderSide::Bid,
                });
            }
        }
    
        let isin_code = IsinCode::new(b"KRXXXXXXXXXX").unwrap();
        let venue = Venue::KRX;
        let id = InstId::new(Symbol::Isin(isin_code), venue);
        let mut order_book = OrderBook::initialize_with_id(id);
    
        for order in bid_order_vec.clone().into_iter() {
            order_book.add_limit_order(order).unwrap();
        }
    
        for order in ask_order_vec.clone().into_iter() {
            order_book.add_limit_order(order).unwrap();
        }

        assert!(order_book.check_validity_quantity());
        assert_eq!(order_book.bids.best_price, 99);
        assert_eq!(order_book.asks.get_total_quantity(), 18);
        assert_eq!(order_book.asks.best_price, 100);
        assert_eq!(order_book.bids.get_total_quantity(), 18);

        let market_order = MarketOrder {
            order_id: order_counter.next_id(),
            quantity: 2,
            order_side: OrderSide::Bid,
        };
    
        let trades = order_book.process_market_order(&market_order).unwrap();
        
        assert_eq!(trades.0[0].0, 100);
        assert_eq!(trades.0[0].1, 2);
        assert_eq!(trades.1, 0);
        assert!(order_book.check_validity_quantity());
        assert_eq!(order_book.asks.get_total_quantity(), 16);

        let limit_order = LimitOrder {
            order_id: order_counter.next_id(),
            price: 102,
            quantity: 2,
            order_side: OrderSide::Bid,
        };

        let _ = order_book.process_limit_order(limit_order).unwrap();
        assert!(order_book.check_validity_quantity());
        assert_eq!(order_book.asks.get_total_quantity(), 14);
        assert_eq!(order_book.asks.best_price, 100);

        let ask_limit_order = LimitOrder {
            order_id: order_counter.next_id(),
            price: 102,
            quantity: 4,
            order_side: OrderSide::Bid,
        };
    
        let (trades, rem) = order_book.process_limit_order(ask_limit_order).unwrap();
        assert!((trades.average_trade_price() - 100.5).abs() < 1e-6);
        assert_eq!(rem, 0);
        assert!(order_book.check_validity_quantity());
        assert_eq!(order_book.asks.get_total_quantity(), 10);
        assert_eq!(order_book.asks.best_price, 101);

        let bid_limit_order = LimitOrder {
            order_id: order_counter.next_id(),
            price: 101,
            quantity: 6,
            order_side: OrderSide::Bid,
        };
    
        let (trades, rem) = order_book.process_limit_order(bid_limit_order).unwrap();
        assert!((trades.average_trade_price() - 101.0).abs() < 1e-6);
        assert!(order_book.check_validity_quantity());
        assert_eq!(order_book.asks.get_total_quantity(), 6);
        assert_eq!(order_book.asks.best_price, 102);
        assert_eq!(order_book.bids.get_total_quantity(), 20);
        assert_eq!(order_book.bids.best_price, 101);


        let remove_order = RemoveAnyOrder {
            price: 101,
            quantity: 6,
            order_side: OrderSide::Bid,
        };

        let rem = order_book.remove_order(remove_order).unwrap();
        assert_eq!(rem, 4);
        assert!(order_book.check_validity_quantity());
        assert_eq!(order_book.bids.get_total_quantity(), 18);
        assert_eq!(order_book.bids.best_price, 99);

    }

    #[test]
    fn test_orderbook_update_from_quote_snapshot() {
        let mut test_data_vec = b"B602F        G140KR4106V30004000020104939405656001379.70001379.500000000030000000030000300003001379.80001379.400000000040000000040000400004001379.90001379.300000000070000000050000600005001380.00001379.200000000050000000070000500007001380.10001379.1000000000500000000500005000050000009020000025920031700642000000.00000000000".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0034 = crate::data::krx::derivative_quote::IFMSRPD0034::default();
        
        let mut date_gen = crate::types::timestamp::DateUnixNanoGenerator::from(
            time::macros::date!(2021-12-30)
        );

        let quote_snapshot = ifmsrpd0034.to_quote_snapshot(test_data, &mut date_gen)
            .expect("failed to parse IFMSRPD0034");
        
        let mut order_book = OrderBook::initialize_with_id(quote_snapshot.id);
        order_book.update_from_quote_snapshot(&quote_snapshot).unwrap();
        
        let display = order_book.to_string();
        println!("{}", display);
        //dbg!(order_book.clone());
    }
}