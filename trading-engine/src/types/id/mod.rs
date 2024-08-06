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
    Serializer,
};
use std::hash::{Hash, Hasher};
use std::ptr::eq as ptr_eq;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Symbol {
    Isin(IsinCode),
    Ticker(Ticker),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdData {
    pub symbol: Symbol,
    pub venue: Venue,
}

lazy_static! {
    static ref ID_CACHE: Mutex<FxHashMap<IdData, &'static IdData>> = Mutex::new(FxHashMap::default());
}

/// ID is a pointer to { (Symbol, Venue) }
#[derive(Debug, Clone, Serialize)]
pub struct ProdId {
    id_ptr: &'static IdData,
}

impl<'de> Deserialize<'de> for ProdId {
    fn deserialize<D>(deserializer: D) -> Result<ProdId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id_data = IdData::deserialize(deserializer)?;
        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(id_data.clone()).or_insert_with(|| Box::leak(Box::new(id_data.clone()));

        Ok(ProdId { id_ptr: *interned })
    }
}

impl ProdId {
    pub fn new(symbol: Symbol, venue: Venue) -> Self {
        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(IdData { symbol: symbol.clone(), venue }).or_insert_with(|| Box::leak(Box::new(IdData { symbol, venue })));

        ProdId { id_ptr: *interned }
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
}

impl PartialEq for ProdId {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self.id_ptr, other.id_ptr)
    }
}

impl Eq for ProdId {}

impl Hash for ProdId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id_ptr.hash(state);
    }
}