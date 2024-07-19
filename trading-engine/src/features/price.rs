use crate::utils::numeric_converter::OrderConverter;
use crate::data::krx::krx_converter::get_krx_base_bond_order_converter;
use crate::types::enums::TimeStampType;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub is_normalised: bool,
    pub utc: u8, // 0 ~ 24
    pub timestamp_type: TimeStampType,
    pub converter: &'static OrderConverter,
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            is_normalised: true,
            utc: 9, // Seoul
            timestamp_type: TimeStampType::default(),
            converter: get_krx_base_bond_order_converter(),
        }
    }
}
/*
/ These are keyed by (Venue, IsinCode)
#[derive(Debug, Clone)]
pub struct MidPrice { // 
    pub metadata: MetaData,
    pub date: u32, // YYYYMMDD
    pub values: Vec<TimeSeriesPoint>,
}

impl Default for MidPrice {
    fn default() -> Self {
        MidPrice {
            metadata: MetaData::default(),
            date: 19700101,
            values: Vec::new(),
        }
    }
}

impl MidPrice {
    pub fn with_capacity(n: usize) -> Self {
        MidPrice {
            metadata: MetaData::default(),
            date: 19700101,
            values: Vec::with_capacity(n),
        }
    }
}
 */
//