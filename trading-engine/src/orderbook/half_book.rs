use crate::types::{
    enums::OrderSide,
    precision::Precision,
    book_price::BookPrice,
    venue::OrderId,
};
use crate::data::book_order::BookOrder;
use crate::orderbook::level::Level;
//
use hashbrown::HashMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use anyhow::Result;


#[derive(Debug, Clone, Default)]
pub struct HalfBook
<T: Precision + Clone + Debug + Ord, S: Precision + Clone + Debug> {
    pub order_side: OrderSide,
    pub levels: BTreeMap<BookPrice<T>, Level<T, S>>,
    pub cache: HashMap<OrderId, BookPrice<T>>,
}

impl<T: Precision + Clone + Debug + Ord, S: Precision + Clone + Debug> HalfBook<T, S> {
    #[must_use]
    pub fn initialize(order_side: OrderSide) -> Self {
        HalfBook {
            order_side,
            levels: BTreeMap::new(),
            cache: HashMap::new(),
        }
    }

    #[must_use]
    pub fn initialize_with_order(order: BookOrder<T, S>) -> Self {
        let mut levels = BTreeMap::new();
        let price = order.price.clone();
        let level = Level::initialize_with_order(order.clone());
        levels.insert(price.clone(), level);
        let mut cache = HashMap::new();
        cache.insert(order.order_id, price);
        HalfBook {
            order_side: order.order_side,
            levels,
            cache,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.levels.len()
    }

    pub fn clear(&mut self) {
        self.levels.clear();
        self.cache.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    pub fn add_order(&mut self, order: BookOrder<T, S>) -> Result<()> {
        self.cache.insert(order.order_id, order.price.clone());

        match self.levels.get_mut(&order.price) {
            Some(level) => {
                level.add_order(order)?;
            },
            None => {
                let level = Level::initialize_with_order(order.clone());
                self.levels.insert(order.price.clone(), level);
            }
        }

        Ok(())
    }
}