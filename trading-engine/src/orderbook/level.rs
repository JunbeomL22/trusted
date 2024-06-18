use crate::data::order::Order;
use crate::types::{
    types::{OrderId, TraderId},
    price::Price,
    precision::Precision,
};
//
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Level<T: Precision> {
    pub price: Price<T>,
    pub orders: BTreeMap<OrderId, Order>,
    arraival_order: Vec<OrderId>,
}

impl<T: Precision> Level<T> {
    #[must_use]
    pub fn initialize(price: Price<T>) -> Self {
        Level {
            price: price,
            orders: BTreeMap::new(),
            arraival_order: Vec::new(), // *optimizable* assign with capacityk
        }
    }

    #[must_use]
    pub fn initialize_with_order(order: Order) -> Self {
        let mut orders = BTreeMap::new();
        orders.insert(order.order_id, order);
        Level {
            price: order.price.clone(),
            orders: orders,
            arraival_order: vec![order.order_id],
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.insert(order.order_id, order);
        self.arraival_order.push(order.order_id);
    }

    pub fn remove_order(&mut self, order_id: OrderId) {
        self.orders.remove(&order_id);
        self.arraival_order.retain(|&x| x != order_id);
    }

    pub fn get_order(&self, order_id: OrderId) -> Option<&Order> {
        self.orders.get(&order_id)
    }

    #[must_use]
    pub fn get_orders_in_arrival_order(&self) -> Vec<&Order> {
        self.arraival_order
            .iter()
            .filter_map(|&order_id| self.orders.get(&order_id))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }

}