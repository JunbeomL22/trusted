pub mod simple_taker;

use crate::types::base::MilliTimeStamp;
use crate::types::venue::Venue;
use anyhow::Result;
pub trait OMS {
    fn cancel_order_by_timeout(&self, timestamp: MilliTimeStamp);
    //fn cancel_order(venue: Venue, order_id: OrderId) -> Result<()>{
    //    unimplemented!()
    //}
}