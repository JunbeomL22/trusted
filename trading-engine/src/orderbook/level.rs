use crate::data::order::LimitOrder;
use crate::types::base::{BookPrice, OrderId, BookQuantity};
use crate::types::enums::OrderSide;
//
use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::fmt::Debug;

/// The VecDeque can be optimized by splitting the VecDeque into multiple VecDeque with a fixed size
#[derive(Debug, Clone, Default)]
pub struct Level {
    pub book_price: BookPrice,
    pub orders: VecDeque<(OrderId, BookQuantity)>,
    pub total_quantity: BookQuantity,
}

impl Level {
    #[must_use]
    #[inline]
    pub fn initialize(book_price: BookPrice) -> Self {
        Level {
            book_price,
            orders: VecDeque::default(),
            total_quantity: 0,
        }
    }

    #[must_use]
    #[inline]
    pub fn aggregate_quantity(&self) -> BookQuantity {
        self.total_quantity
    }
    
    #[must_use]
    #[inline]
    pub fn initialize_with_order(order: LimitOrder) -> Self {
        let mut orders = VecDeque::default();
        orders.push_back((order.order_id, order.quantity));
        Level {
            book_price: order.price,
            orders,
            total_quantity: order.quantity,
        }
    }

    pub fn add_limit_order(&mut self, order: LimitOrder) -> Result<()> {
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
            self.total_quantity += order.quantity;
            Ok(())
        }
    }

    /// Returns Option<(removed quantity, total quantity)>
    /// None means the order is not found
    #[inline]
    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<(BookQuantity, BookQuantity)> {
        match self.orders.iter().position(|(k, _)| *k == order_id) {
            Some(idx) => {
                let (_oid, q) = self.orders.remove(idx).unwrap();
                self.total_quantity -= q;
                Some((q, self.total_quantity))
            }
            None => None,
        }
    }

    #[must_use]
    #[inline]
    pub fn get_order(&self, order_id: OrderId, order_side: OrderSide) -> Option<LimitOrder> {
        self.orders
            .iter()
            .find(|(k, _)| *k == order_id)
            .map(|(k, v)| LimitOrder {
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

    /// Returns (all_traded_id, remaining quantity, total quantity remained in the level)
    /// all_traded orderid is returned so that it is removed from half_book.cache
    pub fn trade(&mut self, quantity: BookQuantity) -> (Vec<OrderId>, BookQuantity, BookQuantity) {
        let mut remaining = quantity;
        let mut traded_ids = Vec::new();
        if self.orders.is_empty() {
            return (traded_ids, remaining, self.total_quantity);
        }

        while remaining > 0 {
            if let Some((id, q)) = self.orders.front_mut() {
                if *q > remaining {
                    *q -= remaining;
                    self.total_quantity -= remaining;
                    remaining = 0;
                } else {
                    remaining -= *q;
                    self.total_quantity -= *q;
                    traded_ids.push(*id);
                    self.orders.pop_front();
                }
            } else {
                break;
            }
        }
        
        (traded_ids, remaining, self.total_quantity)
    }

    /// Returns Option<original quantity>
    #[must_use]
    #[inline]
    pub fn change_quantity(&mut self, order_id: OrderId, new_quantity: BookQuantity) -> Option<BookQuantity> {
        if let Some((_, q)) = self.orders.iter_mut().find(|(k, _)| *k == order_id) {
            let res = *q;
            *q = new_quantity;
            self.total_quantity += new_quantity
                .saturating_sub(res);
            Some(res)
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

        let order = LimitOrder {
            order_id: 1,
            price: bp,
            quantity: 100,
            order_side: OrderSide::Bid,
        };

        let mut level = Level::initialize_with_order(order.clone());
        assert_eq!(level.order_count(), 1);
        assert_eq!(level.is_empty(), false);

        let order2 = LimitOrder {
            order_id: 2,
            price: bp,
            quantity: 200,
            order_side: OrderSide::Bid,
        };

        level.add_limit_order(order2.clone()).unwrap();
        assert_eq!(level.order_count(), 2);

        let order3 = LimitOrder {
            order_id: 3,
            price: bp,
            quantity: 50,
            order_side: OrderSide::Bid,
        };

        level.add_limit_order(order3.clone()).unwrap();
        assert_eq!(level.order_count(), 3);
        assert_eq!(level.aggregate_quantity(), 350);

        level.cancel_order(2);
        assert_eq!(level.order_count(), 2);
        assert_eq!(level.aggregate_quantity(), 150);

        let _ = level.trade(50);

        assert_eq!(level.order_count(), 2);
        assert_eq!(level.orders[0].1, 50);

        let _ = level.trade(70);
        assert_eq!(level.order_count(), 1);
        assert_eq!(level.orders[0].1, 30);

        assert_eq!(level.aggregate_quantity(), 30);

        let (_ids, remaining, total) = level.trade(10);
        assert_eq!(remaining, 0);
        assert_eq!(total, 20);

        let (_ids, remaining, total) = level.trade(30);
        assert_eq!(remaining, 10);
        assert_eq!(total, 0);

        Ok(())
    }
}
