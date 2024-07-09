use crate::data::trade_quote_data::TradeQuoteData;
use crate::types::{
    venue::Venue,
    base::{
        Slice,
        BookPrice,
        BookQuantity,
        OrderCount,
        OrderData,
    },
    isin_code::IsinCode,
    enums::TradeType,
};
use crate::data::krx::krx_converter::{
    KRX_DERIVATIVE_CONVERTER,
    KRX_TIMESTAMP_CONVERTER,
};
use anyhow::Result;
use std::str::from_utf8_unchecked;

/// Message Structure:
/// (Derivatives) trade + best 5 level Bid/Ask
/// +----------------------------------+------------+--------+---------------+
/// | Item Name                        | Data Type  | Length | Accum Length  |
/// +----------------------------------+------------+--------+---------------+
/// | Data Category                    | String     | 2      | 2             |
/// | Information Category             | String     | 3      | 5             |
/// | Message sequence number          | Int        | 8      | 13            |
/// | Board ID                         | String     | 2      | 15            |
/// | Session ID                       | String     | 2      | 17            |
/// | ISIN Code                        | String     | 12     | 29            |
/// | A designated number for an issue | Int        | 6      | 35            |
/// | Processing Time of Trading System| String     | 12     | 47            |
/// | Trading Price                    | Double     | 9      | 56            |
/// | Trading volume                   | Int        | 9      | 65            |
/// | Nearby Month Contract Price      | Double     | 9      | 74            |
/// | Distant Month Contract Price     | Double     | 9      | 83            |
/// | Opening Price                    | Double     | 9      | 92            |
/// | Today's High                     | Double     | 9      | 101           |
/// | Today's Low                      | Double     | 9      | 110           |
/// | Previous price                   | Double     | 9      | 119           |
/// | Accumulated Trading Volume       | Long       | 12     | 131           |
/// | Accumulated Trading value        | FLOAT128   | 22     | 153           |
/// | Final Ask/Bid Type Code          | String     | 1      | 154           |
/// | UpperLimit of Dynamic PriceRange | Double     | 9      | 163           |
/// | LowerLimit of Dynamic PriceRange | Double     | 9      | 172           |
/// | Ask Level 1 price                | Double     | 9      | 181           |
/// | Bid Level 1 price                | Double     | 9      | 190           |
/// | Ask Level 1 volume               | Int        | 9      | 199           |
/// | Bid Level 1 volume               | Int        | 9      | 208           |
/// | Ask Level 1_Order Counts         | Int        | 5      | 213           |
/// | Bid Level 1_Order Counts         | Int        | 5      | 218           |
/// | Ask Level 2 price                | Double     | 9      | 227           |
/// | Bid Level 2 price                | Double     | 9      | 236           |
/// | Ask Level 2 volume               | Int        | 9      | 245           |
/// | Bid Level 2 volume               | Int        | 9      | 254           |
/// | Ask Level 2_Order Counts         | Int        | 5      | 259           |
/// | Bid Level 2_Order Counts         | Int        | 5      | 264           |
/// | Ask Level 3 price                | Double     | 9      | 273           |
/// | Bid Level 3 price                | Double     | 9      | 282           |
/// | Ask Level 3 volume               | Int        | 9      | 291           |
/// | Bid Level 3 volume               | Int        | 9      | 300           |
/// | Ask Level 3_Order Counts         | Int        | 5      | 305           |
/// | Bid Level 3_Order Counts         | Int        | 5      | 310           |
/// | Ask Level 4 price                | Double     | 9      | 319           |
/// | Bid Level 4 price                | Double     | 9      | 328           |
/// | Ask Level 4 volume               | Int        | 9      | 337           |
/// | Bid Level 4 volume               | Int        | 9      | 346           |
/// | Ask Level 4_Order Counts         | Int        | 5      | 351           |
/// | Bid Level 4_Order Counts         | Int        | 5      | 356           |
/// | Ask Level 5 price                | Double     | 9      | 365           |
/// | Bid Level 5 price                | Double     | 9      | 374           |
/// | Ask Level 5 volume               | Int        | 9      | 383           |
/// | Bid Level 5 volume               | Int        | 9      | 392           |
/// | Ask Level 5_Order Counts         | Int        | 5      | 397           |
/// | Bid Level 5_Order Counts         | Int        | 5      | 402           |
/// | Ask Total Volume                 | Int        | 9      | 411           |
/// | Bid Total Volume                 | Int        | 9      | 420           |
/// | Ask Price_Valid Counts           | Int        | 5      | 425           |
/// | Bid Price_Valid Counts           | Int        | 5      | 430           |
/// | End Keyword                      | String     | 1      | 431           |
/// +----------------------------------+------------+--------+---------------+
#[derive(Debug, Clone)]
pub struct IFMSRPD0037 {
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    trade_price_slice: Slice,
    trade_quantity_slice: Slice,
    //
    near_month_trade_price_slice: Slice,
    //
    trade_type_slice: Slice,
    //
    ask_price_slice_vec: Vec<Slice>,
    bid_price_slice_vec: Vec<Slice>,
    //
    ask_quantity_slice_vec: Vec<Slice>,
    bid_quantity_slice_vec: Vec<Slice>,
    //
    ask_order_count_slice_vec: Vec<Slice>,
    bid_order_count_slice_vec: Vec<Slice>,
    //
}

impl Default for IFMSRPD0037 {
    fn default() -> Self {
        IFMSRPD0037 {
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            trade_price_slice: Slice { start: 47, end: 56 },
            trade_quantity_slice: Slice { start: 56, end: 65 },
            //
            near_month_trade_price_slice: Slice { start: 65, end: 74 },
            //
            trade_type_slice: Slice { start: 153, end: 154 },
            //
            ask_price_slice_vec: vec![
                Slice { start: 172, end: 181 },
                Slice { start: 218, end: 227 },
                Slice { start: 264, end: 273 },
                Slice { start: 310, end: 319 },
                Slice { start: 356, end: 365 },
            ],
            bid_price_slice_vec: vec![
                Slice { start: 181, end: 190 },
                Slice { start: 227, end: 236 },
                Slice { start: 273, end: 282 },
                Slice { start: 319, end: 328 },
                Slice { start: 365, end: 374 },
            ],
            //
            ask_quantity_slice_vec: vec![
                Slice { start: 190, end: 199 },
                Slice { start: 236, end: 245 },
                Slice { start: 282, end: 291 },
                Slice { start: 328, end: 337 },
                Slice { start: 374, end: 383 },
            ],
            bid_quantity_slice_vec: vec![
                Slice { start: 199, end: 208 },
                Slice { start: 245, end: 254 },
                Slice { start: 291, end: 300 },
                Slice { start: 337, end: 346 },
                Slice { start: 383, end: 392 },
            ],
            //
            ask_order_count_slice_vec: vec![
                Slice { start: 208, end: 213 },
                Slice { start: 254, end: 259 },
                Slice { start: 300, end: 305 },
                Slice { start: 346, end: 351 },
                Slice { start: 392, end: 397 },
            ],
            bid_order_count_slice_vec: vec![
                Slice { start: 213, end: 218 },
                Slice { start: 259, end: 264 },
                Slice { start: 305, end: 310 },
                Slice { start: 351, end: 356 },
                Slice { start: 397, end: 402 },
            ],
        }
    }
}

/*
impl IFMSRPD0037 {
    pub fn to_trade_quote_date(&self, payload: &[u8]) -> TradeQuoteData {
        let converter = &KRX_DERIVATIVE_CONVERTER;
        let timestamp_converter = &KRX_TIMESTAMP_CONVERTER;
        
        //
        let venue = Venue::KRX;

        let isin_code = IsinCode { isin: payload[self.isin_code_slice.start..self.isin_code_slice.end] };

        let timestemp = timestamp_converter.to_timestamp(payload[self.timestamp_slice.start..self.timestamp_slice.end]);

        let trade_price = converter.to_price(payload[self.trade_price_slice.start..self.trade_price_slice.end]);

        let trade_quantity = converter.to_quantity(&payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end]);
        trade_quote_data.near_month_trade_price = converter.to_price(&payload[ifmsrpd0037.near_month_trade_price_slice.start..ifmsrpd0037.near_month_trade_price_slice.end]);
        trade_quote_data.trade_type = match payload[ifmsrpd0037.trade_type_slice.start] {
            b'2' => Some(TradeType::Buy),
            b'1' => Some(TradeType::Sell),
            _ => Some(TradeType::Undefined),
        };
        //
        for i in 0..5 {
            trade_quote_data.ask_order_data.push(OrderData {
                order_count: converter.to_order_count(&payload[ifmsrpd0037.ask_order_count_slice_vec[i].start..ifmsrpd0037.ask_order_count_slice_vec[i].end]),
                book_price: converter.to_price(&payload[ifmsrpd0037.ask_price_slice_vec[i].start..ifmsrpd0037.ask_price_slice_vec[i].end]),
                book_quantity: converter.to_quantity(&payload[ifmsrpd0037.ask_quantity_slice_vec[i].start..ifmsrpd0037.ask_quantity_slice_vec[i].end]),
            });
            trade_quote_data.bid_order_data.push(OrderData {
                order_count: converter.to_order_count(&payload[ifmsrpd0037.bid_order_count_slice_vec[i].start..ifmsr
    }
}
 */