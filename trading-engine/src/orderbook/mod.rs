pub mod half_book;
pub mod level;
//
//
use crate::types::enums::OrderSide;
use crate::orderbook::half_book::HalfBook;
use crate::data::order::{
    LimitOrder,
    MarketOrder,
};
use crate::data::{
    quote::QuoteSnapshot,
    trade_quote::TradeQuoteSnapshot,
};
use crate::topics::LogTopic;
use crate::types::{
    isin_code::IsinCode,
    venue::Venue,
    base::{
        OrderId,
        BookPrice,
        BookQuantity,
        TradeHistory,
    },
};
//
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Default)]
pub struct OrderBook {
    pub asks: HalfBook,
    pub bids: HalfBook,
    isin_code: IsinCode,
    venue: Venue,
}

impl OrderBook {
    fn check_isin_venue(&self, isin_code: &IsinCode, venue: Venue) -> Result<()> {
        if isin_code != &self.isin_code {
            let err = || anyhow!(
                "Isin code mismatch orderbook: {:?} input: {:?}",
                self.isin_code, isin_code,
            );
            return Err(err());
        } 
        if venue != self.venue {
            let err = || anyhow!(
                "Venue mismatch orderbook: {:?} input: {:?}",
                self.venue, venue,
            );
            return Err(err());
        }
        Ok(())
    }

    /// update by snapshot
    #[inline]
    pub fn update_from_quote_snapshot(&mut self, quote: &QuoteSnapshot) -> Result<()> {
        self.check_isin_venue(&quote.isin_code, quote.venue)?;
        self.asks.update_by_level_snapshot(&quote.ask_quote_data);
        self.bids.update_by_level_snapshot(&quote.bid_quote_data);

        Ok(())
    }

    #[inline]
    pub fn update_from_trade_quote_snapshot(&mut self, trade_quote: &TradeQuoteSnapshot) -> Result<()> {
        self.check_isin_venue(&trade_quote.isin_code, trade_quote.venue)?;
        self.asks.update_by_level_snapshot(&trade_quote.ask_quote_data);
        self.bids.update_by_level_snapshot(&trade_quote.bid_quote_data);

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
    pub fn initialize_with_isin_venue(isin_code: IsinCode, venue: Venue) -> Self {
        OrderBook {
            asks: HalfBook::initialize(OrderSide::Ask),
            bids: HalfBook::initialize(OrderSide::Bid),
            isin_code,
            venue,
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
    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<()> {
        if let Some(_) = self.asks.cancel_order(order_id) {
            Some(())
        } else if let Some(_) = self.bids.cancel_order(order_id) {
            Some(())
        } else {
            let message = format!(
                "Order {} not found\n\
                isin: {}, venue: {:?}",
                order_id, self.isin_code.as_str(), self.venue);

            crate::log_warn!(LogTopic::OrderNotFound, message = message);
            None
        }
    }

    /// None means order_id not found, either in cache or levels
    #[inline]
    pub fn change_price(&mut self, order_id: OrderId, new_price: BookPrice) -> Option<()> {
        if let Some(_) = self.asks.change_price(order_id, new_price) {
            Some(())
        } else if let Some(_) = self.bids.change_price(order_id, new_price) {
            Some(())
        } else {
            let message = format!(
                "Order {} not found\n\
                isin: {}, venue: {:?}",
                order_id, self.isin_code.as_str(), self.venue);

            crate::log_warn!(LogTopic::OrderNotFound, message = message);
            None
        }
    }

    /// None means order_id not found, either in cache or levels
    #[inline]
    pub fn change_quantity(&mut self, order_id: OrderId, new_quantity: BookQuantity) -> Option<()> {
        if let Some(_) = self.asks.change_quantity(order_id, new_quantity) {
            Some(())
        } else if let Some(_) = self.bids.change_quantity(order_id, new_quantity) {
            Some(())
        } else {
            let message = format!(
                "Order {} not found\n\
                isin: {}, venue: {:?}",
                order_id, self.isin_code.as_str(), self.venue);

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

    #[test]
    fn test_orderbook_initialize() {
       unimplemented!();
    }

    #[test]
    fn test_orderbook_update_from_quote_snapshot() {
        unimplemented!();
    }
}