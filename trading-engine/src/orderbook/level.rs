use crate::data::book_order::BookOrder;
use crate::types::base::{BookPrice, OrderId, BookQuantity};
use crate::types::enums::OrderSide;
//
use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct Level {
    pub book_price: BookPrice,
    pub orders: VecDeque<(OrderId, BookQuantity)>,
}

impl Level {
    #[must_use]
    #[inline]
    pub fn initialize(book_price: BookPrice) -> Self {
        Level {
            book_price,
            orders: VecDeque::default(),
        }
    }

    #[must_use]
    #[inline]
    pub fn aggregate_quantity(&self) -> BookQuantity {
        self.orders.iter().map(|(_, q)| *q).sum()
    }
    
    #[must_use]
    #[inline]
    pub fn initialize_with_order(order: BookOrder) -> Self {
        let mut orders = VecDeque::default();
        orders.push_back((order.order_id, order.quantity));
        Level {
            book_price: order.price,
            orders,
        }
    }

    pub fn add_order(&mut self, order: BookOrder) -> Result<()> {
        if self.book_price != order.price {
            let lazy_error = || {
                anyhow!(
                    "Price mismatch\n
                level: {:?}\n
                input_order: {:?}",
                    self,
                    order,
                )
            };
            Err(lazy_error())
        } else {
            self.orders.push_back((order.order_id, order.quantity));
            Ok(())
        }
    }

    #[inline]
    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<(OrderId, BookQuantity)> {
        if let Some(index) = self.orders.iter().position(|&(k, _)| k == order_id) {
            self.orders.remove(index)
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn get_order(&self, order_id: OrderId, order_side: OrderSide) -> Option<BookOrder> {
        self.orders
            .iter()
            .find(|(k, _)| *k == order_id)
            .map(|(k, v)| BookOrder {
                order_id: *k,
                price: self.book_price,
                quantity: *v,
                order_side,
            })
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    #[must_use]
    #[inline]
    pub fn order_count(&self) -> usize {
        self.orders.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.orders.clear();
    }

    /// Returns the remaining quantity after trading
    pub fn trade(&mut self, quantity: BookQuantity) -> BookQuantity {
        let mut remaining = quantity;
        if self.orders.is_empty() {
            return remaining;
        }

        while remaining > 0 {
            if let Some((_, q)) = self.orders.front_mut() {
                if *q > remaining {
                    *q -= remaining;
                    remaining = 0;
                } else {
                    remaining -= *q;
                    self.orders.pop_front();
                }
            } else {
                break;
            }
        }
        remaining
    }

    pub fn change_quantity(&mut self, order_id: OrderId, quantity: BookQuantity) -> Option<()> {
        if let Some((_, q)) = self.orders.iter_mut().find(|(k, _)| *k == order_id) {
            *q = quantity;
            Some(())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::OrderSide;
    use crate::utils::numeric_converter::{IntegerConverter, NumReprCfg};

    #[test]
    fn test_level() -> Result<()> {
        let cfg = NumReprCfg {
            digit_length: 7,
            decimal_point_length: 3,
            drop_decimal_point: false,
            is_signed: true,
            unused_length: 0,
            total_length: 11,
            float_normalizer: None,
        };

        let conevrter = IntegerConverter::new(cfg).unwrap();
        let price_str = b"05000111.19";
        let bp = conevrter.to_i64(price_str);

        let order = BookOrder {
            order_id: 1,
            price: bp,
            quantity: 100,
            order_side: OrderSide::Bid,
        };

        let mut level = Level::initialize_with_order(order.clone());
        assert_eq!(level.order_count(), 1);
        assert_eq!(level.is_empty(), false);

        let order2 = BookOrder {
            order_id: 2,
            price: bp,
            quantity: 200,
            order_side: OrderSide::Bid,
        };

        level.add_order(order2.clone()).unwrap();
        assert_eq!(level.order_count(), 2);

        let order3 = BookOrder {
            order_id: 3,
            price: bp,
            quantity: 50,
            order_side: OrderSide::Bid,
        };

        level.add_order(order3.clone()).unwrap();
        assert_eq!(level.order_count(), 3);

        level.cancel_order(2);
        assert_eq!(level.order_count(), 2);

        let _ = level.trade(50);

        assert_eq!(level.order_count(), 2);
        assert_eq!(level.orders[0].1, 50);

        let _ = level.trade(70);
        assert_eq!(level.order_count(), 1);
        assert_eq!(level.orders[0].1, 30);

        let remaining = level.trade(50);
        assert_eq!(remaining, 20);

        let remaining = level.trade(30);
        assert_eq!(remaining, 30);

        Ok(())
    }
}
