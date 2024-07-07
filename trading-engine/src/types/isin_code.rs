use crate::utils::checkers;
use flexstr::LocalStr;
use anyhow::{Result, anyhow};
use serde::{
    Serialize, 
    Deserialize,
};

#[derive(Debug, Clone, Eq, Hash, Deserialize, Serialize, Default)]
pub struct IsinCode {
    isin: LocalStr,
}

impl IsinCode {
    pub fn new(isin: &str) -> Result<Self> {
        if !checkers::valid_isin_code_length(isin) {
            let err = || anyhow!("Invalid ISIN code: invalid length: {}", isin);
            return Err(err());
        }

        if checkers::contains_white_space(isin) {
            let err = || anyhow!("Invalid ISIN code: contains white space: {}", isin);
            return Err(err());
        }

        if !checkers::is_ascii(isin) {
            let err = || anyhow!("Invalid ISIN code: not ascii: {}", isin);
            return Err(err());
        }

        Ok(IsinCode { isin: LocalStr::from(isin) } )
    }

    pub fn as_str(&self) -> &str {
        self.isin.as_str()
    }
}

impl PartialEq<&str> for IsinCode {
    fn eq(&self, other: &&str) -> bool {
        self.isin.as_str() == *other
    }
}

impl PartialEq<IsinCode> for &str {
    fn eq(&self, other: &IsinCode) -> bool {
        *self == other.isin.as_str()
    }
}

impl PartialEq<IsinCode> for IsinCode {
    fn eq(&self, other: &IsinCode) -> bool {
        self.isin.as_str() == other.isin.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_hash::FxHashMap;

    #[test]
    fn test_isin_code() -> Result<()> {
        let isin = "KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");
        
        assert_eq!(isin_code.as_str(), isin);
        Ok(())
    }

    #[test]
    fn test_isin_code_invalid() {
        let isin = "KR7005930003 ";
        let isin_code = IsinCode::new(isin);
        assert!(isin_code.is_err());
    }

    #[test]
    fn test_paraility() -> Result<()> {
        let isin = "KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");
        
        assert_eq!(isin_code, isin);
        assert_eq!(isin, isin_code);
        assert_eq!(isin_code, isin_code);
        Ok(())
    }

    #[test]
    fn test_hash_map() -> Result<()> {
        let isin = "KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");
        
        let mut map = FxHashMap::default();
        map.insert(isin_code.clone(), 1);

        let test_key = IsinCode::new("KR7005930003").expect("failed to create IsinCode");
        assert_eq!(map.get(&test_key), Some(&1));
        Ok(())
    }
}

