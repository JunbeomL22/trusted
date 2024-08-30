pub mod isin_code;
pub mod ticker;

use crate::instruments::id::isin_code::IsinCode;
use crate::instruments::id::ticker::Ticker;
use rustc_hash::FxHashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{
    Deserialize, 
    Serialize,
    Deserializer,
};
use std::hash::{Hash, Hasher};
use std::ptr::eq as ptr_eq;
use once_cell::sync::Lazy;

/// Venue means the first channel of the market
/// For example, if we trade with brocker KIS, the venue is KIS
/// The product-specific venue like CME, EUREX, etc are ignored.
/// If we trade thru DMA given by KRX, then the venue is KRX
/// Likewise, KRX-CCP is just KRX
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Copy)]
pub enum Venue {
    #[default]
    KRX,
    KoreaInvSec,
    SI,
    KAP,
    KIS, 
    NICE,
    FNP,
    Undefined,
}

//
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Symbol {
    Isin(IsinCode),
    Ticker(Ticker),
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Isin(IsinCode::default())
    }
}
//
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct IdData {
    pub symbol: Symbol,
    pub venue: Venue,
}

lazy_static! {
    static ref ID_CACHE: Mutex<FxHashMap<IdData, &'static IdData>> = {
        let mut map = FxHashMap::default();
        let default_id_data = IdData::default();
        
        let id_ptr: &'static IdData = Box::leak(Box::new(default_id_data.clone()));
        
        map.insert(default_id_data, id_ptr);

        Mutex::new(map)
    };
}

/// ID is a pointer to { (Symbol, Venue) }
#[derive(Clone, Serialize, Copy)]
pub struct ID {
    #[serde(flatten)]
    id_ptr: &'static IdData,
}

impl std::fmt::Debug for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ID")
            .field("symbol", &self.id_ptr.symbol)
            .field("venue", &self.id_ptr.venue)
            .finish()
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id_ptr.symbol {
            Symbol::Isin(isin) => writeln!(f, "{:?}", isin),
            Symbol::Ticker(ticker) => writeln!(f, "{:?}", ticker),
        }?;
        writeln!(f, "{:?}", self.id_ptr.venue)?;

        Ok(())
    }
}

impl Default for ID {
    fn default() -> Self {
        static DEFAULT_ID_PTR: Lazy<&'static IdData> = Lazy::new(|| {
            let cache = ID_CACHE.lock().unwrap();
            cache.get(&IdData::default()).unwrap()
        });

        ID { id_ptr: *DEFAULT_ID_PTR }
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<ID, D::Error>
    where D: Deserializer<'de>,
    {
        let id_data = IdData::deserialize(deserializer)?;
        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(id_data.clone()).or_insert_with(|| Box::leak(Box::new(id_data.clone())));

        Ok(ID { id_ptr: *interned })
    }
}

impl ID {
    pub fn new(symbol: Symbol, venue: Venue) -> Self {
        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(IdData { symbol: symbol.clone(), venue }).or_insert_with(|| Box::leak(Box::new(IdData { symbol, venue })));

        ID { id_ptr: *interned }
    }

    #[inline]
    #[must_use]
    pub fn get_id_clone(&self) -> IdData {
        self.id_ptr.clone()
    }

    #[inline]
    #[must_use]
    pub fn get_id(&self) -> &'static IdData {
        self.id_ptr
    }

    pub fn symbol_str(&self) -> &str {
        match &self.id_ptr.symbol {
            Symbol::Isin(isin) => isin.as_str(),
            Symbol::Ticker(ticker) => ticker.as_str(),
        }
    }

    pub fn venue(&self) -> Venue {
        self.id_ptr.venue
    }
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self.id_ptr, other.id_ptr)
    }
}

impl Eq for ID {}

impl Hash for ID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id_ptr.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use anyhow::Result;

    #[test]
    fn test_prod_id() {
        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id = ID::new(symbol, venue);

        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id2 = ID::new(symbol, venue);
        assert_eq!(prod_id, prod_id2);


        let ticker = Ticker::new(b"005930").unwrap();
        let symbol = Symbol::Ticker(ticker);
        let venue = Venue::KRX;
        let prod_id3 = ID::new(symbol, venue);
        assert_ne!(prod_id, prod_id3);
        // check length of ID_CACHE
        let cache = ID_CACHE.lock().unwrap();
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn serialize() -> Result<()> {
        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id = ID::new(symbol, venue);

        let serialized = serde_json::to_string(&prod_id).unwrap();
        let deserialized: ID = serde_json::from_str(&serialized).unwrap();

        assert_eq!(prod_id, deserialized);

        let ticker = Ticker::new(b"005930").unwrap();
        let symbol = Symbol::Ticker(ticker);
        let venue = Venue::KRX;
        let prod_id = ID::new(symbol, venue);

        let serialized = serde_json::to_string(&prod_id).unwrap();
        let deserialized: ID = serde_json::from_str(&serialized).unwrap();

        assert_eq!(prod_id, deserialized);

        Ok(())
    }

    #[test]
    fn hashmap_test() {
        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id = ID::new(symbol, venue);

        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id2 = ID::new(symbol, venue);
        assert_eq!(prod_id, prod_id2);

        let ticker = Ticker::new(b"005930").unwrap();
        let symbol = Symbol::Ticker(ticker);
        let venue = Venue::KRX;
        let prod_id3 = ID::new(symbol, venue);
        assert_ne!(prod_id, prod_id3);
        // check length of ID_CACHE
        let cache = ID_CACHE.lock().unwrap();
        assert_eq!(cache.len(), 3);
        drop(cache);

        let mut map = FxHashMap::default();
        map.insert(prod_id.clone(), 1);
        map.insert(prod_id3.clone(), 2);

        let test_key = ID::new(Symbol::Isin(IsinCode::new(b"US0378331005").unwrap()), Venue::KRX);
        assert_eq!(map.get(&test_key), Some(&1));

        let test_key = ID::new(Symbol::Ticker(Ticker::new(b"005930").unwrap()), Venue::KRX);
        assert_eq!(map.get(&test_key), Some(&2));

    }
}