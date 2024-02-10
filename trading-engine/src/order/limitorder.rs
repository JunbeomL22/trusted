use crate::base::conversions::{f64_to_fixed_i64, FIXED_PRECISION};
use crate::base::price::Price;
use crate::base::base_enum::Side;
use crate::order::orderstatus::OrderStatus;

pub struct LimitOrder {
    id: u64,
    price: Price,
    quantity: u64,
    side: Side,
    status: OrderStatus,
}