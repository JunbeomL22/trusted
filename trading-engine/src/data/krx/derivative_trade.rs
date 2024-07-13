use crate::data::trade_quote::TradeQuoteData;
use crate::types::{
    base::{OrderBase, Slice},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};
use crate::utils::numeric_converter::{OrderConverter, TimeStampConverter};
use anyhow::{anyhow, Result};

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
/// ex) G703F        G140KR4301V13502001656104939081108000002.12000000005000000.00000000.00000002.83000002.93000002.06000002.11000000021511000000013250790000.0002000006.86000000.01000002.12000002.110000000100000000100000300006000002.13000002.100000000330000000410001100011000002.14000002.090000000290000000430000800010000002.15000002.080000000380000000370000900013000002.16000002.0700000001800000006200007000110000017960000059190049400380�"
#[derive(Debug, Clone)]
pub struct IFMSRPD0037 {
    order_converter: OrderConverter,
    timestamp_converter: TimeStampConverter,
    payload_length: usize,
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    trade_price_slice: Slice,
    trade_quantity_slice: Slice,
    //
    // near_month_trade_price_slice: Slice,
    //
    trade_type_slice: Slice,
    //
    quote_level: usize, // 5 given in this case
    quote_level_cut: usize, // <= self.quote_level, ex) 3 => only 3 levels parsed
    quote_start_index: usize, // 172 in this case
}

impl Default for IFMSRPD0037 {
    fn default() -> Self {
        IFMSRPD0037 {
            order_converter: OrderConverter::get_krx_derivative_converter(),
            timestamp_converter: OrderConverter::krx_timestamp_converter(),
            payload_length: 431,
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            trade_price_slice: Slice { start: 47, end: 56 },
            trade_quantity_slice: Slice { start: 56, end: 65 },
            //
            // near_month_trade_price_slice: Slice { start: 65, end: 74 },
            //
            trade_type_slice: Slice {
                start: 153,
                end: 154,
            },
            //
            quote_level: 5,
            quote_level_cut: 5,
            quote_start_index: 172,
            //
        }
    }
}

impl IFMSRPD0037 {
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Self {
        self.quote_level_cut = quote_level_cut;
        self
    }

    pub fn to_trade_quote_data(&self, payload: &[u8]) -> Result<TradeQuoteData> {
        if payload.len() != self.payload_length {
            let err = || {
                anyhow!(
                    "Invalid payload length: {}\n\
                message:\n\
                {:?}",
                    payload.len(),
                    std::str::from_utf8(payload),
                )
            };
            return Err(err());
        }

        if payload[self.payload_length - 1] != 255 {
            let err = || {
                anyhow!(
                    "Invalid end keyword: {}\n\
                message:\n\
                {:?}",
                    payload[self.payload_length - 1],
                    std::str::from_utf8(payload),
                )
            };
            return Err(err());
        }

        let converter = &self.order_converter;
        let timestamp_converter = &self.timestamp_converter;
        
        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = converter.order_count.get_config().total_length;
        //
        let venue = Venue::KRX;

        let isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        let timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };

        let trade_price = converter
            .to_book_price(&payload[self.trade_price_slice.start..self.trade_price_slice.end]);
        
        let trade_quantity = unsafe {
            converter.to_book_quantity_unchecked(&payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end])
        };

        let trade_type = match &payload[self.trade_type_slice.start..self.trade_type_slice.end] {
            b"0" => Some(TradeType::Undefined),
            b"1" => Some(TradeType::Sell),
            _ => Some(TradeType::Buy),
        };
        //

        let mut ask_quote_data = vec![OrderBase::default(); self.quote_level_cut];
        let mut bid_quote_data = vec![OrderBase::default(); self.quote_level_cut];

        // sell_price => buy_price => sell_quantity => buy_quantity => sell_order_count => buy_order_count
        unsafe {
            //for i in 0..(self.quote_level - 3 ) {
            let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
            if self.quote_level_cut >= 1 {
                let st_idx_marker = self.quote_start_index + offset * 0;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                ask_quote_data[0].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                bid_quote_data[0].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[0].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[0].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[0].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                bid_quote_data[0].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 2 {
                let st_idx_marker = self.quote_start_index + offset;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                ask_quote_data[1].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                bid_quote_data[1].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[1].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[1].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[1].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[1].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 3 {
                let st_idx_marker = self.quote_start_index + offset *2;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                ask_quote_data[2].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                bid_quote_data[2].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[2].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[2].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[2].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[2].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 4 {
                let st_idx_marker = self.quote_start_index + offset * 3;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                ask_quote_data[3].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                bid_quote_data[3].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[3].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[3].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[3].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[3].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 5 {
                let st_idx_marker = self.quote_start_index + offset * 3;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                ask_quote_data[4].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                bid_quote_data[4].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[4].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[4].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[4].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[4].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
        }
        Ok(TradeQuoteData {
            venue,
            isin_code,
            timestamp,
            trade_price,
            trade_quantity,
            trade_type,
            ask_quote_data,
            bid_quote_data,
            quote_level_cut: self.quote_level_cut,
        })
    }

    pub fn to_trade_quote_data_buffer(&self, payload: &[u8], data_buffer: &mut TradeQuoteData) -> Result<()> {
        if data_buffer.ask_quote_data.len() != self.quote_level_cut
            || data_buffer.bid_quote_data.len() != self.quote_level_cut {
            let err = || anyhow!(
                "ask_quote_data length is not equal to quote_level: {} != {} \n\
                This method is for low latency. Generality is not the purpose\n\
                TradeQuoteData buffer {:?}",
                data_buffer.ask_quote_data.len(),
                self.quote_level,
                data_buffer
            );
            return Err(err());
        }

        if payload.len() != self.payload_length {
            let err = || {
                anyhow!(
                    "Invalid payload length: {}\n\
                message:\n\
                {:?}",
                    payload.len(),
                    std::str::from_utf8(payload),
                )
            };
            return Err(err());
        }
    
        if payload[self.payload_length - 1] != 255 {
            let err = || {
                anyhow!(
                    "Invalid end keyword: {}\n\
                message:\n\
                {:?}",
                    payload[self.payload_length - 1],
                    std::str::from_utf8(payload),
                )
            };
            return Err(err());
        }
    
        let converter = &self.order_converter;
        let timestamp_converter = &self.timestamp_converter;
        
        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = converter.order_count.get_config().total_length;
        //
        data_buffer.venue = Venue::KRX;

        data_buffer.isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        data_buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };
    
        data_buffer.trade_price = converter.to_book_price(&payload[self.trade_price_slice.start..self.trade_price_slice.end]);
            
        data_buffer.trade_quantity = unsafe {
            converter.to_book_quantity_unchecked(&payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end])
        };
    
        data_buffer.trade_type = match &payload[self.trade_type_slice.start..self.trade_type_slice.end] {
            b"0" => Some(TradeType::Undefined),
            b"1" => Some(TradeType::Sell),
            _ => Some(TradeType::Buy),
        };
        //
    
        // sell_price => buy_price => sell_quantity => buy_quantity => sell_order_count => buy_order_count
        unsafe {
            //for i in 0..(self.quote_level - 3 ) {
            let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
            if self.quote_level_cut >= 1 {
                let st_idx_marker = self.quote_start_index + offset * 0;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                data_buffer.ask_quote_data[0].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                data_buffer.bid_quote_data[0].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[0].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[0].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[0].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                data_buffer.bid_quote_data[0].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 2 {
                let st_idx_marker = self.quote_start_index + offset;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                data_buffer.ask_quote_data[1].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                data_buffer.bid_quote_data[1].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[1].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[1].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[1].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                data_buffer.bid_quote_data[1].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 3 {
                let st_idx_marker = self.quote_start_index + offset *2;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                data_buffer.ask_quote_data[2].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                data_buffer.bid_quote_data[2].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[2].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[2].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[2].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                data_buffer.bid_quote_data[2].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 4 {
                let st_idx_marker = self.quote_start_index + offset * 3;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                data_buffer.ask_quote_data[3].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                data_buffer.bid_quote_data[3].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[3].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[3].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[3].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                data_buffer.bid_quote_data[3].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 5 {
                let st_idx_marker = self.quote_start_index + offset * 3;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker+offset];

                data_buffer.ask_quote_data[4].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln; 
                data_buffer.bid_quote_data[4].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[4].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1+ qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[4].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);
                
                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[4].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                data_buffer.bid_quote_data[4].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }

            Ok(())
        }

        
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let mut test_data_vec = b"G703F        G140KR4301V13502001656104939081108000002.12000000005000000.00000000.00000002.83000002.93000002.06000002.11000000021511000000013250790000.0002000006.86000000.01000002.12000002.110000000100000000100000300006000002.13000002.100000000330000000410001100011000002.14000002.090000000290000000430000800010000002.15000002.080000000380000000370000900013000002.16000002.0700000001800000006200007000110000017960000059190049400380".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0037 = IFMSRPD0037::default();

        let trade_quote_data = ifmsrpd0037
            .to_trade_quote_data(test_data)
            .expect("failed to convert to TradeQuoteData");
        println!(
            "\n* G703F parsing for isin code: {:?}\n",
            trade_quote_data.isin_code.as_str()
        );
        dbg!(trade_quote_data);
        assert_eq!(1, 1);

        Ok(())
    }

    #[test]
    fn test_parse_with_buffer() -> Result<()> {
        let mut test_data_vec = b"G703F        G140KR4301V13502001656104939081108000002.12000000005000000.00000000.00000002.83000002.93000002.06000002.11000000021511000000013250790000.0002000006.86000000.01000002.12000002.110000000100000000100000300006000002.13000002.100000000330000000410001100011000002.14000002.090000000290000000430000800010000002.15000002.080000000380000000370000900013000002.16000002.0700000001800000006200007000110000017960000059190049400380".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0037 = IFMSRPD0037::default().with_quote_level_cut(4);

        let mut trade_quote_data = TradeQuoteData::with_quote_level(4);
        ifmsrpd0037
            .to_trade_quote_data_buffer(test_data, &mut trade_quote_data)
            .expect("failed to convert to TradeQuoteData");
        println!(
            "\n* G703F parsing for isin code: {:?}\n",
            trade_quote_data.isin_code.as_str()
        );
        dbg!(trade_quote_data);
        assert_eq!(1, 1);

        Ok(())
    }
}
