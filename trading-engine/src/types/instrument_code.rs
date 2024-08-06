use lazy_static::lazy_static;
use std::sync::Mutex;
use rustc_hash::FxHashMap;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref CODE_CACHE: Mutex<FxHashMap<&'static str, &'static str>> = Mutex::new(FxHashMap::default());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentCode {
    code: &'static str,
}

impl Default for InstrumentCode {
    fn default() -> Self {
        InstrumentCode { code: "Defualt" }
    }
}

impl InstrumentCode {
    pub fn new(code: &[u8]) -> Self {
        let mut cache = CODE_CACHE.lock().unwrap();
        let ptr_str = std::str::from_utf8(code).unwrap();
        let interned = cache.entry(ptr_str).or_insert_with(|| {
            let boxed = Box::leak(Box::new(ptr_str));
            boxed
        });

        InstrumentCode {
            code: ptr_str,
        }
    }
}
