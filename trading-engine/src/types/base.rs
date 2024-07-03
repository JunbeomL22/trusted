use ustr::Ustr;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

/// if we encounter a venue using non u64 type OrderId, we must change this to enum OrderId.
/// I leave this primitive for performance reasons.
pub type OrderId = u64; 

pub type AccountId = Ustr;

pub type TraderId = Ustr;

#[derive(Default, Debug, Clone, Serialize, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumReprCfg {
    pub digit_length: usize,
    pub decimal_point_length: usize,
    pub include_negative: bool,
    pub total_length: usize,
}

impl NumReprCfg {
    pub fn new(
        digit_length: usize, 
        decimal_point_length: usize, 
        include_negative: bool,
        total_length: usize,
    ) -> Result<NumReprCfg> {
        let mut check_size: usize = if decimal_point_length == 0 {
            digit_length
        } else {
            digit_length + decimal_point_length + 1
        };
    
        check_size += if include_negative { 1 } else { 0 };
    
        if check_size != total_length {
            let error = || anyhow!(
                "NumReprCfg::new => size check failed: \n\
                digit_length: {:?}\n\
                decimal_point_length: {:?}\n\
                include_negative: {:?}\n\
                total_length: {:?}\n\
                check_size: {:?}",
                digit_length, decimal_point_length, include_negative, total_length, check_size
            );
            return Err(error());
        } 
        Ok(NumReprCfg {
            digit_length,
            decimal_point_length,
            include_negative,
            total_length,
        })
    }
}

pub type BookPrice = i64;

pub type BookQuantity = u64;
