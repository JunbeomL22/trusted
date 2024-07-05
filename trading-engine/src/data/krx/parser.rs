use crate::utils::numeric_converter::{
    NumReprCfg,
    IntegerConverter,
};


#[derive(Debug, Clone)]
pub struct KrxParser {
    // index, bond, etc
    base_derivatives_price_converter: IntegerConverter,
    base_derivatives_quantity_converter: IntegerConverter,
    base_derivatives_order_count_converter: IntegerConverter,
    //
    stock_price_converter: IntegerConverter,
    stock_quantity_converter: IntegerConverter,
    stock_order_count_converter: IntegerConverter,
    //
    stock_derivative_price_converter: IntegerConverter,
    stock_derivative_quantity_converter: IntegerConverter,
    stock_derivative_order_count_converter: IntegerConverter,
    //
    bond_price_converter: IntegerConverter,
    bond_quantity_converter: IntegerConverter,
    bond_order_count_converter: IntegerConverter,
    bond_yield_converter: IntegerConverter,
    //
    repo_price_converter: IntegerConverter,
    repo_quantity_converter: IntegerConverter,
    repo_order_count_converter: IntegerConverter,
    repo_yield_converter: IntegerConverter,
    //
    fx_futures_price_converter: IntegerConverter,
    fx_futures_quantity_converter: IntegerConverter,
    fx_futures_order_count_converter: IntegerConverter,
    // 
}
