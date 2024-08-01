use crate::{
    Real,
    TimeStamp,
    UnixNano,
};
use crate::utils::numeric_converter::OrderConverter; 


pub struct MetaData {
    level: u8, // best or second best, etc
    duration: UnixNano,
    order_converter: &'static OrderConverter,
}
pub struct OrderFlowImbalance {
    metadata: MetaData,
    system_timestamp: TimeStamp,
    data_timestamp: u64,
    tail_index: usize,
    head_index: usize,
    tick_flows: Vec<Real>,
    value: Real,
}