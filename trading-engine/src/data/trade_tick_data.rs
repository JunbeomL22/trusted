use crate::types::{
    isin_code::IsinCode,
    base::{
        BookPrice,
        BookQuantity,
        OrderCount,
        OrderData,
    },
    venue::Venue,
};
use serde::{Deserialize, Serialize};
use flexstr::LocalStr;
use smallvec::SmallVec;
    
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TradeType {
    Buy,
    #[default]
    Undefined,
    Sell,
}

// "B604F        G140KR41EYV1000900421009135225519800028350000028300000000002800000006500006000190002840000002825000000000810000001190001000018000284500000282000000000115000000113000130002400028500000028150000000008900000011600014000140002855000002810000000000710000000660002300016000286000000280500000000072000000117000180001300028650000028000000000005200000007900012000160002870000002795000000000460000000310001600009000287500000279000000000048000000066000130001000028800000027850000000003400000005600014000060000014850000011550033400305000000000"
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TradeTickData {
    data_code: LocalStr, // this will be sent to another thread anyway
    venue: Venue,
    isin_code: IsinCode, // this can be spread product
    data_timestamp: u64, // HHMMSSuuuuuu
    trade_price: BookPrice,
    trade_quantity: BookQuantity,
    prev_trade_price: Option<BookPrice>,
    trade_type: Option<TradeType>,
    // in case of spread product
    near_month_trade_price: Option<BookPrice>,
    far_month_trade_price: Option<BookPrice>,
    // not sure if we need these
    // accumulated_trade_quantity: BookQuantity,
    // accumulated_trade_volume: u64,
    // 
    ask_order_data: Vec<OrderData>,
    bid_order_data: Vec<OrderData>,
}
