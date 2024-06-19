use crate::data::book_order::BookOrder;
use crate::types::{
    types::OrderId,
    book_price::BookPrice,
    precision::Precision,
};
//
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Level<T: Precision + Clone> {
    pub book_price: BookPrice<T>,
    pub orders: BTreeMap<OrderId, BookOrder<T>>,
    arraival_order: Vec<OrderId>,
}

impl<T: Precision + Clone> Level<T> {
    #[must_use]
    pub fn initialize(book_price: BookPrice<T>) -> Self {
        Level {
            book_price: book_price,
            orders: BTreeMap::new(),
            arraival_order: Vec::new(), // *optimizable* assign with capacityk
        }
    }

    #[must_use]
    pub fn initialize_with_order(order: BookOrder<T>) -> Self {
        let mut orders = BTreeMap::new();
        let order_clone = order.clone();
        orders.insert(order.order_id.clone(), order);
        Level {
            book_price: order_clone.price,
            orders: orders,
            arraival_order: vec![order_clone.order_id],
        }
    }

    pub fn add_order(&mut self, order: BookOrder<T>) {
        self.arraival_order.push(order.order_id.clone());
        self.orders.insert(order.order_id, order);
    }

    pub fn remove_order(&mut self, order_id: OrderId) {
        self.orders.remove(&order_id);
        self.arraival_order.retain(|&x| x != order_id);
    }

    pub fn get_order(&self, order_id: OrderId) -> Option<&BookOrder<T>> {
        self.orders.get(&order_id)
    }

    pub fn price(&self) -> f64 {
        self.book_price.as_f64()
    }

    pub fn book_quanity_sum(&self) -> u64 {
        self.orders
            .values()
            .map(|order| order.quantity.iovalue)
            .sum()
    }

    pub fn quantity_sum(&self) -> f64 {
        self.orders
            .values()
            .map(|order| order.quantity.as_f64())
            .sum()
    }

    pub fn order_count(&self) -> usize {
        self.orders.len()
    }

    #[must_use]
    pub fn get_orders_in_arrival_order(&self) -> Vec<&BookOrder<T>> {
        self.arraival_order
            .iter()
            .filter_map(|&order_id| self.orders.get(&order_id))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

}