use crate::utils::checkers;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::from_utf8_unchecked;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, Deserialize, Serialize, Default)]
pub struct IsinCode {
    isin: [u8; 12],
}

impl PartialEq for IsinCode {
    fn eq(&self, other: &Self) -> bool {
        self.isin == other.isin
    }
}

impl Hash for IsinCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.isin.hash(state);
    }
}

impl PartialEq<[u8; 12]> for IsinCode {
    fn eq(&self, other: &[u8; 12]) -> bool {
        self.isin == *other
    }
}

impl IsinCode {
    pub fn from_u8_unchecked(isin: [u8; 12]) -> Self {
        IsinCode { isin }
    }

    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.isin) }
    }

    pub fn new(isin: &[u8]) -> Result<Self> {
        if !checkers::valid_isin_code_length(isin) {
            let err = || anyhow!("Invalid ISIN code: invalid length: {:?}", isin);
            return Err(err());
        }

        if checkers::contains_white_space(isin) {
            let err = || anyhow!("Invalid ISIN code: contains white space: {:?}", isin);
            return Err(err());
        }

        if !checkers::is_ascii(isin) {
            let err = || anyhow!("Invalid ISIN code: not ascii: {:?}", isin);
            return Err(err());
        }

        Ok(IsinCode {
            isin: unsafe { *isin.as_ptr().cast::<[u8; 12]>() },
        })
    }
}

impl PartialEq<&str> for IsinCode {
    fn eq(&self, other: &&str) -> bool {
        self.isin == other.as_bytes()
    }
}

impl PartialEq<IsinCode> for &str {
    fn eq(&self, other: &IsinCode) -> bool {
        self.as_bytes() == other.isin.as_slice()
    }
}

impl PartialEq<Vec<u8>> for IsinCode {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.isin == other.as_slice()
    }
}

impl PartialEq<&[u8]> for IsinCode {
    fn eq(&self, other: &&[u8]) -> bool {
        self.isin == *other
    }
}

impl PartialEq<IsinCode> for Vec<u8> {
    fn eq(&self, other: &IsinCode) -> bool {
        *self == other.isin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_hash::FxHashMap;

    #[test]
    fn test_isin_code() -> Result<()> {
        let isin = b"KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");

        assert_eq!(isin_code, isin.to_vec());
        Ok(())
    }

    #[test]
    fn test_isin_code_invalid() {
        let isin = b"KR7005930003 ";
        let isin_code = IsinCode::new(isin);
        assert!(isin_code.is_err());
    }

    #[test]
    fn test_paraility() -> Result<()> {
        let isin = b"KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");

        assert_eq!(isin_code, isin.to_vec());
        assert_eq!(isin.to_vec(), isin_code);
        assert_eq!(isin_code, isin_code);
        Ok(())
    }

    #[test]
    fn test_hash_map() -> Result<()> {
        let isin = b"KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");

        let mut map = FxHashMap::default();
        map.insert(isin_code.clone(), 1);

        let test_key = IsinCode::new(b"KR7005930003").expect("failed to create IsinCode");
        assert_eq!(map.get(&test_key), Some(&1));
        Ok(())
    }
}
