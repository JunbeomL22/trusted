use crate::data::book_order::BookOrder;
use crate::types::{
    venue::OrderId,
    book_price::BookPrice,
    precision::Precision,
};
use crate::warn;
//
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use anyhow::{Result, anyhow};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Level
<T: Precision + Clone + Debug, S: Precision + Clone + Debug> {
    pub book_price: BookPrice<T>,
    pub orders: BTreeMap<OrderId, BookOrder<T, S>>,
    arraival_order: Vec<OrderId>,
}

impl<T: Precision + Clone + Debug, S: Precision + Clone + Debug> Level<T, S> {
    #[must_use]
    pub fn initialize(book_price: BookPrice<T>) -> Self {
        Level {
            book_price: book_price,
            orders: BTreeMap::new(),
            arraival_order: Vec::new(), // *optimizable* assign with capacityk
        }
    }

    #[must_use]
    pub fn initialize_with_order(order: BookOrder<T, S>) -> Self {
        let mut orders = BTreeMap::new();
        let order_clone = order.clone();
        orders.insert(order.order_id.clone(), order);
        Level {
            book_price: order_clone.price,
            orders: orders,
            arraival_order: vec![order_clone.order_id],
        }
    }

    pub fn add_order(&mut self, order: BookOrder<T, S>) -> Result<()> {
        if self.book_price.iovalue != order.price.iovalue {
            let lazy_error = || anyhow!(
                "Price mismatch\n
                level: {:?}\n
                input_order: {:?}",
                self,
                order,
            );
            return Err(lazy_error());
        } else {
            self.arraival_order.push(order.order_id.clone());
            self.orders.insert(order.order_id, order);
            return Ok(())
        }
    }

    pub fn remove_order(&mut self, order_id: OrderId) {
        self.orders.remove(&order_id);
        self.arraival_order.retain(|&x| x != order_id);
    }

    pub fn get_order(&self, order_id: OrderId) -> Option<&BookOrder<T, S>> {
        self.orders.get(&order_id)
    }

    pub fn price(&self) -> f64 {
        self.book_price.as_f64()
    }

    pub fn book_quanity_sum(&self) -> u64 {
        warn!("book_quanity_sum is deprecated. Use quantity_sum instead.");
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
    pub fn get_orders_in_arrival_order(&self) -> Vec<&BookOrder<T, S>> {
        self.arraival_order
            .iter()
            .filter_map(|&order_id| self.orders.get(&order_id))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::precision::{
        Prec0,
        Prec2,
    };
    use crate::types::book_quantity::BookQuantity;
    use crate::types::enums::OrderSide;
    use crate::utils::counter::UnboundedCounterU64;
    use crate::types::venues::krx::KrxOrderId;

    #[test]
    fn test_level() -> Result<()> {
        let counter = UnboundedCounterU64::new(1);
        let _counter = counter.next();
        let _order_id = KrxOrderId::new(_counter);
        let order_id1 = OrderId::KrxOrderId(_order_id);

        let order1 = BookOrder {
            price: BookPrice::<Prec2>::new(101.123).unwrap(),
            quantity: BookQuantity::<Prec0>::new(1.0).unwrap(),
            order_side: OrderSide::Buy,
            order_id: order_id1.clone(),
        };

        let _counter = counter.next();
        let _order_id = KrxOrderId::new(_counter);
        let order_id2 = OrderId::KrxOrderId(_order_id);

        let order2 = BookOrder {
            price: BookPrice::<Prec2>::new(101.123).unwrap(),
            quantity: BookQuantity::<Prec0>::new(2.0).unwrap(),
            order_side: OrderSide::Buy,
            order_id: order_id2.clone(),
        };

        let _counter = counter.next();
        let _order_id = KrxOrderId::new(_counter);
        let order_id3 = OrderId::KrxOrderId(_order_id);

        let order3 = BookOrder {
            price: BookPrice::<Prec2>::new(101.123).unwrap(),
            quantity: BookQuantity::<Prec0>::new(3.0).unwrap(),
            order_side: OrderSide::Buy,
            order_id: order_id3.clone(),
        };

        let mut level = Level::<Prec2, Prec0>::initialize_with_order(order1);
        level.add_order(order2)?;
        level.add_order(order3)?;

        //dbg!(level.clone());

        assert_eq!(level.price(), 101.12);
        assert_eq!(level.book_quanity_sum(), 6_000_000_000); 
        assert_eq!(level.quantity_sum(), 6.0);
        assert_eq!(level.order_count(), 3);

        let orders = level.get_orders_in_arrival_order();
        assert_eq!(orders.len(), 3);
        assert_eq!(orders[0].order_id, 1);
        assert_eq!(orders[1].order_id, 2);
        assert_eq!(orders[2].order_id, 3);

        level.remove_order(order_id2);
        assert_eq!(level.quantity_sum(), 4.0);

        Ok(())
    }
}