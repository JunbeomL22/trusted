use crate::data::quote::QuoteSnapshot;
use crate::types::{
    base::{LevelSnapshot, Slice},
    isin_code::IsinCode,
    venue::Venue,
    timestamp::DateStampGenerator,
};
use crate::data::checker::Checker;
use crate::{
    parse_unroll,
    parse_unroll_with_buffer,
};
use crate::data::krx::krx_converter::{
    get_krx_derivative_converter,
    get_krx_timestamp_converter,
    get_krx_base_order_counter,
};

use anyhow::{anyhow, Result};

/// Message Structure:
/// 파생 우선호가 (우선호가 5단계)
/// Derivative Best Bid/Ask (5 levels)
/// +----------------------------------|----------|------|----------+
/// | ItemName                         | DataType | 길이 |  누적길이 |
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
    payload_length: usize,
    //
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    quote_level_cut: usize,   // <= self.quote_level, ex) 3 => only 3 levels parsed
    quote_start_index: usize, // 172 in this case
}

impl Default for IFMSRPD0034 {
    fn default() -> Self {
        IFMSRPD0034 {
            payload_length: 324,
            //
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            quote_level_cut: 5,
            quote_start_index: 47,
        }
    }
}

impl Checker for IFMSRPD0034 {
    #[inline]
    fn as_str(&self) -> &'static str {
        "IFMSRPD0034"
    }

    #[inline]
    fn get_payload_length(&self) -> usize {
        self.payload_length
    }

    #[inline]
    fn get_quote_level_cut(&self) -> usize {
        self.quote_level_cut
    }
}

impl IFMSRPD0034 {
    #[inline]
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Result<Self> {
        if quote_level_cut > 5 {
            let err = || anyhow!("{} can not have more than 5 levels of quote data", self.as_str());
            return Err(err());
        }
        self.quote_level_cut = quote_level_cut;
        Ok(self)
    }

    pub fn to_quote_snapshot(&self, payload: &[u8], date_gen: &mut DateStampGenerator) -> Result<QuoteSnapshot> {
        self.is_valid_krx_payload(payload)?;
        let venue = Venue::KRX;
        let isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;

        let converter = get_krx_derivative_converter(&payload[..5], &isin_code);
        let timestamp_converter = get_krx_timestamp_converter();
        let order_counter = get_krx_base_order_counter();

        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = order_counter.order_count.get_config().total_length;

        /* 
        let timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };
        */
        let timestamp = timestamp_converter.parse_hhmmssuuuuuu(&payload[self.timestamp_slice.start..self.timestamp_slice.end], date_gen)?;
        
        let quote_level_cut = self.quote_level_cut;
        let quote_start_index = self.quote_start_index;
        let mut ask_quote_data = vec![LevelSnapshot::default(); quote_level_cut];
        let mut bid_quote_data = vec![LevelSnapshot::default(); quote_level_cut];

        let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
        parse_unroll!(
            quote_level_cut,
            quote_start_index,
            offset,
            payload,
            ask_quote_data,
            bid_quote_data,
            converter,
            order_counter,
            pr_ln,
            qn_ln,
            or_ln
        );

        Ok(QuoteSnapshot {
            venue,
            isin_code,
            timestamp,
            ask_quote_data,
            bid_quote_data,
            quote_level_cut: self.quote_level_cut,
            all_lp_holdings: None,
            //
            order_counter: order_counter,
            order_converter: converter,
        })
    }

    pub fn to_quote_snapshot_buffer(
        &self, 
        payload: &[u8], 
        buffer: &mut QuoteSnapshot,
        date_gen: &mut DateStampGenerator
    ) -> Result<()> {
        self.is_valid_krx_payload(payload)?;
        self.is_valid_quote_snapshot_buffer(payload, buffer)?;

        buffer.venue = Venue::KRX;
        buffer.isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;

        let converter = get_krx_derivative_converter(&payload[..5], &buffer.isin_code);
        let order_counter = get_krx_base_order_counter();
        let timestamp_converter = get_krx_timestamp_converter();

        buffer.order_converter = converter;
        buffer.order_counter = order_counter;

        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = order_counter.order_count.get_config().total_length;

        buffer.timestamp = timestamp_converter.parse_hhmmssuuuuuu(&payload[self.timestamp_slice.start..self.timestamp_slice.end], date_gen)?;
        /*
        buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };
        */

        let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
        let quote_level_cut = self.quote_level_cut;
        let quote_start_index = self.quote_start_index;

        parse_unroll_with_buffer!(
            quote_level_cut,
            quote_start_index,
            offset,
            payload,
            buffer,
            converter,
            order_counter,
            pr_ln,
            qn_ln,
            or_ln
        );
       
        Ok(())
    }

}


/// Message Structure:
/// 파생 우선호가 (우선호가 10단계) 주식 선물 + 옵션
/// Derivative Best Bid/Ask (10 levels) Stock Futures + Options
/// +----------------------------------|----------|--------|-------------------+
/// | ItemName                         | DataType | Length | CummulativeLength |
/// |----------------------------------|----------|--------|-------------------|
/// | Data Category                    | String   | 2      | 2                 |
/// | Information Category             | String   | 3      | 5                 |
/// | Message sequence number          | Int      | 8      | 13                |
/// | Board ID                         | String   | 2      | 15                |
/// | Session ID                       | String   | 2      | 17                |
/// | ISIN Code                        | String   | 12     | 29                |
/// | A designated number for an issue | Int      | 6      | 35                |
/// | Processing Time of Trading System| String   | 12     | 47                |
/// | Ask Level 1 price                | Double   | 9      | 56                |
/// | Bid Level 1 price                | Double   | 9      | 65                |
/// | Ask Level 1 volume               | Int      | 9      | 74                |
/// | Bid Level 1 volume               | Int      | 9      | 83                |
/// | Ask Level 1_Order Counts         | Int      | 5      | 88                |
/// | Bid Level 1_Order Counts         | Int      | 5      | 93                |
/// | Ask Level 2 price                | Double   | 9      | 102               |
/// | Bid Level 2 price                | Double   | 9      | 111               |
/// | Ask Level 2 volume               | Int      | 9      | 120               |
/// | Bid Level 2 volume               | Int      | 9      | 129               |
/// | Ask Level 2_Order Counts         | Int      | 5      | 134               |
/// | Bid Level 2_Order Counts         | Int      | 5      | 139               |
/// | Ask Level 3 price                | Double   | 9      | 148               |
/// | Bid Level 3 price                | Double   | 9      | 157               |
/// | Ask Level 3 volume               | Int      | 9      | 166               |
/// | Bid Level 3 volume               | Int      | 9      | 175               |
/// | Ask Level 3_Order Counts         | Int      | 5      | 180               |
/// | Bid Level 3_Order Counts         | Int      | 5      | 185               |
/// | Ask Level 4 price                | Double   | 9      | 194               |
/// | Bid Level 4 price                | Double   | 9      | 203               |
/// | Ask Level 4 volume               | Int      | 9      | 212               |
/// | Bid Level 4 volume               | Int      | 9      | 221               |
/// | Ask Level 4_Order Counts         | Int      | 5      | 226               |
/// | Bid Level 4_Order Counts         | Int      | 5      | 231               |
/// | Ask Level 5 price                | Double   | 9      | 240               |
/// | Bid Level 5 price                | Double   | 9      | 249               |
/// | Ask Level 5 volume               | Int      | 9      | 258               |
/// | Bid Level 5 volume               | Int      | 9      | 267               |
/// | Ask Level 5_Order Counts         | Int      | 5      | 272               |
/// | Bid Level 5_Order Counts         | Int      | 5      | 277               |
/// | Ask Level 6 price                | Double   | 9      | 286               |
/// | Bid Level 6 price                | Double   | 9      | 295               |
/// | Ask Level 6 volume               | Int      | 9      | 304               |
/// | Bid Level 6 volume               | Int      | 9      | 313               |
/// | Ask Level 6_Order Counts         | Int      | 5      | 318               |
/// | Bid Level 6_Order Counts         | Int      | 5      | 323               |
/// | Ask Level 7 price                | Double   | 9      | 332               |
/// | Bid Level 7 price                | Double   | 9      | 341               |
/// | Ask Level 7 volume               | Int      | 9      | 350               |
/// | Bid Level 7 volume               | Int      | 9      | 359               |
/// | Ask Level 7_Order Counts         | Int      | 5      | 364               |
/// | Bid Level 7_Order Counts         | Int      | 5      | 369               |
/// | Ask Level 8 price                | Double   | 9      | 378               |
/// | Bid Level 8 price                | Double   | 9      | 387               |
/// | Ask Level 8 volume               | Int      | 9      | 396               |
/// | Bid Level 8 volume               | Int      | 9      | 405               |
/// | Ask Level 8_Order Counts         | Int      | 5      | 410               |
/// | Bid Level 8_Order Counts         | Int      | 5      | 415               |
/// | Ask Level 9 price                | Double   | 9      | 424               |
/// | Bid Level 9 price                | Double   | 9      | 433               |
/// | Ask Level 9 volume               | Int      | 9      | 442               |
/// | Bid Level 9 volume               | Int      | 9      | 451               |
/// | Ask Level 9_Order Counts         | Int      | 5      | 456               |
/// | Bid Level 9_Order Counts         | Int      | 5      | 461               |
/// | Ask Level 10 price               | Double   | 9      | 470               |
/// | Bid Level 10 price               | Double   | 9      | 479               |
/// | Ask Level 10 volume              | Int      | 9      | 488               |
/// | Bid Level 10 volume              | Int      | 9      | 497               |
/// | Ask Level 10_Order Counts        | Int      | 5      | 502               |
/// | Bid Level 10_Order Counts        | Int      | 5      | 507               |
/// | Ask Total Volume                 | Int      | 9      | 516               |
/// | Bid Total Volume                 | Int      | 9      | 525               |
/// | Ask Price_Valid Counts           | Int      | 5      | 530               |
/// | Bid Price_Valid Counts           | Int      | 5      | 535               |
/// | Estimated Trading Price          | Double   | 9      | 544               |
/// | Estimated Trading Volume         | Int      | 9      | 553               |
/// | End Keyword                      | String   | 1      | 554               |
/// |----------------------------------|----------|--------|-------------------|
#[derive(Debug, Clone)]
pub struct IFMSRPD0035 {
    payload_length: usize,
    //
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    quote_level_cut: usize,   // <= self.quote_level, ex) 3 => only 3 levels parsed
    quote_start_index: usize, // 172 in this case
}

impl Checker for IFMSRPD0035 {
    #[inline]
    fn as_str(&self) -> &'static str {
        "IFMSRPD0035"
    }

    #[inline]
    fn get_payload_length(&self) -> usize {
        self.payload_length
    }

    #[inline]
    fn get_quote_level_cut(&self) -> usize {
        self.quote_level_cut
    }
}
impl Default for IFMSRPD0035 {
    fn default() -> Self {
        IFMSRPD0035 {
            payload_length: 554,
            //
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            quote_level_cut: 10,
            quote_start_index: 47,
        }
    }
}

impl IFMSRPD0035 {
    #[inline]
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Result<Self> {
        if quote_level_cut > 10 {
            let err = || anyhow!("{} can not have more than 10 levels of quote data", self.as_str());
            return Err(err());
        }
        self.quote_level_cut = quote_level_cut;
        Ok(self)
    }

    pub fn to_quote_snapshot(&self, payload: &[u8], date_gen: &mut DateStampGenerator) -> Result<QuoteSnapshot> {
        self.is_valid_krx_payload(payload)?;

        let venue = Venue::KRX;
        let isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;

        let converter = get_krx_derivative_converter(&payload[..5], &isin_code);
        let timestamp_converter = get_krx_timestamp_converter();
        let order_counter = get_krx_base_order_counter();

        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = order_counter.order_count.get_config().total_length;

        /* 
        let timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };
        */
        let timestamp = timestamp_converter.parse_hhmmssuuuuuu(&payload[self.timestamp_slice.start..self.timestamp_slice.end], date_gen)?;
        
        let quote_level_cut = self.quote_level_cut;
        let quote_start_index = self.quote_start_index;

        let mut ask_quote_data = vec![LevelSnapshot::default(); quote_level_cut];
        let mut bid_quote_data = vec![LevelSnapshot::default(); quote_level_cut];

        let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
        parse_unroll!(
            quote_level_cut,
            quote_start_index,
            offset,
            payload,
            ask_quote_data,
            bid_quote_data,
            converter,
            order_counter,
            pr_ln,
            qn_ln,
            or_ln
        );

        Ok(QuoteSnapshot {
            venue,
            isin_code,
            timestamp,
            ask_quote_data,
            bid_quote_data,
            quote_level_cut: self.quote_level_cut,
            all_lp_holdings: None,
            //
            order_counter: order_counter,
            order_converter: converter,
        })
    }

    pub fn to_quote_snapshot_buffer(
        &self, payload: &[u8], buffer: &mut QuoteSnapshot, date_gen: &mut DateStampGenerator) -> Result<()> {
        self.is_valid_krx_payload(payload)?;
        self.is_valid_quote_snapshot_buffer(payload, buffer)?;

        buffer.venue = Venue::KRX;
        buffer.isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;

        let converter = get_krx_derivative_converter(&payload[..5], &buffer.isin_code);
        let order_counter = get_krx_base_order_counter();
        let timestamp_converter = get_krx_timestamp_converter();

        buffer.order_converter = converter;
        buffer.order_counter = order_counter;
        
        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = order_counter.order_count.get_config().total_length;
        /* 
        buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };
        */
        buffer.timestamp = timestamp_converter.parse_hhmmssuuuuuu(&payload[self.timestamp_slice.start..self.timestamp_slice.end], date_gen)?;

        let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;

        let quote_level_cut = self.quote_level_cut;
        let quote_start_index = self.quote_start_index;

        parse_unroll_with_buffer!(
            quote_level_cut,
            quote_start_index,
            offset,
            payload,
            buffer,
            converter,
            order_counter,
            pr_ln,
            qn_ln,
            or_ln
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_parse_ifmsrpd0034() -> Result<()> {
        let mut test_data_vec = b"B602F        G140KR4106V30004000020104939405656001379.70001379.500000000030000000030000300003001379.80001379.400000000040000000040000400004001379.90001379.300000000070000000050000600005001380.00001379.200000000050000000070000500007001380.10001379.1000000000500000000500005000050000009020000025920031700642000000.00000000000".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let ifmsrpd0034 = IFMSRPD0034::default();
        let mut date_gen = &mut DateStampGenerator::from(NaiveDate::from_ymd_opt(2023, 12, 30).unwrap());
        let quote_snapshot = ifmsrpd0034.to_quote_snapshot(test_data, &mut date_gen)
            .expect("failed to parse IFMSRPD0034");

        assert_eq!(quote_snapshot.isin_code.as_str(), "KR4106V30004");
        let ask_quote_data = quote_snapshot.effective_ask_data();
        let bid_quote_data = quote_snapshot.effective_bid_data();
        
        assert_eq!(
            ask_quote_data[4],
            LevelSnapshot {
                order_count: Some(5),
                book_price: 138010,
                book_quantity: 5,
                lp_quantity: None,
            },
        );

        assert_eq!(
            bid_quote_data[4],
            LevelSnapshot {
                order_count: Some(5),
                book_price: 137910,
                book_quantity: 5,
                lp_quantity: None,
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
        let mut date_gen = &mut DateStampGenerator::from(NaiveDate::from_ymd_opt(2023, 12, 30).unwrap());
        ifmsrpd0034.to_quote_snapshot_buffer(test_data, &mut quote_snapshot, &mut date_gen)
            .expect("failed to parse IFMSRPD0034");

        assert_eq!(quote_snapshot.isin_code.as_str(), "KR4106V30004");
        let ask_quote_data = quote_snapshot.effective_ask_data();
        let bid_quote_data = quote_snapshot.effective_bid_data();
        
        assert_eq!(
            ask_quote_data[4],
            LevelSnapshot {
                order_count: Some(5),
                book_price: 138010,
                book_quantity: 5,
                lp_quantity: None,
            },
        );

        assert_eq!(
            bid_quote_data[4],
            LevelSnapshot {
                order_count: Some(5),
                book_price: 137910,
                book_quantity: 5,
                lp_quantity: None,
            },
        );

        assert!(true);
        Ok(())
    }
    
    #[test]
    fn test_parse_ifmsrpd0035() {
        unimplemented!("\n\n This test is not implemented yet. \n\n");
    }

    #[test]
    fn test_parse_ifmsrpd0035_with_buffer() {
        unimplemented!("\n\n This test is not implemented yet. \n\n");
    }
}
