use crate::types::base::{
    Real,
    UnixNano,
};
use crate::utils::numeric_converter::OrderConverter; 
use crate::types::timestamp::TimeStampType;

pub struct MetaData {
    level: u8, // best or second best, etc
    data_timestamp_type: TimeStampType,
    calc_interval_in_micro: u64,
    order_converter: &'static OrderConverter,
}
pub struct OrderFlowImbalance {
    metadata: MetaData,
    engine_timestamp: UnixNano,
    data_timestamp: u64,
    tail_index: usize,
    head_index: usize,
    tick_flows: Vec<Real>,
    value: Real,
}