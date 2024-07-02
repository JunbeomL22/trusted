use crate::data::book_order::BookOrder;
use crate::types::{
    base::{
        OrderId,
        BookPrice,
    },
    precision::PrecisionHelper,
};
use crate::warn;
//
use std::collections::BTreeMap;
use anyhow::{Result, anyhow};
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct Level
{
    pub book_price: BookPrice,
    pub orders: BTreeMap<OrderId, BookOrder>,
    arraival_order: Vec<OrderId>,
}

impl Level {
    #[must_use]
    #[inline]
    pub fn initialize(book_price: BookPrice) -> Self {
        Level {
            book_price,
            orders: BTreeMap::new(),
            arraival_order: Vec::new(), // *optimizable* assign with capacityk
        }
    }

    #[must_use]
    #[inline]
    pub fn initialize_with_order(order: BookOrder) -> Self {
        let mut orders = BTreeMap::new();
        let order_clone = order.clone();
        orders.insert(order.order_id, order);
        Level {
            book_price: order_clone.price,
            orders,
            arraival_order: vec![order_clone.order_id],
        }
    }

    pub fn add_order(&mut self, order: BookOrder) -> Result<()> {
        if self.book_price != order.price {
            let lazy_error = || anyhow!(
                "Price mismatch\n
                level: {:?}\n
                input_order: {:?}",
                self,
                order,
            );
            Err(lazy_error())
        } else {
            self.arraival_order.push(order.order_id);
            self.orders.insert(order.order_id, order);
            Ok(())
        }
    }

    #[must_use]
    #[inline]
    pub fn remove_order(&mut self, order_id: OrderId) {
        self.orders.remove(&order_id);
        self.arraival_order.retain(|&x| x != order_id);
    }

    #[must_use]
    #[inline]
    pub fn get_order(&self, order_id: OrderId) -> Option<&BookOrder> {
        self.orders.get(&order_id)
    }

    #[must_use]
    #[inline]
    pub fn price(&self, prec_helper: PrecisionHelper) -> f64 {
        prec_helper.price_i64_to_f64(self.book_price)
    }

    pub fn book_quanity_sum(&self) -> u64 {
        warn!("book_quanity_sum is deprecated. Use quantity_sum instead.");
        self.orders
            .values()
            .map(|order| order.quantity)
            .sum()
    }

    pub fn quantity_sum(&self, prec_helper: PrecisionHelper) -> f64 {
        self.orders
            .values()
            .map(|order| prec_helper.quantity_u64_to_f64(order.quantity))   
            .sum()
    }

    #[must_use]
    #[inline]
    pub fn order_count(&self) -> usize {
        self.orders.len()
    }

    #[must_use]
    pub fn get_orders_in_arrival_order(&self) -> Vec<&BookOrder> {
        self.arraival_order
            .iter()
            .filter_map(|&order_id| self.orders.get(&order_id))
            .collect()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::OrderSide;
    use crate::types::base::{
        OrderId,
        BookPrice,
        BookQuantity,
    };

    #[test]
    fn test_level() {
        let price: BookPrice = 100;
        let quantity: BookQuantity = 100;
        let order_side: OrderSide = OrderSide::Buy;
        let order_id: OrderId = 1;

        let book_order = BookOrder::new(price, quantity, order_side, order_id);

        let level = Level::initialize_with_order(book_order.clone());

        assert_eq!(level.book_price, price);
        assert_eq!(level.orders.len(), 1);
        assert_eq!(level.orders.get(&order_id), Some(&book_order));
        assert_eq!(level.arraival_order.len(), 1);
        assert_eq!(level.arraival_order[0], order_id);
        assert_eq!(level.price(PrecisionHelper::Prec0_3), 100.0);
        assert_eq!(level.book_quanity_sum(), 100);
        assert_eq!(level.quantity_sum(PrecisionHelper::Prec0_3), 100.0);
        assert_eq!(level.order_count(), 1);
        assert_eq!(level.get_orders_in_arrival_order(), vec![&book_order]);
        assert_eq!(level.is_empty(), false);
    }
}