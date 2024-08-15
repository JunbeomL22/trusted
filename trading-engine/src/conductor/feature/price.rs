use crate::utils::numeric_converter::OrderConverter;
use crate::data::krx::krx_converter::get_krx_base_bond_order_converter;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub is_normalised: bool,
    pub utc: u8, // 0 ~ 24
    pub converter: &'static OrderConverter,
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            is_normalised: true,
            utc: 9, // Seoul
            converter: get_krx_base_bond_order_converter(),
        }
    }
}