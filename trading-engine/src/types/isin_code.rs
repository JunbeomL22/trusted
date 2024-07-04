use crate::utils::checkers;
use flexstr::LocalStr;
use anyhow::{Result, anyhow};
use serde::{
    Serialize, 
    Deserialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub struct IsinCode {
    isin: LocalStr,
}

impl IsinCode {
    pub fn new(isin: &str) -> Result<Self> {
        match checkers::valid_isin_code_length(isin)? &&
                checkers::contains_white_space(isin)? &&
                checkers::is_ascii(isin)? {
            true => Ok(IsinCode {
                isin: LocalStr::from(isin),
            }),
            false => {
                let lazy_error = || anyhow!("Invalid ISIN code: {}", isin);
                Err( lazy_error() )
            }
        }
    }

    pub fn as_str(&self) -> &str {
        self.isin.as_str()
    }
}
