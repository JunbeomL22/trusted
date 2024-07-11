use crate::types::{
    base::{
        OrderId,
        BookPrice,
    },
    enums::OrderSide,
};
use crate::data::book_order::BookOrder;
use crate::orderbook::level::Level;
//
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use anyhow::Result;


#[derive(Debug, Clone, Default)]
pub struct HalfBook {
    pub order_side: OrderSide,
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
        let price = order.price.clone();
        let level = Level::initialize_with_order(order.clone());
        levels.insert(price.clone(), level);
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
            },
            None => {
                let level = Level::initialize_with_order(order.clone());
                self.levels.insert(order.price, level);
            }
        }

        Ok(())
    }
}