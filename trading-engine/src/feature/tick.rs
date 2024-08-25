use crate::types::base::{
    Real,
    NormalizedReal,
};
use crate::types::timestamp::TimeStamp;
use crate::data::{
    trade_quote::TradeQuoteSnapshot,
    quote::QuoteSnapshot,
    trade::TradeData,
};
use crate::utils::numeric_converter::{
    IntegerConverter,
    OrderConverter,
};
use crate::data::get_default_order_converter;
use anyhow::{
    Result,
    anyhow,
};
use serde::Serialize;
const VWAP_NORM_FACTOR: Real = 100.0;

#[derive(Debug, Clone)]
pub struct TradePrice {
    value: NormalizedReal,
    timestamp: TimeStamp,
    converter: &'static IntegerConverter, // to roll-back to the original value
}

impl Default for TradePrice {
    fn default() -> Self {
        let converter = get_default_order_converter();
        Self {
            value: 0.0,
            timestamp: TimeStamp::default(),
            converter: &converter.price,
        }
    }
}

impl TradePrice {
    pub fn new_from_trade_data(data: &TradeData) -> Self {
        let order_converter = data.order_converter;
        let timestamp = data.timestamp;
        Self {
            value: data.to_normalized_real(),
            timestamp,
            converter: &order_converter.price,
        }
    }

    pub fn update_trade_data(&mut self, data: &TradeData) {
        self.value = data.to_normalized_real();
        self.timestamp = data.timestamp;
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct SimpleSnapshot {
    price: NormalizedReal,
    quantity: NormalizedReal,
}

/// L2 Snapshot of Quotes
#[derive(Debug, Clone, Serialize)]
pub struct SimpleQuotes {
    bids: Vec<SimpleSnapshot>,
    asks: Vec<SimpleSnapshot>,
    level_cut: usize,
    timestamp: TimeStamp,
    system_time: TimeStamp,
    #[serde(skip)]
    order_converter: &'static OrderConverter,
}

impl Default for SimpleQuotes {
    fn default() -> Self {
        let order_converter = get_default_order_converter();
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            level_cut: 0,
            timestamp: TimeStamp::default(),
            system_time: TimeStamp::default(),
            order_converter,
        }
    }
}

impl SimpleQuotes {
    pub fn with_capacity(capacity: usize) -> Self {
        let converter = get_default_order_converter();
        Self {
            bids: Vec::with_capacity(capacity),
            asks: Vec::with_capacity(capacity),
            level_cut: capacity,
            timestamp: TimeStamp::default(),
            system_time: TimeStamp::default(),
            order_converter: converter,
        }
    }

    #[inline]
    pub fn vwap(&self) -> Option<Real> {
        let mut total_quantity = 0.0;
        let mut total_value = 0.0;


        for ask in &self.asks {
            total_quantity += ask.quantity;
            total_value += ask.price * ask.quantity / VWAP_NORM_FACTOR;
        }

        if total_quantity < Real::EPSILON {
            return None;
        }

        for bid in &self.bids {
            total_quantity += bid.quantity;
            total_value += bid.price * bid.quantity / VWAP_NORM_FACTOR;
        }

        if total_quantity < Real::EPSILON {
            return None;
        }
        
        Some(total_value / total_quantity * VWAP_NORM_FACTOR)
        
    }

    #[inline]
    pub fn mid_price(&self) -> Option<Real> {
        for ask in &self.asks {
            for bid in &self.bids {
                if ask.quantity > 0.0 && bid.quantity > 0.0 {
                    return Some((ask.price + bid.price) / 2.0);
                    
                }
            }
        }
        None
    }

    #[inline]
    pub fn best_ask_price(&self) -> Option<Real> {
        for ask in &self.asks {
            if ask.quantity > 0.0 {
                return Some(ask.price);
            }
        }
        None
    }

    #[inline]
    pub fn best_bid_price(&self) -> Option<Real> {
        for bid in &self.bids {
            if bid.quantity > 0.0 {
                return Some(bid.price);
            }
        }
        None
    }

    pub fn new_from_quote_sanpshot(data: &QuoteSnapshot) -> Self {
        let order_converter = data.order_converter;
        //
        let quote_cut = data.quote_level_cut;
        let iter_nuum = data.effective_bid_data().len().min(quote_cut);
        //
        let bids = data.effective_bid_data().iter().take(iter_nuum).map(|quote| 
           SimpleSnapshot {
                price: order_converter.price.normalized_real_from_i64(quote.book_price),
                quantity: order_converter.quantity.normalized_real_from_u64(quote.book_quantity),
            }
        ).collect();
        let asks = data.effective_ask_data().iter().take(iter_nuum).map(|quote| 
            SimpleSnapshot {
                price: order_converter.price.normalized_real_from_i64(quote.book_price),
                quantity: order_converter.quantity.normalized_real_from_u64(quote.book_quantity),
            }
        ).collect();
        
        Self {
            bids,
            asks,
            level_cut: quote_cut,
            timestamp: data.timestamp,
            system_time: data.system_timestamp,
            order_converter,
        }
    }

    pub fn new_from_trade_quote_snapshot(data: &TradeQuoteSnapshot) -> Self {
        let order_converter = data.order_converter;
        //
        let quote_cut = data.quote_level_cut;
        let iter_nuum = data.bid_quote_data.len().min(quote_cut);
        //
        let bids = data.bid_quote_data.iter().take(iter_nuum).map(|quote| 
            SimpleSnapshot {
                price: order_converter.price.normalized_real_from_i64(quote.book_price),
                quantity: order_converter.quantity.normalized_real_from_u64(quote.book_quantity),
            }
        ).collect();
        let asks = data.ask_quote_data.iter().take(iter_nuum).map(|quote| 
            SimpleSnapshot {
                price: order_converter.price.normalized_real_from_i64(quote.book_price),
                quantity: order_converter.quantity.normalized_real_from_u64(quote.book_quantity),
            }
        ).collect();
        
        Self {
            bids,
            asks,
            level_cut: quote_cut,
            timestamp: data.timestamp,
            system_time: data.system_timestamp,
            order_converter,
        }
    }

    pub fn update_trade_quote_snapshot(&mut self, data: &TradeQuoteSnapshot) -> Result<()> {
        if data.quote_level_cut < self.level_cut {
            let err = || anyhow!(
                "Quotes (feature) level cut ({}) is less than \n
                TradeQuoteSnapshot (data) level cut ({})",
                self.level_cut,
                data.quote_level_cut
            );
            let self_cut = self.level_cut;
            let data_clone = data.clone();
            crate::log_error!(
                crate::LogTopic::OfiLevelMismatch.as_str(), 
                feature_level_cut = self_cut,
                data = data_clone);
            return Err(err());
        }
        self.order_converter = data.order_converter;
        self.timestamp = data.timestamp;

        for i in 0..self.level_cut {
            self.bids[i].price = self.order_converter.price.normalized_real_from_i64(
                data.bid_quote_data[i].book_price);
            self.bids[i].quantity = self.order_converter.quantity.normalized_real_from_u64(
                data.bid_quote_data[i].book_quantity);
            //
            self.asks[i].price = self.order_converter.price.normalized_real_from_i64(
                data.ask_quote_data[i].book_price);
            self.asks[i].quantity = self.order_converter.quantity.normalized_real_from_u64(
                data.ask_quote_data[i].book_quantity);
        }

        Ok(())
    }   

    pub fn update_quote_snapshot(&mut self, data: &QuoteSnapshot) -> Result<()> {
        if data.quote_level_cut < self.level_cut {
            let err = || anyhow!(
                "Quotes (feature) level cut ({}) is less than \n
                QuoteSnapshot (data) level cut ({})",
                self.level_cut,
                data.quote_level_cut
            );
            let self_cut = self.level_cut;
            let data_clone = data.clone();
            crate::log_error!(
                crate::LogTopic::OfiLevelMismatch.as_str(), 
                feature_level_cut = self_cut,
                data = data_clone);
            return Err(err());
        }
        self.order_converter = data.order_converter;
        self.timestamp = data.timestamp;

        for i in 0..self.level_cut {
            self.bids[i].price = self.order_converter.price.normalized_real_from_i64(
                data.effective_bid_data()[i].book_price);
            self.bids[i].quantity = self.order_converter.quantity.normalized_real_from_u64(
                data.effective_bid_data()[i].book_quantity);
            //
            self.asks[i].price = self.order_converter.price.normalized_real_from_i64(
                data.effective_ask_data()[i].book_price);
            self.asks[i].quantity = self.order_converter.quantity.normalized_real_from_u64(
                data.effective_ask_data()[i].book_quantity);
        }

        Ok(())
    }
}

impl SimpleQuotes {
    pub fn get_bid_orderflow_imbalance(
        &self,
        level: usize,
        arriving_quotes: &SimpleQuotes,
    ) -> Result<NormalizedReal> {
        if level == 0 {
            return Err(anyhow!("as a convention, ofi level starts from 1"));
        }

        if level > self.level_cut || level > arriving_quotes.level_cut {
            let err = || anyhow!(
                "ofi level exceeds the level cut\n\
                previous level cut: {}\n\
                arriving level cut: {}\n\
                arriving quotes: {:?}",
                self.level_cut, arriving_quotes.level_cut, arriving_quotes
            );

            let self_clone = self.clone();
            let arriving_quotes_clone = arriving_quotes.clone();

            crate::log_error!(
                crate::LogTopic::OfiLevelMismatch.as_str(), 
                prev_quote = self_clone,
                current_quotes = arriving_quotes_clone);

            return Err(err());
        }

        let previous_bid = &self.bids[level - 1];
        let current_bid = &arriving_quotes.bids[level - 1];

        let res = if current_bid.price > previous_bid.price {
            current_bid.quantity
        } else if current_bid.price == previous_bid.price {
            current_bid.quantity - previous_bid.quantity
        } else {
            -previous_bid.quantity
        };
        
        Ok(res)
    }

    pub fn get_ask_orderflow_imbalance(
        &self,
        level: usize,
        arriving_quotes: &SimpleQuotes,
    ) -> Result<NormalizedReal> {
        if level == 0 {
            return Err(anyhow!("as a convention, ofi level starts from 1"));
        }

        if level > self.level_cut || level > arriving_quotes.level_cut {
            let err = || anyhow!(
                "ofi level exceeds the level cut\n\
                previous level cut: {}\n\
                arriving level cut: {}\n\
                arriving quotes: {:?}",
                self.level_cut, arriving_quotes.level_cut, arriving_quotes
            );

            let self_clone = self.clone();
            let arriving_quotes_clone = arriving_quotes.clone();

            crate::log_error!(
                crate::LogTopic::OfiLevelMismatch.as_str(), 
                prev_quote = self_clone,
                current_quotes = arriving_quotes_clone);

            return Err(err());
        }

        let previous_ask = &self.asks[level - 1];
        let current_ask = &arriving_quotes.asks[level - 1];

        let res = if current_ask.price > previous_ask.price {
            - previous_ask.quantity
        } else if current_ask.price == previous_ask.price {
            current_ask.quantity - previous_ask.quantity
        } else {
            current_ask.quantity
        };

        Ok(res)
    }

    pub fn get_ask_ln_orderflow_imbalance(
        &self,
        level: usize,
        arriving_quotes: &SimpleQuotes,
    ) -> Result<NormalizedReal> {
        if level == 0 {
            return Err(anyhow!("as a convention, ofi level starts from 1"));
        }

        if level > self.level_cut || level > arriving_quotes.level_cut {
            let err = || anyhow!(
                "ofi level exceeds the level cut\n\
                previous level cut: {}\n\
                arriving level cut: {}\n\
                arriving quotes: {:?}",
                self.level_cut, arriving_quotes.level_cut, arriving_quotes
            );

            let self_clone = self.clone();
            let arriving_quotes_clone = arriving_quotes.clone();

            crate::log_error!(
                crate::LogTopic::OfiLevelMismatch.as_str(), 
                prev_quote = self_clone,
                current_quotes = arriving_quotes_clone);

            return Err(err());
        }

        let previous_ask = &self.asks[level - 1];
        let current_ask = &arriving_quotes.asks[level - 1];

        let res = if current_ask.price > previous_ask.price {
            if current_ask.quantity > 0.0 {
                - previous_ask.quantity.ln()
            } else { 
                let current_ask_clone = current_ask.clone();
                crate::log_warn!(
                    crate::LogTopic::ZeroQuantity.as_str(), 
                    current_ask = current_ask_clone,
                );
                0.0 
            }
        } else if current_ask.price == previous_ask.price {
            if previous_ask.quantity > 0.0 {
                (current_ask.quantity / previous_ask.quantity).ln()
            } else { 
                let current_ask_clone = current_ask.clone();
                crate::log_warn!(
                    crate::LogTopic::ZeroQuantity.as_str(), 
                    current_ask = current_ask_clone,
                );
                0.0 
            }
        } else {
            if current_ask.quantity > 0.0 {
                current_ask.quantity.ln()
            } else { 
                let current_ask_clone = current_ask.clone();
                crate::log_warn!(
                    crate::LogTopic::ZeroQuantity.as_str(), 
                    current_ask = current_ask_clone,
                );
                0.0 
            }
        };

        Ok(res)
    }

    pub fn get_bid_ln_orderflow_imbalance(
        &self,
        level: usize,
        arriving_quotes: &SimpleQuotes,
    ) -> Result<NormalizedReal> {
        if level == 0 {
            return Err(anyhow!("as a convention, ofi level starts from 1"));
        }

        if level > self.level_cut || level > arriving_quotes.level_cut {
            let err = || anyhow!(
                "ofi level exceeds the level cut\n\
                previous level cut: {}\n\
                arriving level cut: {}\n\
                arriving quotes: {:?}",
                self.level_cut, arriving_quotes.level_cut, arriving_quotes
            );

            let self_clone = self.clone();
            let arriving_quotes_clone = arriving_quotes.clone();

            crate::log_error!(
                crate::LogTopic::OfiLevelMismatch.as_str(), 
                prev_quote = self_clone,
                current_quotes = arriving_quotes_clone);

            return Err(err());
        }

        let previous_bid = &self.bids[level - 1];
        let current_bid = &arriving_quotes.bids[level - 1];

        let res = if current_bid.price > previous_bid.price {
            if current_bid.quantity > 0.0 {
                current_bid.quantity.ln()
            } else { 
                let current_bid_clone = current_bid.clone();
                crate::log_warn!(
                    crate::LogTopic::ZeroQuantity.as_str(), 
                    current_bid = current_bid_clone,
                );
                0.0 
            }
        } else if current_bid.price == previous_bid.price {
            if previous_bid.quantity > 0.0 {
                (current_bid.quantity / previous_bid.quantity).ln()
            } else { 
                let current_bid_clone = current_bid.clone();
                crate::log_warn!(
                    crate::LogTopic::ZeroQuantity.as_str(), 
                    current_bid = current_bid_clone,
                );
                0.0 
            }
        } else if current_bid.quantity > 0.0 {
            - previous_bid.quantity.ln()
        } else { 
            let current_bid_clone = current_bid.clone();
            crate::log_warn!(
                crate::LogTopic::ZeroQuantity.as_str(), 
                current_bid = current_bid_clone,
            );
            0.0 
        };

        Ok(res)
    }

    pub fn get_orederflow_imbalance(
        &self,
        level: usize,
        arriving_quotes: &SimpleQuotes,
    ) -> Result<NormalizedReal> {
        let bid_imbalance = self.get_bid_orderflow_imbalance(level, arriving_quotes)?;
        let ask_imbalance = self.get_ask_orderflow_imbalance(level, arriving_quotes)?;

        Ok(bid_imbalance - ask_imbalance)
    }

    pub fn get_ln_orderflow_imbalance(
        &self,
        level: usize,
        arriving_quotes: &SimpleQuotes,
    ) -> Result<NormalizedReal> {
        let bid_imbalance = self.get_bid_ln_orderflow_imbalance(level, arriving_quotes)?;
        let ask_imbalance = self.get_ask_ln_orderflow_imbalance(level, arriving_quotes)?;

        Ok(bid_imbalance - ask_imbalance)
    }

}