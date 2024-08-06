use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Result};
use std::str::from_utf8;
use std::hash::{Hash, Hasher};

/// The code length is not fixed. So, it may be tempted to use a Vec<u8> or String.
/// However, most of such empty container uses 24 bytes of memory when empty.
/// Hence, there is no benefit of using Vec<u8> or String. Instead, we allocate 32 bytes in advance.
/// The remaining part (right side) will be set to 0.
/// If we encounter a situation where the code length is longer than 32 bytes, we will have to change the code.
/// In the end, the Ticker will dwell on only in data area. Ultimately, we will use ID struct that is basically a pointer to
/// { (Symbol, Venue) }, where Symbol is an Enum type of IsinCode and Ticker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    ticker: [u8; 32],
}

impl Default for Ticker {
    fn default() -> Self {
        Ticker { ticker: [0u8; 32] }
    }
}

impl Ticker {
    pub fn new(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > 32 {
            let err = || anyhow!("Ticker(={}) length should be less than 32", from_utf8(bytes));
            return Err(err());
        }
        let mut ticker = [0u8; 32];
        ticker[..bytes.len()].copy_from_slice(bytes);
        Ok(Ticker { ticker })
    }
}

impl PartialEq for Ticker {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker
    }
}

impl Eq for Ticker {}

impl Hash for Ticker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ticker.hash(state);
    }
}
