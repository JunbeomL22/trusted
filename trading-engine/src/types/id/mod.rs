pub mod isin_code;
pub mod ticker;

use crate::types::venue::Venue;
use crate::types::id::isin_code::IsinCode;
use crate::types::id::ticker::Ticker;
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

    static ref DEFAULT_ID_PTR: &'static IdData = ID_CACHE.lock().unwrap().get(&IdData::default()).unwrap();
}

/// ID is a pointer to { (Symbol, Venue) }
#[derive(Clone, Serialize, Copy)]
pub struct InstId {
    #[serde(flatten)]
    id_ptr: &'static IdData,
}

impl std::fmt::Debug for InstId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstId")
            .field("symbol", &self.id_ptr.symbol)
            .field("venue", &self.id_ptr.venue)
            .finish()
    }
}

impl Default for InstId {
    fn default() -> Self {
        InstId { id_ptr: *DEFAULT_ID_PTR }
    }
}

impl<'de> Deserialize<'de> for InstId {
    fn deserialize<D>(deserializer: D) -> Result<InstId, D::Error>
    where D: Deserializer<'de>,
    {
        let id_data = IdData::deserialize(deserializer)?;
        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(id_data.clone()).or_insert_with(|| Box::leak(Box::new(id_data.clone())));

        Ok(InstId { id_ptr: *interned })
    }
}

impl InstId {
    pub fn new(symbol: Symbol, venue: Venue) -> Self {
        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(IdData { symbol: symbol.clone(), venue }).or_insert_with(|| Box::leak(Box::new(IdData { symbol, venue })));

        InstId { id_ptr: *interned }
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

impl PartialEq for InstId {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self.id_ptr, other.id_ptr)
    }
}

impl Eq for InstId {}

impl Hash for InstId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id_ptr.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::venue::Venue;
    use serde_json;
    use anyhow::Result;

    #[test]
    fn test_prod_id() {
        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id = InstId::new(symbol, venue);

        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id2 = InstId::new(symbol, venue);
        assert_eq!(prod_id, prod_id2);


        let ticker = Ticker::new(b"005930").unwrap();
        let symbol = Symbol::Ticker(ticker);
        let venue = Venue::KRX;
        let prod_id3 = InstId::new(symbol, venue);
        assert_ne!(prod_id, prod_id3);
        // check length of ID_CACHE
        let cache = ID_CACHE.lock().unwrap();
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn serialize() -> Result<()> {
        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id = InstId::new(symbol, venue);

        let serialized = serde_json::to_string(&prod_id).unwrap();
        println!("{}", &serialized);
        let deserialized: InstId = serde_json::from_str(&serialized).unwrap();

        assert_eq!(prod_id, deserialized);

        let ticker = Ticker::new(b"005930").unwrap();
        let symbol = Symbol::Ticker(ticker);
        let venue = Venue::KRX;
        let prod_id = InstId::new(symbol, venue);

        let serialized = serde_json::to_string(&prod_id).unwrap();
        println!("{}", &serialized);
        let deserialized: InstId = serde_json::from_str(&serialized).unwrap();

        assert_eq!(prod_id, deserialized);

        Ok(())
    }

    #[test]
    fn hashmap_test() {
        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id = InstId::new(symbol, venue);

        let isin = IsinCode::new(b"US0378331005").unwrap();
        let symbol = Symbol::Isin(isin);
        let venue = Venue::KRX;
        let prod_id2 = InstId::new(symbol, venue);
        assert_eq!(prod_id, prod_id2);

        let ticker = Ticker::new(b"005930").unwrap();
        let symbol = Symbol::Ticker(ticker);
        let venue = Venue::KRX;
        let prod_id3 = InstId::new(symbol, venue);
        assert_ne!(prod_id, prod_id3);
        // check length of ID_CACHE
        let cache = ID_CACHE.lock().unwrap();
        assert_eq!(cache.len(), 2);

        let mut map = FxHashMap::default();
        map.insert(prod_id.clone(), 1);
        map.insert(prod_id3.clone(), 2);

        let test_key = InstId::new(Symbol::Isin(IsinCode::new(b"US0378331005").unwrap()), Venue::KRX);
        assert_eq!(map.get(&test_key), Some(&1));

        let test_key = InstId::new(Symbol::Ticker(Ticker::new(b"005930").unwrap()), Venue::KRX);
        assert_eq!(map.get(&test_key), Some(&2));

    }
}