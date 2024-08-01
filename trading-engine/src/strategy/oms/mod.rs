pub mod simple_taker;

use crate::types::timestamp::{
    TimeStamp,
    UnixNano,
};
use anyhow::Result;
pub trait OMS {
    fn cancel_order_by_timeout(&self, elapsed_time: UnixNano) -> Result<()>;
    //fn cancel_order(venue: Venue, order_id: OrderId) -> Result<()>{
    //    unimplemented!()
    //}
}