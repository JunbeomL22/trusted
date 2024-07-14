use crate::data::trade_quote::TradeQuoteSnapshot;
use crate::types::{
    base::{LevelSnapshot, Slice},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};
use crate::utils::numeric_converter::{OrderConverter, TimeStampConverter};
use anyhow::{anyhow, Result};

/// Message Structure:
/// 파생 체결 + 우선호가 (우선호가 5단계)
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
    //
    payload_length: usize,
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    trade_price_slice: Slice,
    trade_quantity_slice: Slice,
    //
    trade_type_slice: Slice,
    //
    quote_level: usize,       // 5 given in this case
    quote_level_cut: usize,   // <= self.quote_level, ex) 3 => only 3 levels parsed
    quote_start_index: usize, // 172 in this case
}

impl Default for IFMSRPD0037 {
    fn default() -> Self {
        IFMSRPD0037 {
            order_converter: OrderConverter::krx_derivative_converter(),
            timestamp_converter: TimeStampConverter::krx_timestamp_converter(),
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
    #[inline]
    pub fn get_order_converter(&self) -> &OrderConverter {
        &self.order_converter
    }

    #[inline]
    pub fn get_timestamp_converter(&self) -> &TimeStampConverter {
        &self.timestamp_converter
    }

    #[inline]
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Self {
        self.quote_level_cut = quote_level_cut;
        self
    }

    pub fn to_trade_quote_snapshot(&self, payload: &[u8]) -> Result<TradeQuoteSnapshot> {
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

        let isin_code =
            IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        let timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(
                &payload[self.timestamp_slice.start..self.timestamp_slice.end],
            )
        };

        let trade_price = converter
            .to_book_price(&payload[self.trade_price_slice.start..self.trade_price_slice.end]);

        let trade_quantity = unsafe {
            converter.to_book_quantity_unchecked(
                &payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end],
            )
        };

        let trade_type = match &payload[self.trade_type_slice.start..self.trade_type_slice.end] {
            b"0" => Some(TradeType::Undefined),
            b"1" => Some(TradeType::Sell),
            _ => Some(TradeType::Buy),
        };
        //

        let mut ask_quote_data = vec![LevelSnapshot::default(); self.quote_level_cut];
        let mut bid_quote_data = vec![LevelSnapshot::default(); self.quote_level_cut];

        // sell_price => buy_price => sell_quantity => buy_quantity => sell_order_count => buy_order_count
        unsafe {
            //for i in 0..(self.quote_level - 3 ) {
            let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
            if self.quote_level_cut >= 1 {
                let st_idx_marker = self.quote_start_index;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[0].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[0].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[0].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[0].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[0].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                bid_quote_data[0].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 2 {
                let st_idx_marker = self.quote_start_index + offset;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[1].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[1].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[1].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[1].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[1].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[1].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 3 {
                let st_idx_marker = self.quote_start_index + offset * 2;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[2].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[2].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[2].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[2].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[2].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[2].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 4 {
                let st_idx_marker = self.quote_start_index + offset * 3;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[3].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[3].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[3].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[3].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[3].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[3].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 5 {
                let st_idx_marker = self.quote_start_index + offset * 4;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[4].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[4].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[4].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[4].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[4].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                bid_quote_data[4].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
        }
        Ok(TradeQuoteSnapshot {
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

    pub fn to_trade_quote_snapshot_buffer(
        &self,
        payload: &[u8],
        data_buffer: &mut TradeQuoteSnapshot,
    ) -> Result<()> {
        if data_buffer.ask_quote_data.len() < self.quote_level_cut
            || data_buffer.bid_quote_data.len() < self.quote_level_cut
        {
            let err = || {
                anyhow!(
                    "ask_quote_data length is not equal to quote_level: {} != {} \n\
                This method is for low latency. Generality is not the purpose\n\
                TradeQuoteSnapshot buffer {:?}",
                    data_buffer.ask_quote_data.len(),
                    self.quote_level,
                    data_buffer
                )
            };
            return Err(err());
        }

        if payload.len() != self.payload_length {
            let err = || {
                anyhow!(
                    "Invalid payload length: {}\n\
                message:\n\
                {:?}",
                    payload.len(),
                    std::str::from_utf8(&payload[..(payload.len()-1)]),
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
                    std::str::from_utf8(&payload[..(payload.len()-1)]),
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

        data_buffer.isin_code =
            IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        data_buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(
                &payload[self.timestamp_slice.start..self.timestamp_slice.end],
            )
        };

        data_buffer.trade_price = converter
            .to_book_price(&payload[self.trade_price_slice.start..self.trade_price_slice.end]);

        data_buffer.trade_quantity = unsafe {
            converter.to_book_quantity_unchecked(
                &payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end],
            )
        };

        data_buffer.trade_type =
            match &payload[self.trade_type_slice.start..self.trade_type_slice.end] {
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
                let st_idx_marker = self.quote_start_index;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                data_buffer.ask_quote_data[0].book_price =
                    converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                data_buffer.bid_quote_data[0].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[0].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[0].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[0].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                data_buffer.bid_quote_data[0].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 2 {
                let st_idx_marker = self.quote_start_index + offset;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                data_buffer.ask_quote_data[1].book_price =
                    converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                data_buffer.bid_quote_data[1].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[1].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[1].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[1].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                data_buffer.bid_quote_data[1].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 3 {
                let st_idx_marker = self.quote_start_index + offset * 2;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                data_buffer.ask_quote_data[2].book_price =
                    converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                data_buffer.bid_quote_data[2].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[2].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[2].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[2].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                data_buffer.bid_quote_data[2].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 4 {
                let st_idx_marker = self.quote_start_index + offset * 3;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                data_buffer.ask_quote_data[3].book_price =
                    converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                data_buffer.bid_quote_data[3].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[3].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[3].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[3].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;

                data_buffer.bid_quote_data[3].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
            //
            //
            if self.quote_level_cut >= 5 {
                let st_idx_marker = self.quote_start_index + offset * 4;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                data_buffer.ask_quote_data[4].book_price =
                    converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                data_buffer.bid_quote_data[4].book_price =
                    converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[4].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[4].book_quantity = converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[4].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                data_buffer.bid_quote_data[4].order_count = converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }

            Ok(())
        }
    }
}

/// Message Structure:
/// IFMSRPD0038
/// Derivatives_Order Filled + Quote (Ten levels of Quotes)
/// 파생 체결 + 우선호가 (우선호가 10단계)
/// | Item Name                           | Length | Accum Length |
/// |-------------------------------------|--------|--------------|
/// | Data Category                       |      2 |            2 |
/// | Information Category                |      3 |            5 |
/// | Message sequence number             |      8 |           13 |
/// | Board ID                            |      2 |           15 |
/// | Session ID                          |      2 |           17 |
/// | ISIN Code                           |     12 |           29 |
/// | A designated number for an issue    |      6 |           35 |
/// | Processing Time of Trading System   |     12 |           47 |
/// | Trading Price                       |      9 |           56 |
/// | Trading volume                      |      9 |           65 |
/// | Nearby Month Contract_Trading Price |      9 |           74 |
/// | Distant Month Contract_Trading Price|      9 |           83 |
/// | Opening Price                       |      9 |           92 |
/// | Today's High                        |      9 |          101 |
/// | Today's Low                         |      9 |          110 |
/// | Previous price                      |      9 |          119 |
/// | Accumulated Trading Volume          |     12 |          131 |
/// | Accumulated Trading value           |     22 |          153 |
/// | Final Ask/Bid Type Code             |      1 |          154 |
/// | Upper Limit of Dynamic Price Range  |      9 |          163 |
/// | Lower Limit of Dynamic Price Range  |      9 |          172 |
/// | Ask Level 1 price                   |      9 |          181 |
/// | Bid Level 1 price                   |      9 |          190 |
/// | Ask Level 1 volume                  |      9 |          199 |
/// | Bid Level 1 volume                  |      9 |          208 |
/// | Ask Level 1_Order Counts            |      5 |          213 |
/// | Bid Level 1_Order Counts            |      5 |          218 |
/// | Ask Level 2 price                   |      9 |          227 |
/// | Bid Level 2 price                   |      9 |          236 |
/// | Ask Level 2 volume                  |      9 |          245 |
/// | Bid Level 2 volume                  |      9 |          254 |
/// | Ask Level 2_Order Counts            |      5 |          259 |
/// | Bid Level 2_Order Counts            |      5 |          264 |
/// | Ask Level 3 price                   |      9 |          273 |
/// | Bid Level 3 price                   |      9 |          282 |
/// | Ask Level 3 volume                  |      9 |          291 |
/// | Bid Level 3 volume                  |      9 |          300 |
/// | Ask Level 3_Order Counts            |      5 |          305 |
/// | Bid Level 3_Order Counts            |      5 |          310 |
/// | Ask Level 4 price                   |      9 |          319 |
/// | Bid Level 4 price                   |      9 |          328 |
/// | Ask Level 4 volume                  |      9 |          337 |
/// | Bid Level 4 volume                  |      9 |          346 |
/// | Ask Level 4_Order Counts            |      5 |          351 |
/// | Bid Level 4_Order Counts            |      5 |          356 |
/// | Ask Level 5 price                   |      9 |          365 |
/// | Bid Level 5 price                   |      9 |          374 |
/// | Ask Level 5 volume                  |      9 |          383 |
/// | Bid Level 5 volume                  |      9 |          392 |
/// | Ask Level 5_Order Counts            |      5 |          397 |
/// | Bid Level 5_Order Counts            |      5 |          402 |
/// | Ask Level 6 price                   |      9 |          411 |
/// | Bid Level 6 price                   |      9 |          420 |
/// | Ask Level 6 volume                  |      9 |          429 |
/// | Bid Level 6 volume                  |      9 |          438 |
/// | Ask Level 6_Order Counts            |      5 |          443 |
/// | Bid Level 6_Order Counts            |      5 |          448 |
/// | Ask Level 7 price                   |      9 |          457 |
/// | Bid Level 7 price                   |      9 |          466 |
/// | Ask Level 7 volume                  |      9 |          475 |
/// | Bid Level 7 volume                  |      9 |          484 |
/// | Ask Level 7_Order Counts            |      5 |          489 |
/// | Bid Level 7_Order Counts            |      5 |          494 |
/// | Ask Level 8 price                   |      9 |          503 |
/// | Bid Level 8 price                   |      9 |          512 |
/// | Ask Level 8 volume                  |      9 |          521 |
/// | Bid Level 8 volume                  |      9 |          530 |
/// | Ask Level 8_Order Counts            |      5 |          535 |
/// | Bid Level 8_Order Counts            |      5 |          540 |
/// | Ask Level 9 price                   |      9 |          549 |
/// | Bid Level 9 price                   |      9 |          558 |
/// | Ask Level 9 volume                  |      9 |          567 |
/// | Bid Level 9 volume                  |      9 |          576 |
/// | Ask Level 9_Order Counts            |      5 |          581 |
/// | Bid Level 9_Order Counts            |      5 |          586 |
/// | Ask Level 10 price                  |      9 |          595 |
/// | Bid Level 10 price                  |      9 |          604 |
/// | Ask Level 10 volume                 |      9 |          613 |
/// | Bid Level 10 volume                 |      9 |          622 |
/// | Ask Level 10_Order Counts           |      5 |          627 |
/// | Bid Level 10_Order Counts           |      5 |          632 |
/// | Ask Total Volume                    |      9 |          641 |
/// | Bid Total Volume                    |      9 |          650 |
/// | Ask Price_Valid Counts              |      5 |          655 |
/// | Bid Price_Valid Counts              |      5 |          660 |
/// | End Keyword                         |      1 |          661 |

#[derive(Debug, Clone)]
pub struct IFMSRPD0038 {
    order_converter: OrderConverter,
    timestamp_converter: TimeStampConverter,
    //
    payload_length: usize,
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    trade_price_slice: Slice,
    trade_quantity_slice: Slice,
    trade_type_slice: Slice,
    //
    quote_level: usize,       // 10 given in this case
    quote_level_cut: usize,   // <= self.quote_level, ex) 3 => only 3 levels parsed
    quote_start_index: usize, // 181 in this case
}

impl Default for IFMSRPD0038 {
    fn default() -> Self {
        IFMSRPD0038 {
            order_converter: OrderConverter::krx_stock_converter(),
            timestamp_converter: TimeStampConverter::krx_timestamp_converter(),
            payload_length: 661,
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            trade_price_slice: Slice { 
                start: 47, 
                end: 56 },
            trade_quantity_slice: Slice {
                start: 56, 
                end: 65 },
            trade_type_slice: Slice {
                start: 153,
                end: 154,
            },
            //
            quote_level: 10,
            quote_level_cut: 10,
            quote_start_index: 172,
        }
    }
}

impl IFMSRPD0038 {
    pub fn get_order_converter(&self) -> &OrderConverter {
        &self.order_converter
    }

    pub fn get_timestamp_converter(&self) -> &TimeStampConverter {
        &self.timestamp_converter
    }

    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Self {
        self.quote_level_cut = quote_level_cut;
        self
    }

    pub fn to_trade_quote_snapshot(&self, payload: &[u8]) -> Result<TradeQuoteSnapshot> {
        if payload.len() != self.payload_length {
            let err = || {
                anyhow!(
                    "Invalid payload length: {}\n\
                message:\n\
                {:?}",
                    payload.len(),
                    std::str::from_utf8(&payload[..(payload.len()-1)]).unwrap(),
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
                    std::str::from_utf8(&payload[..(payload.len()-1)]).unwrap(),
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

        let isin_code =
            IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        let timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };

        let trade_price = unsafe { 
            converter.to_book_price_unchecked(&payload[self.trade_price_slice.start..self.trade_price_slice.end])
        };

        let trade_quantity = unsafe {
            converter.to_book_quantity_unchecked(&payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end])
        };

        let trade_type = match &payload[self.trade_type_slice.start..self.trade_type_slice.end] {
            b"0" => Some(TradeType::Undefined),
            b"1" => Some(TradeType::Sell),
            _ => Some(TradeType::Buy),
        };
        //

        let mut ask_quote_data = vec![LevelSnapshot::default(); self.quote_level_cut];
        let mut bid_quote_data = vec![LevelSnapshot::default(); self.quote_level_cut];
        // sell_price => buy_price => sell_quantity => buy_quantity => sell_order_count => buy_order_count
        unsafe{
            for i in 0..self.quote_level_cut {
                let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
                let st_idx_marker = self.quote_start_index + offset * i;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[i].book_price = converter.to_book_price_unchecked(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[i].book_price = converter.to_book_price_unchecked(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                bid_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
        }

        Ok(TradeQuoteSnapshot {
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

    pub fn to_trade_quote_snapshot_buffer(
        &self,
        payload: &[u8],
        data_buffer: &mut TradeQuoteSnapshot,
    ) -> Result<()> {
        if data_buffer.ask_quote_data.len() < self.quote_level_cut
            || data_buffer.bid_quote_data.len() < self.quote_level_cut
        {
            let err = || {
                anyhow!(
                    "ask_quote_data length is not equal to quote_level: {} != {} \n\
                This method is for low latency. Generality is not the purpose\n\
                TradeQuoteSnapshot buffer {:?}",
                    data_buffer.ask_quote_data.len(),
                    self.quote_level,
                    data_buffer
                )
            };
            return Err(err());
        }

        if payload.len() != self.payload_length {
            let err = || {
                anyhow!(
                    "Invalid payload length: {}\n\
                message:\n\
                {:?}",
                    payload.len(),
                    std::str::from_utf8(&payload[..(payload.len()-1)]).unwrap(),
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

        data_buffer.isin_code =
            IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        data_buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(
                &payload[self.timestamp_slice.start..self.timestamp_slice.end],
            )
        };

        data_buffer.trade_price = converter
            .to_book_price(&payload[self.trade_price_slice.start..self.trade_price_slice.end]);

        data_buffer.trade_quantity = unsafe {
            converter.to_book_quantity_unchecked(
                &payload[self.trade_quantity_slice.start..self.trade_quantity_slice.end],
            )
        };

        data_buffer.trade_type = match &payload[self.trade_type_slice.start..self.trade_type_slice.end] {
            b"0" => Some(TradeType::Undefined),
            b"1" => Some(TradeType::Sell),
            _ => Some(TradeType::Buy),
        };

        // sell_price => buy_price => sell_quantity => buy_quantity => sell_order_count => buy_order_count
        unsafe {
            for i in 0..self.quote_level_cut {
                let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
                let st_idx_marker = self.quote_start_index + offset * i;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                data_buffer.ask_quote_data[i].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                data_buffer.bid_quote_data[i].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                data_buffer.ask_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                data_buffer.bid_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                data_buffer.ask_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                data_buffer.bid_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
        }

        Ok(())
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
            .to_trade_quote_snapshot(test_data)
            .expect("failed to convert to TradeQuoteSnapshot");
        println!(
            "\n* G703F parsing for isin code: {:?}\n",
            trade_quote_data.isin_code.as_str()
        );
        dbg!(trade_quote_data.clone());

        let converter = ifmsrpd0037.get_order_converter();
        assert_eq!(trade_quote_data.isin_code.as_str(), "KR4301V13502");
        assert_eq!(trade_quote_data.timestamp, 104939081108);
        assert_eq!(trade_quote_data.trade_price, 212);
        assert_eq!(
            (converter.price.to_f64_from_i64(trade_quote_data.trade_price) - 2.12).abs() < 1.0e-8, 
            true,
            "trade_price_f64: {}", converter.price.to_f64_from_i64(trade_quote_data.trade_price)
        );

        assert_eq!(trade_quote_data.trade_quantity, 5);
        assert_eq!(trade_quote_data.trade_type, Some(TradeType::Buy));

        let ask_order1 = trade_quote_data.ask_quote_data[0];
        assert_eq!(
            ask_order1, 
            LevelSnapshot {
                order_count: 3,
                book_price: 212,
                book_quantity: 10,
            }
        );

        let bid_order1 = trade_quote_data.bid_quote_data[0];
        assert_eq!(
            bid_order1, 
            LevelSnapshot {
                order_count: 6,
                book_price: 211,
                book_quantity: 10,
            },
        );

        let ask_order5 = trade_quote_data.ask_quote_data[4];
        assert_eq!(
            ask_order5, 
            LevelSnapshot {
                order_count: 7,
                book_price: 216,
                book_quantity: 18,
            },
        );

        let bid_order5 = trade_quote_data.bid_quote_data[4];
        assert_eq!(
            bid_order5, 
            LevelSnapshot {
                order_count: 11,
                book_price: 207,
                book_quantity: 62,
            },
        );

        Ok(())
    }

    #[test]
    fn test_parse_with_buffer() -> Result<()> {
        let mut test_data_vec = b"G703F        G140KR4301V13502001656104939081108000002.12000000005000000.00000000.00000002.83000002.93000002.06000002.11000000021511000000013250790000.0002000006.86000000.01000002.12000002.110000000100000000100000300006000002.13000002.100000000330000000410001100011000002.14000002.090000000290000000430000800010000002.15000002.080000000380000000370000900013000002.16000002.0700000001800000006200007000110000017960000059190049400380".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0037 = IFMSRPD0037::default().with_quote_level_cut(4);

        let mut trade_quote_data = TradeQuoteSnapshot::with_quote_level(4);
        ifmsrpd0037
            .to_trade_quote_snapshot_buffer(test_data, &mut trade_quote_data)
            .expect("failed to convert to TradeQuoteSnapshot");
        println!(
            "\n* G703F parsing for isin code: {:?}\n",
            trade_quote_data.isin_code.as_str()
        );
        dbg!(trade_quote_data.clone());
        let converter = ifmsrpd0037.get_order_converter();
        assert_eq!(trade_quote_data.isin_code.as_str(), "KR4301V13502");
        assert_eq!(trade_quote_data.timestamp, 104939081108);
        assert_eq!(trade_quote_data.trade_price, 212);
        assert_eq!(
            (converter.price.to_f64_from_i64(trade_quote_data.trade_price) - 2.12).abs() < 1.0e-8, 
            true,
            "trade_price_f64: {}", converter.price.to_f64_from_i64(trade_quote_data.trade_price)
        );

        assert_eq!(trade_quote_data.trade_quantity, 5);
        assert_eq!(trade_quote_data.trade_type, Some(TradeType::Buy));

        let ask_order1 = trade_quote_data.ask_quote_data[0];
        assert_eq!(
            ask_order1, 
            LevelSnapshot {
                order_count: 3,
                book_price: 212,
                book_quantity: 10,
            }
        );

        let bid_order1 = trade_quote_data.bid_quote_data[0];
        assert_eq!(
            bid_order1, 
            LevelSnapshot {
                order_count: 6,
                book_price: 211,
                book_quantity: 10,
            },
        );

        let ask_order4 = trade_quote_data.ask_quote_data[3];
        assert_eq!(
            ask_order4, 
            LevelSnapshot {
                order_count: 9,
                book_price: 215,
                book_quantity: 38,
            },
        );

        let bid_order4 = trade_quote_data.bid_quote_data[3];
        assert_eq!(
            bid_order4, 
            LevelSnapshot {
                order_count: 13,
                book_price: 208,
                book_quantity: 37,
            },
        );

        Ok(())
    }

    #[test]
    fn test_parse_ifmsrpd0038() -> Result<()> {
        let mut test_data_vec = b"G704F        G140KR41CNV10006003661104939829612000066500000000007000000000000000000000070300000070900000066100000066400000000041770000000028415067000.000200006990000006310000006660000006640000000006900000006800010000060000667000000663000000000810000001630001200011000066800000066200000000066000000049000120000700006690000006610000000004400000012900013000200000670000000660000000000300000000970000900016000067100000065900000000030000000036000060000600006720000006580000000009100000002300007000080000673000000657000000000290000000160001000005000067400000065600000000026000000043000060001100006750000006550000000004500000004000011000080000023600000021120046600205".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0038 = IFMSRPD0038::default();

        let trade_quote_data = ifmsrpd0038
            .to_trade_quote_snapshot(test_data)
            .expect("failed to convert to TradeQuoteSnapshot");

        println!(
            "\n* G704F parsing for isin code: {:?}\n",
            trade_quote_data.isin_code.as_str()
        );

        dbg!(trade_quote_data.clone());

        assert_eq!(trade_quote_data.isin_code.as_str(), "KR41CNV10006");
        assert_eq!(trade_quote_data.timestamp, 104_939_829_612);
        assert_eq!(trade_quote_data.trade_price, 66500);
        assert_eq!(trade_quote_data.trade_quantity, 7);
        assert_eq!(trade_quote_data.trade_type, Some(TradeType::Buy));
        let ask_quote = trade_quote_data.effective_ask_data();
        let bid_quote = trade_quote_data.effective_bid_data();

        assert_eq!(
            ask_quote[0],
            LevelSnapshot {
                order_count: 10,
                book_price: 66600,
                book_quantity: 69,
            },
        );

        assert_eq!(
            ask_quote[9],
            LevelSnapshot {
                order_count: 11,
                book_price: 67500,
                book_quantity: 45,
            },
        );

        assert_eq!(
            bid_quote[9],
            LevelSnapshot {
                order_count: 8,
                book_price: 65500,
                book_quantity: 40,
            },
        );
        Ok(())
    }

    #[test]
    fn test_parse_ifmsrpd0038_with_buffer() -> Result<()> {
        let mut test_data_vec = b"G704F        G140KR41CNV10006003661104939829612000066500000000007000000000000000000000070300000070900000066100000066400000000041770000000028415067000.000200006990000006310000006660000006640000000006900000006800010000060000667000000663000000000810000001630001200011000066800000066200000000066000000049000120000700006690000006610000000004400000012900013000200000670000000660000000000300000000970000900016000067100000065900000000030000000036000060000600006720000006580000000009100000002300007000080000673000000657000000000290000000160001000005000067400000065600000000026000000043000060001100006750000006550000000004500000004000011000080000023600000021120046600205".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0038 = IFMSRPD0038::default().with_quote_level_cut(6);

        let mut trade_quote_data = TradeQuoteSnapshot::with_quote_level(6);
        ifmsrpd0038
            .to_trade_quote_snapshot_buffer(test_data, &mut trade_quote_data)
            .expect("failed to convert to TradeQuoteSnapshot");

        println!(
            "\n* G704F parsing for isin code: {:?}\n",
            trade_quote_data.isin_code.as_str()
        );

        dbg!(trade_quote_data.clone());

        assert_eq!(trade_quote_data.isin_code.as_str(), "KR41CNV10006");
        assert_eq!(trade_quote_data.timestamp, 104_939_829_612);
        assert_eq!(trade_quote_data.trade_price, 66500);
        assert_eq!(trade_quote_data.trade_quantity, 7);
        assert_eq!(trade_quote_data.trade_type, Some(TradeType::Buy));
        let ask_quote = trade_quote_data.effective_ask_data();
        let bid_quote = trade_quote_data.effective_bid_data();

        dbg!(trade_quote_data.clone());

        assert_eq!(
            ask_quote[0],
            LevelSnapshot {
                order_count: 10,
                book_price: 66600,
                book_quantity: 69,
            },
        );
        
        assert_eq!(
            ask_quote[5],
            LevelSnapshot {
                order_count: 6,
                book_price: 67100,
                book_quantity: 30,
            },
        );
        
        assert_eq!(
            bid_quote[0],
            LevelSnapshot {
                order_count: 6,
                book_price: 66400,
                book_quantity: 68,
            },
        );

        assert_eq!(
            bid_quote[5],
            LevelSnapshot {
                order_count: 6,
                book_price: 65900,
                book_quantity: 36,
            },
        );
        Ok(())
    }
    
}
