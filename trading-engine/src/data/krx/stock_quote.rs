use crate::{
    types::{
        base::{
            LevelSnapshot,
            Slice,
        },
        isin_code::IsinCode,
        venue::Venue,
    },
    data::{
        quote::QuoteSnapshot,
        krx::krx_converter::{
            KRX_STOCK_ORDER_CONVERTER,
            KRX_TIMESTAMP_CONVERTER,
        },
        checker::Checker,
    },
    utils::numeric_converter::{
        OrderConverter,
        TimeStampConverter,
    },
    parse_unroll_unchecked_price,
    parse_unroll_unchecked_price_with_buffer,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};


/// 증권 우선호가 (MM/LP호가 제외)
/// Stock Priority Price (Excluding MM/LP Price)
/// Message Structure:
/// |---------------------------------------|----------|------|----------|
/// | ItemName                              | DataType | 길이 |  누적길이 |
/// |---------------------------------------|----------|------|----------|
/// | Data Category                         | String   |   2  |     2    |
/// | Information Category                  | String   |   3  |     5    |
/// | Message sequence number               | Int      |   8  |    13    |
/// | Board ID                              | String   |   2  |    15    |
/// | Session ID                            | String   |   2  |    17    |
/// | ISIN Code                             | String   |  12  |    29    |
/// | A designated number for an issue      | Int      |   6  |    35    |
/// | Processing Time of Trading System     | String   |  12  |    47    |
/// | Ask Level 1 price                     | Double   |  11  |    58    |
/// | Bid Level 1 price                     | Double   |  11  |    69    |
/// | Ask Level 1 volume                    | Long     |  12  |    81    |
/// | Bid Level 1 volume                    | Long     |  12  |    93    |
/// | Ask Level 2 price                     | Double   |  11  |   104    |
/// | Bid Level 2 price                     | Double   |  11  |   115    |
/// | Ask Level 2 volume                    | Long     |  12  |   127    |
/// | Bid Level 2 volume                    | Long     |  12  |   139    |
/// | Ask Level 3 price                     | Double   |  11  |   150    |
/// | Bid Level 3 price                     | Double   |  11  |   161    |
/// | Ask Level 3 volume                    | Long     |  12  |   173    |
/// | Bid Level 3 volume                    | Long     |  12  |   185    |
/// | Ask Level 4 price                     | Double   |  11  |   196    |
/// | Bid Level 4 price                     | Double   |  11  |   207    |
/// | Ask Level 4 volume                    | Long     |  12  |   219    |
/// | Bid Level 4 volume                    | Long     |  12  |   231    |
/// | Ask Level 5 price                     | Double   |  11  |   242    |
/// | Bid Level 5 price                     | Double   |  11  |   253    |
/// | Ask Level 5 volume                    | Long     |  12  |   265    |
/// | Bid Level 5 volume                    | Long     |  12  |   277    |
/// | Ask Level 6 price                     | Double   |  11  |   288    |
/// | Bid Level 6 price                     | Double   |  11  |   299    |
/// | Ask Level 6 volume                    | Long     |  12  |   311    |
/// | Bid Level 6 volume                    | Long     |  12  |   323    |
/// | Ask Level 7 price                     | Double   |  11  |   334    |
/// | Bid Level 7 price                     | Double   |  11  |   345    |
/// | Ask Level 7 volume                    | Long     |  12  |   357    |
/// | Bid Level 7 volume                    | Long     |  12  |   369    |
/// | Ask Level 8 price                     | Double   |  11  |   380    |
/// | Bid Level 8 price                     | Double   |  11  |   391    |
/// | Ask Level 8 volume                    | Long     |  12  |   403    |
/// | Bid Level 8 volume                    | Long     |  12  |   415    |
/// | Ask Level 9 price                     | Double   |  11  |   426    |
/// | Bid Level 9 price                     | Double   |  11  |   437    |
/// | Ask Level 9 volume                    | Long     |  12  |   449    |
/// | Bid Level 9 volume                    | Long     |  12  |   461    |
/// | Ask Level 10 price                    | Double   |  11  |   472    |
/// | Bid Level 10 price                    | Double   |  11  |   483    |
/// | Ask Level 10 volume                   | Long     |  12  |   495    |
/// | Bid Level 10 volume                   | Long     |  12  |   507    |
/// | Total ask volume                      | Long     |  12  |   519    |
/// | Total bid volume                      | Long     |  12  |   531    |
/// | Estimated Trading Price               | Double   |  11  |   542    |
/// | Estimated Trading Volume              | Long     |  12  |   554    |
/// | End Keyword                           | String   |   1  |   555    |
/// |---------------------------------------|----------|------|----------|
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFMSRPD0002 {
    payload_length: usize,
    //
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    quote_level_cut: usize,
    quote_start_index: usize,
}

impl Checker for IFMSRPD0002 {
    #[inline]
    fn as_str(&self) -> &'static str {
        "IFMSRPD0002"
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
impl Default for IFMSRPD0002 {
    fn default() -> Self {
        IFMSRPD0002 {
            payload_length: 555,
            //
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            quote_level_cut: 10,
            quote_start_index: 47,
        }
    }

}
impl IFMSRPD0002 {
    #[inline]
    #[must_use]
    pub fn get_order_converter(&self) -> &'static OrderConverter {
        &KRX_STOCK_ORDER_CONVERTER
    }

    #[inline]
    #[must_use]
    pub fn get_timestamp_converter(&self) -> &'static TimeStampConverter {
        &KRX_TIMESTAMP_CONVERTER
    }

    #[inline]
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Result<Self> {
        if quote_level_cut > 10 {
            let err = || anyhow!("{} can not have more than 5 levels of quote data", self.as_str());
            return Err(err());
        }
        self.quote_level_cut = quote_level_cut;
        Ok(self)
    }

    pub fn to_quote_snapshot(&self, payload: &[u8]) -> Result<QuoteSnapshot> {
        self.is_valid_krx_payload(payload)?;

        let converter = self.get_order_converter();
        let timestamp_converter = self.get_timestamp_converter();

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
        let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;

        parse_unroll_unchecked_price!(
            self.quote_level_cut,
            self.quote_start_index,
            offset,
            payload,
            ask_quote_data,
            bid_quote_data,
            converter,
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
            lp_ask_quote_data: None,
            lp_bid_quote_data: None,
            lp_quote_level_cut: None,
            lp_holdings: None,
        })
    }


    pub fn to_quote_snapshot_buffer(&self, payload: &[u8], buffer: &mut QuoteSnapshot) -> Result<()> {
        self.is_valid_krx_payload(payload)?;
        self.is_valid_quote_snapshot_buffer(payload, buffer)?;

        let converter = self.get_order_converter();
        let timestamp_converter = self.get_timestamp_converter();

        let pr_ln = converter.price.get_config().total_length;
        let qn_ln = converter.quantity.get_config().total_length;
        let or_ln = converter.order_count.get_config().total_length;

        buffer.venue = Venue::KRX;

        buffer.isin_code = IsinCode::new(&payload[self.isin_code_slice.start..self.isin_code_slice.end])?;

        buffer.timestamp = unsafe {
            timestamp_converter.to_timestamp_unchecked(&payload[self.timestamp_slice.start..self.timestamp_slice.end])
        };

        let offset = pr_ln * 2 + qn_ln * 2 + or_ln * 2;
        parse_unroll_unchecked_price_with_buffer!(
            self.quote_level_cut,
            self.quote_start_index,
            offset,
            payload,
            buffer,
            converter,
            pr_ln,
            qn_ln,
            or_ln
        );
       
        Ok(())
    }
}

/// 증권 우선호가 (MM/LP호가 포함)
/// Stock Priority Price (Including MM/LP Price)
/// Message Structure:
/// |---------------------------------------|----------|------|----------|
/// | ItemName                              | DataType | 길이 |  누적길이 |
/// |---------------------------------------|----------|------|----------|
/// | Data Category                         | String   |   2  |     2    |
/// | Information Category                  | String   |   3  |     5    |
/// | Message sequence number               | Int      |   8  |    13    |
/// | Board ID                              | String   |   2  |    15    |
/// | Session ID                            | String   |   2  |    17    |
/// | ISIN Code                             | String   |  12  |    29    |
/// | A designated number for an issue      | Int      |   6  |    35    |
/// | Processing Time of Trading System     | String   |  12  |    47    |
/// | Ask Level 1 price                     | Double   |  11  |    58    |
/// | Bid Level 1 price                     | Double   |  11  |    69    |
/// | Ask Level 1 volume                    | Long     |  12  |    81    |
/// | Bid Level 1 volume                    | Long     |  12  |    93    |
/// | LP_Ask Level 1 volume                 | Long     |  12  |   105    |
/// | LP_Bid Level 1 volume                 | Long     |  12  |   117    |
/// | Ask Level 2 price                     | Double   |  11  |   128    |
/// | Bid Level 2 price                     | Double   |  11  |   139    |
/// | Ask Level 2 volume                    | Long     |  12  |   151    |
/// | Bid Level 2 volume                    | Long     |  12  |   163    |
/// | LP_Ask Level 2 volume                 | Long     |  12  |   175    |
/// | LP_Bid Level 2 volume                 | Long     |  12  |   187    |
/// | Ask Level 3 price                     | Double   |  11  |   198    |
/// | Bid Level 3 price                     | Double   |  11  |   209    |
/// | Ask Level 3 volume                    | Long     |  12  |   221    |
/// | Bid Level 3 volume                    | Long     |  12  |   233    |
/// | LP_Ask Level 3 volume                 | Long     |  12  |   245    |
/// | LP_Bid Level 3 volume                 | Long     |  12  |   257    |
/// | Ask Level 4 price                     | Double   |  11  |   268    |
/// | Bid Level 4 price                     | Double   |  11  |   279    |
/// | Ask Level 4 volume                    | Long     |  12  |   291    |
/// | Bid Level 4 volume                    | Long     |  12  |   303    |
/// | LP_Ask Level 4 volume                 | Long     |  12  |   315    |
/// | LP_Bid Level 4 volume                 | Long     |  12  |   327    |
/// | Ask Level 5 price                     | Double   |  11  |   338    |
/// | Bid Level 5 price                     | Double   |  11  |   349    |
/// | Ask Level 5 volume                    | Long     |  12  |   361    |
/// | Bid Level 5 volume                    | Long     |  12  |   373    |
/// | LP_Ask Level 5 volume                 | Long     |  12  |   385    |
/// | LP_Bid Level 5 volume                 | Long     |  12  |   397    |
/// | Ask Level 6 price                     | Double   |  11  |   408    |
/// | Bid Level 6 price                     | Double   |  11  |   419    |
/// | Ask Level 6 volume                    | Long     |  12  |   431    |
/// | Bid Level 6 volume                    | Long     |  12  |   443    |
/// | LP_Ask Level 6 volume                 | Long     |  12  |   455    |
/// | LP_Bid Level 6 volume                 | Long     |  12  |   467    |
/// | Ask Level 7 price                     | Double   |  11  |   478    |
/// | Bid Level 7 price                     | Double   |  11  |   489    |
/// | Ask Level 7 volume                    | Long     |  12  |   501    |
/// | Bid Level 7 volume                    | Long     |  12  |   513    |
/// | LP_Ask Level 7 volume                 | Long     |  12  |   525    |
/// | LP_Bid Level 7 volume                 | Long     |  12  |   537    |
/// | Ask Level 8 price                     | Double   |  11  |   548    |
/// | Bid Level 8 price                     | Double   |  11  |   559    |
/// | Ask Level 8 volume                    | Long     |  12  |   571    |
/// | Bid Level 8 volume                    | Long     |  12  |   583    |
/// | LP_Ask Level 8 volume                 | Long     |  12  |   595    |
/// | LP_Bid Level 8 volume                 | Long     |  12  |   607    |
/// | Ask Level 9 price                     | Double   |  11  |   618    |
/// | Bid Level 9 price                     | Double   |  11  |   629    |
/// | Ask Level 9 volume                    | Long     |  12  |   641    |
/// | Bid Level 9 volume                    | Long     |  12  |   653    |
/// | LP_Ask Level 9 volume                 | Long     |  12  |   665    |
/// | LP_Bid Level 9 volume                 | Long     |  12  |   677    |
/// | Ask Level 10 price                    | Double   |  11  |   688    |
/// | Bid Level 10 price                    | Double   |  11  |   699    |
/// | Ask Level 10 volume                   | Long     |  12  |   711    |
/// | Bid Level 10 volume                   | Long     |  12  |   723    |
/// | LP_Ask Level 10 volume                | Long     |  12  |   735    |
/// | LP_Bid Level 10 volume                | Long     |  12  |   747    |
/// | Total ask volume                      | Long     |  12  |   759    |
/// | Total bid volume                      | Long     |  12  |   771    |
/// | Estimated Trading Price               | Double   |  11  |   782    |
/// | Estimated Trading Volume              | Long     |  12  |   794    |
/// | End Keyword                           | String   |   1  |   795    |
/// |---------------------------------------|----------|------|----------|
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFMSRPD0003 {
    payload_length: usize,
    //
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    quote_level_cut: usize,
    quote_start_index: usize,
    //
    lp_quote_level_cut: usize,
}

impl Default for IFMSRPD0003 {
    fn default() -> Self {
        IFMSRPD0003 {
            payload_length: 795,
            //
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            quote_level_cut: 10,
            quote_start_index: 47,
            //
            lp_quote_level_cut: 10,
        }
    }
}

impl Checker for IFMSRPD0003 {
    #[inline]
    fn as_str(&self) -> &'static str {
        "IFMSRPD0003"
    }

    #[inline]
    fn get_payload_length(&self) -> usize {
        self.payload_length
    }

    #[inline]
    fn get_quote_level_cut(&self) -> usize {
        self.quote_level_cut
    }

    #[inline]
    fn get_lp_quote_level_cut(&self) -> usize {
        self.lp_quote_level_cut
    }
}

impl IFMSRPD0003 {
    #[inline]
    pub fn with_quote_level_cut(mut self, quote_level_cut: usize) -> Result<Self> {
        if quote_level_cut > 10 {
            let err = || anyhow!("{} can not have more than 5 levels of quote data", self.as_str());
            return Err(err());
        }
        self.quote_level_cut = quote_level_cut;
        Ok(self)
    }

    #[inline]
    pub fn with_lp_quote_level_cut(mut self, lp_quote_level_cut: usize) -> Result<Self> {
        if lp_quote_level_cut > 10 {
            let err = || anyhow!("{} can not have more than 5 levels of lp quote data", self.as_str());
            return Err(err());
        }
        self.lp_quote_level_cut = lp_quote_level_cut;
        Ok(self)
    }
}