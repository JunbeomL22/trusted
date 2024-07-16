use crate::data::{
    quote::QuoteSnapshot,
    trade_quote::TradeQuoteSnapshot,
};

use anyhow::{
    Result,
    anyhow,
};

pub trait Checker {
    fn as_str(&self) -> &'static str;
    fn get_payload_length(&self) -> usize;
    fn get_quote_level_cut(&self) -> usize;
    fn get_lp_quote_level_cut(&self) -> usize { unimplemented!(); }

    #[inline]
    fn is_valid_krx_payload(&self, payload: &[u8]) -> Result<()> {
        let size_checker = self.get_payload_length();
        let tr_name = self.as_str();
        if payload.len() != size_checker {
            let err = || anyhow!(
                "(tr {}, payload length error) expected {}, got {}\n\
                message: {:?}",
                tr_name,
                size_checker,
                payload.len(),
                std::str::from_utf8(&payload[..(size_checker - 1)]).unwrap(),
            );
            return Err(err());
        }

        if payload[size_checker - 1] != 255 {
            let err = || anyhow!(
                "(tr {}, payload end error) expected 255, got {}\n\
                message: {:?}",
                tr_name,
                payload[size_checker - 1],
                std::str::from_utf8(&payload[..(size_checker - 1)]).unwrap(),
            );
            return Err(err());
        }
        Ok(())
    }

    #[inline]
    fn is_valid_quote_snapshot_buffer(&self, payload: &[u8], buffer: &mut QuoteSnapshot) -> Result<()> {
        if buffer.ask_quote_data.len() < self.get_quote_level_cut() ||
        buffer.bid_quote_data.len() < self.get_quote_level_cut() {
            let err = || anyhow!(
                "Buffer is not enough for {}: expected at least {} levels, got {}\n\
                message: {:?}",
                self.as_str(),
                self.get_quote_level_cut(),
                buffer.ask_quote_data.len(),
                std::str::from_utf8(&payload[..(self.get_payload_length()-1)]).unwrap(),
            );
            return Err(err());
        }
        Ok(())
    }

    #[inline]
    fn is_valid_trade_quote_snapshot_buffer(&self, payload: &[u8], buffer: &mut TradeQuoteSnapshot) -> Result<()> {
        if buffer.ask_quote_data.len() < self.get_quote_level_cut() ||
        buffer.bid_quote_data.len() < self.get_quote_level_cut() {
            let err = || anyhow!(
                "Buffer is not enough for {}: expected at least {} levels, got {}\n\
                message: {:?}",
                self.as_str(),
                self.get_quote_level_cut(),
                buffer.ask_quote_data.len(),
                std::str::from_utf8(&payload[..(self.get_payload_length()-1)]).unwrap(),
            );
            return Err(err());
        }
        Ok(())
    }
}