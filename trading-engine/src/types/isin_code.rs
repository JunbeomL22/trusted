use crate::utils::checkers;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::ptr::eq as ptr_eq;
use rustc_hash::FxHashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Deserializer, Serializer};

lazy_static! {
    static ref ISIN_CACHE: Mutex<FxHashMap<[u8; 12], &'static [u8; 12]>> = Mutex::new(FxHashMap::default());
}

#[derive(Debug, Clone)]
pub struct IsinCode {
    //isin: [u8; 12],
    isin: &'static [u8; 12],
}

impl Default for IsinCode {
    fn default() -> Self {
        IsinCode { isin: b"000000000000" }
    }
}

impl Serialize for IsinCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = std::str::from_utf8(self.isin).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for IsinCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = s.as_bytes();
        if checkers::contains_white_space(bytes) {
            return Err(serde::de::Error::custom("contains white space"));
        }

        if !checkers::is_ascii(bytes) {
            return Err(serde::de::Error::custom("not ascii"));
        }
        if bytes.len() != 12 {
            return Err(serde::de::Error::custom("length should be 12"));
        }

        let mut code = [0u8; 12];
        code.copy_from_slice(bytes);

        let mut cache = ISIN_CACHE.lock().unwrap();
        let interned = cache.entry(code).or_insert_with(|| {
            let boxed = Box::leak(Box::new(code));
            boxed
        });

        Ok(IsinCode { isin: *interned })
    }
}

impl PartialEq for IsinCode {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self.isin, other.isin)
    }
}

impl Eq for IsinCode {}

impl Hash for IsinCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.isin as *const [u8; 12]).hash(state);
    }
}

impl IsinCode {
    pub fn new(bytes: &[u8]) -> Result<Self> {
        if checkers::contains_white_space(bytes) {
            let err = || anyhow!("Invalid ISIN code: contains white space: {:?}", bytes);
            return Err(err());
        }

        if !checkers::is_ascii(bytes) {
            let err = || anyhow!("Invalid ISIN code: not ascii: {:?}", bytes);
            return Err(err());
        }

        if bytes.len() != 12 {
            let err = || anyhow!("Invalid ISIN code: length should be 12: {:?}", bytes);
            return Err(err());
        }
        let mut code = [0u8; 12];
        code.copy_from_slice(bytes);

        let mut cache = ISIN_CACHE.lock().unwrap();
        let interned = cache.entry(code).or_insert_with(|| {
            let boxed = Box::leak(Box::new(code));
            boxed
        });

        Ok(IsinCode { isin: *interned })
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        self.isin
    }

    pub fn as_str(&self) -> &str {
        // This is safe because we know ISINs are always valid UTF-8
        unsafe { std::str::from_utf8_unchecked(self.isin) }
    }

    pub fn starts_with(&self, prefix: &[u8]) -> bool {
        self.isin.starts_with(prefix)
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
    use crate::utils::memory_investigation::print_struct_info;


    pub struct DirectIsin([u8; 12]);

    #[test]
    fn test_isin_code_size() {
        let isin_code = IsinCode::default();
        let direct_isin_code = DirectIsin([0u8; 12]);
        println!("IsinCode size: {}", std::mem::size_of_val(&isin_code));
        print_struct_info(isin_code);

        println!("DirectIsin size: {}", std::mem::size_of_val(&direct_isin_code));
        print_struct_info(direct_isin_code);

        assert!(true);
    }
    #[test]
    fn test_isin_code() -> Result<()> {
        let isin = b"KR7005930003";
        let isin_code = IsinCode::new(isin).expect("failed to create IsinCode");

        assert_eq!(isin_code.as_bytes(), isin);
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
