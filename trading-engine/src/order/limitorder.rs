use crate::base::conversions::{f64_to_fixed_i64, FIXED_PRECISION};
use crate::base::io64::IoI64;
use crate::base::enums::OrderSide;
use crate::order::orderstatus::OrderStatus;

pub struct LimitOrder {
    id: u64,
    price: IoI64,
    quantity: IoI64,
    side: OrderSide,
    status: OrderStatus,
}