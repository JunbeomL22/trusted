pub mod half_book;
pub mod level;
//
//
use crate::types::enums::OrderSide;
use crate::orderbook::half_book::HalfBook;
use crate::data::order::{
    LimitOrder,
    MarketOrder,
    OrderRequest,
};
use crate::topics::LogTopic;
use crate::types::{
    isin_code::IsinCode,
    venue::Venue,
    base::{
        OrderId,
        BookPrice,
        BookQuantity,
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

    /// It trades the order from the best_price. 
    /// If the order is worse than the best (higher for best ask, lower for best bid),
    /// it will be added to the book.
    #[inline]
    pub fn trade_limit_order(&mut self, mut order: LimitOrder) -> Result<()> {
        match order.order_side {
            OrderSide::Ask => {
                let rem = self.bids.trade_limit_order(&order).unwrap();
                if rem > 0 {
                    order.quantity = rem;
                    self.asks.add_limit_order(order)?;
                }
            },
            OrderSide::Bid => {
                let rem = self.asks.trade_limit_order(&order).unwrap();
                if rem > 0 {
                    order.quantity = rem;
                    self.bids.add_limit_order(order)?;
                }
            },
        }
        Ok(())
    }

    /// None means there is nothing traded.
    /// Otherwise, it returns the last traded price and quantity.
    /// The traded price may not be the unique. If the ask market-order is larger than the best price of the bid.
    /// Then the it is traded at the best bid, then the next best bid, and so on.
    #[inline]
    pub fn trade_market_order(&mut self, order: MarketOrder) -> Option<(BookPrice, BookQuantity)> {
        match order.order_side {
            OrderSide::Ask => self.bids.trade_market_order(order.quantity),
            OrderSide::Bid => self.asks.trade_market_order(order.quantity),
        }
    }

}