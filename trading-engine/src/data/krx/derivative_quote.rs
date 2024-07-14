use crate::data::quote::QuoteSnapshot;
use crate::types::{
    base::{LevelSnapshot, Slice},
    enums::TradeType,
    isin_code::IsinCode,
    venue::Venue,
};
use crate::utils::numeric_converter::{OrderConverter, TimeStampConverter};
use anyhow::{anyhow, Result};
/// Message Structure:
/// 파생 우선호가 (우선호가 5단계)
/// Derivative Best Bid/Ask (5 levels)
/// +----------------------------------|----------|------|----------+
/// | ItemName                         | DataType | 길이 | 누적길이 |
/// |----------------------------------|----------|------|----------|
/// | Data Category                    | String   | 2    | 2        |
/// | Information Category             | String   | 3    | 5        |
/// | Message sequence number          | Int      | 8    | 13       |
/// | Board ID                         | String   | 2    | 15       |
/// | Session ID                       | String   | 2    | 17       |
/// | ISIN Code                        | String   | 12   | 29       |
/// | A designated number for an issue | Int      | 6    | 35       |
/// | Processing Time of Trading System| String   | 12   | 47       |
/// | Ask Level 1 price                | Double   | 9    | 56       |
/// | Bid Level 1 price                | Double   | 9    | 65       |
/// | Ask Level 1 volume               | Int      | 9    | 74       |
/// | Bid Level 1 volume               | Int      | 9    | 83       |
/// | Ask Level 1_Order Counts         | Int      | 5    | 88       |
/// | Bid Level 1_Order Counts         | Int      | 5    | 93       |
/// | Ask Level 2 price                | Double   | 9    | 102      |
/// | Bid Level 2 price                | Double   | 9    | 111      |
/// | Ask Level 2 volume               | Int      | 9    | 120      |
/// | Bid Level 2 volume               | Int      | 9    | 129      |
/// | Ask Level 2_Order Counts         | Int      | 5    | 134      |
/// | Bid Level 2_Order Counts         | Int      | 5    | 139      |
/// | Ask Level 3 price                | Double   | 9    | 148      |
/// | Bid Level 3 price                | Double   | 9    | 157      |
/// | Ask Level 3 volume               | Int      | 9    | 166      |
/// | Bid Level 3 volume               | Int      | 9    | 175      |
/// | Ask Level 3_Order Counts         | Int      | 5    | 180      |
/// | Bid Level 3_Order Counts         | Int      | 5    | 185      |
/// | Ask Level 4 price                | Double   | 9    | 194      |
/// | Bid Level 4 price                | Double   | 9    | 203      |
/// | Ask Level 4 volume               | Int      | 9    | 212      |
/// | Bid Level 4 volume               | Int      | 9    | 221      |
/// | Ask Level 4_Order Counts         | Int      | 5    | 226      |
/// | Bid Level 4_Order Counts         | Int      | 5    | 231      |
/// | Ask Level 5 price                | Double   | 9    | 240      |
/// | Bid Level 5 price                | Double   | 9    | 249      |
/// | Ask Level 5 volume               | Int      | 9    | 258      |
/// | Bid Level 5 volume               | Int      | 9    | 267      |
/// | Ask Level 5_Order Counts         | Int      | 5    | 272      |
/// | Bid Level 5_Order Counts         | Int      | 5    | 277      |
/// | Open Step Ask Total Volume       | Int      | 9    | 286      |
/// | Open Step BidTotal Volume        | Int      | 9    | 295      |
/// | Open Step Ask Total Counts       | Int      | 5    | 300      |
/// | Open Step Bid Total Counts       | Int      | 5    | 305      |
/// | Estimated Trading Price          | Double   | 9    | 314      |
/// | Estimated Trading Volume         | Int      | 9    | 323      |
/// | End Keyword                      | String   | 1    | 324      |
/// |----------------------------------|----------|------|----------|
#[derive(Debug, Clone)]
pub struct IFMSRPD0034 {
    order_converter: OrderConverter,
    time_stamp_converter: TimeStampConverter,
    //
    payload_length: usize,
    //
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    quote_level: usize,       // 5 given in this case
    quote_level_cut: usize,   // <= self.quote_level, ex) 3 => only 3 levels parsed
    quote_start_index: usize, // 172 in this case
}

impl Default for IFMSRPD0034 {
    fn default() -> Self {
        IFMSRPD0034 {
            order_converter: OrderConverter::krx_derivative_converter(),
            time_stamp_converter: TimeStampConverter::krx_timestamp_converter(),
            //
            payload_length: 324,
            //
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            quote_level: 5,
            quote_level_cut: 5,
            quote_start_index: 47,
        }
    }
}

impl IFMSRPD0034 {
    #[inline]
    pub fn get_order_converter(&self) -> &OrderConverter {
        &self.order_converter
    }

    #[inline]
    pub fn get_time_stamp_converter(&self) -> &TimeStampConverter {
        &self.time_stamp_converter
    }

    #[inline]
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Self {
        self.quote_level_cut = quote_level_cut;
        self
    }

    pub fn to_quote_snapshot(&self, payload: &[u8]) -> Result<QuoteSnapshot> {
        if payload.len() != self.payload_length {
            let err = || anyhow!(
                "Unmatched payload length for IFMSRPD0034: expected {}, got {}\n\
                message: {:?}",
                self.payload_length,
                payload.len(),
                std::str::from_utf8(&payload[..(self.payload_length-1)]).unwrap(),
            );
            return Err(err());
        }

        if payload[self.payload_length - 1] != 255 {
            let err = || anyhow!(
                "End keyword is not matched for IFMSRPD0034: expected 255\n\
                message: {:?}",
                payload,
            );
            return Err(err());
        }

        let converter = &self.order_converter;
        let timestamp_converter = &self.time_stamp_converter;

        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = converter.order_count.get_config().total_length;

        let venue = Venue::KRX;

        let isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;
        let timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };
        
        let mut ask_quote_data = vec![LevelSnapshot::default(); self.quote_level_cut];
        let mut bid_quote_data = vec![LevelSnapshot::default(); self.quote_level_cut];

        unsafe {
            for i in 0..self.quote_level_cut {
                if i >= self.quote_level {
                    break;
                }
                let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
                let st_idx_marker = self.quote_start_index + offset * i;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                ask_quote_data[i].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                bid_quote_data[i].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                ask_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                bid_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                ask_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                bid_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
        }

        Ok(QuoteSnapshot {
            venue,
            isin_code,
            timestamp,
            ask_quote_data,
            bid_quote_data,
            quote_level_cut: self.quote_level_cut,
        })
    }

    pub fn to_quote_snapshot_buffer(&self, payload: &[u8], buffer: &mut QuoteSnapshot) -> Result<()> {
        if buffer.ask_quote_data.len() < self.quote_level_cut || 
        buffer.bid_quote_data.len() < self.quote_level_cut {
            let err = || anyhow!(
                "Buffer is not enough for IFMSRPD0034: expected at least {} levels, got {}\n\
                message: {:?}",
                self.quote_level_cut,
                buffer.ask_quote_data.len(),
                std::str::from_utf8(&payload[..(self.payload_length-1)]).unwrap(),
            );
            return Err(err());
        }
        if payload.len() != self.payload_length {
            let err = || anyhow!(
                "Unmatched payload length for IFMSRPD0034: expected {}, got {}\n\
                message: {:?}",
                self.payload_length,
                payload.len(),
                std::str::from_utf8(&payload[..(self.payload_length-1)]).unwrap(),
            );
            return Err(err());
        }

        if payload[self.payload_length - 1] != 255 {
            let err = || anyhow!(
                "End keyword is not matched for IFMSRPD0034: expected 255\n\
                payload[self.payload_length - 1] = {}\n\
                message: {:?}",
                payload[self.payload_length - 1],
                std::str::from_utf8(&payload[..(self.payload_length-1)]).unwrap(),
            );
            return Err(err());
        }

        let converter = &self.order_converter;
        let timestamp_converter = &self.time_stamp_converter;

        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = converter.order_count.get_config().total_length;

        buffer.venue = Venue::KRX;

        buffer.isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;

        buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };

        unsafe {
            for i in 0..self.quote_level_cut {
                if i >= self.quote_level {
                    break;
                }
                let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
                let st_idx_marker = self.quote_start_index + offset * i;
                let payload_clipped = &payload[st_idx_marker..st_idx_marker + offset];

                buffer.ask_quote_data[i].book_price = converter.to_book_price(&payload_clipped[0..pr_ln]);
                let idx_marker1 = pr_ln + pr_ln;
                buffer.bid_quote_data[i].book_price = converter.to_book_price(&payload_clipped[pr_ln..idx_marker1]);

                buffer.ask_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + qn_ln]);
                let idx_marker2 = idx_marker1 + qn_ln;
                buffer.bid_quote_data[i].book_quantity = converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + qn_ln]);

                let idx_marker3 = idx_marker2 + qn_ln;
                buffer.ask_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + or_ln]);
                let idx_marker4 = idx_marker3 + or_ln;
                buffer.bid_quote_data[i].order_count = converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + or_ln]);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ifmsrpd0034() -> Result<()> {
        let mut test_data_vec = b"B602F        G140KR4106V30004000020104939405656001379.70001379.500000000030000000030000300003001379.80001379.400000000040000000040000400004001379.90001379.300000000070000000050000600005001380.00001379.200000000050000000070000500007001380.10001379.1000000000500000000500005000050000009020000025920031700642000000.00000000000".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0034 = IFMSRPD0034::default();
        
        let quote_snapshot = ifmsrpd0034.to_quote_snapshot(test_data)
            .expect("failed to parse IFMSRPD0034");

        assert_eq!(quote_snapshot.isin_code.as_str(), "KR4106V30004");
        let ask_quote_data = quote_snapshot.effective_ask_data();
        let bid_quote_data = quote_snapshot.effective_bid_data();
        
        assert_eq!(
            ask_quote_data[4],
            LevelSnapshot {
                order_count: 5,
                book_price: 138010,
                book_quantity: 5,
            },
        );

        assert_eq!(
            bid_quote_data[4],
            LevelSnapshot {
                order_count: 5,
                book_price: 137910,
                book_quantity: 5,
            },
        );

        assert!(true);
        Ok(())

    }

    #[test]
    fn test_parse_ifmsrpd0034_with_buffer() -> Result<()> {
        let mut test_data_vec = b"B602F        G140KR4106V30004000020104939405656001379.70001379.500000000030000000030000300003001379.80001379.400000000040000000040000400004001379.90001379.300000000070000000050000600005001380.00001379.200000000050000000070000500007001380.10001379.1000000000500000000500005000050000009020000025920031700642000000.00000000000".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0034 = IFMSRPD0034::default();
        let mut quote_snapshot = QuoteSnapshot::with_quote_level(5);
        
        ifmsrpd0034.to_quote_snapshot_buffer(test_data, &mut quote_snapshot)
            .expect("failed to parse IFMSRPD0034");

        assert_eq!(quote_snapshot.isin_code.as_str(), "KR4106V30004");
        let ask_quote_data = quote_snapshot.effective_ask_data();
        let bid_quote_data = quote_snapshot.effective_bid_data();
        
        assert_eq!(
            ask_quote_data[4],
            LevelSnapshot {
                order_count: 5,
                book_price: 138010,
                book_quantity: 5,
            },
        );

        assert_eq!(
            bid_quote_data[4],
            LevelSnapshot {
                order_count: 5,
                book_price: 137910,
                book_quantity: 5,
            },
        );

        assert!(true);
        Ok(())
    }
}