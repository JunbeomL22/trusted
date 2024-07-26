pub mod order;
pub mod krx;
pub mod quote;
pub mod trade;
pub mod trade_quote;
pub mod checker;

use crate::data::krx::krx_converter::get_krx_base_bond_order_converter;
use crate::utils::numeric_converter::OrderConverter;

#[inline]
#[must_use]
pub fn get_default_order_converter() -> &'static OrderConverter {
    get_krx_base_bond_order_converter()
}