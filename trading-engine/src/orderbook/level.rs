use crate::data::book_order::BookOrder;
use crate::types::base::{OrderId, BookPrice};
use crate::utils::numeric_converter::IntegerConverter;
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
    pub fn price(&self, converter: &mut IntegerConverter) -> f64 {
        converter.to_f64_from_i64(self.book_price)
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
    use crate::utils::numeric_converter::{
        NumReprCfg,
        IntegerConverter,
    };

    #[test]
    fn test_level() {
        let cfg = NumReprCfg {
            digit_length: 7,
            decimal_point_length: 2,
            is_signed: true,
            total_length: 11,
        };

        let mut conevrter = IntegerConverter::new(cfg).unwrap();
        let price_str = "5000111.19";
        let bp = conevrter.to_i64(price_str);
        
        let order = BookOrder {
            order_id: 1,
            price: bp,
            quantity: 100,
            order_side: OrderSide::Buy,
        };

        let mut level = Level::initialize_with_order(order.clone());
        assert_eq!(level.order_count(), 1);
        assert_eq!(level.is_empty(), false);

        let order2 = BookOrder {
            order_id: 2,
            price: bp,
            quantity: 200,
            order_side: OrderSide::Buy,
        };

        level.add_order(order2.clone()).unwrap();
        assert_eq!(level.order_count(), 2);
        assert_eq!(level.is_empty(), false);


    }
}