use crate::types::venue::VenueTrait;
use crate::types::base::OrderId;
use flexstr::LocalStr;
use serde::{de::Deserializer, Deserialize, Serialize};
use ustr::Ustr;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use anyhow::{Result, anyhow};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Default, PartialEq, Eq)]
pub struct KRX;

impl VenueTrait for KRX {
    fn check_account_id(&self, _: &str) -> bool {
        unimplemented!("KRX::check_account_id")
    }
    fn check_trader_id(&self, _: &str) -> bool {
        unimplemented!("KRX::check_trader_id")
    }
    fn check_order_id(&self, _: &str) -> bool {
        unimplemented!("KRX::check_order_id")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct KrxOrderId {
    id: u64,
}

impl KrxOrderId {
    pub fn new(id: u64) -> KrxOrderId {
        KrxOrderId { id }
    }
}

impl PartialEq<u64> for KrxOrderId {
    fn eq(&self, other: &u64) -> bool {
        self.id == *other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct KrxAccountId(u64);

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct KrxTraderId {
    #[serde(deserialize_with = "from_str")]
    id: Ustr,
}

impl Serialize for KrxTraderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

fn from_str<'de, D>(deserializer: D) -> Result<Ustr, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Ustr::from(&s))
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OrderIdCounter {
    start: u64,
    end: u64,
    name: LocalStr,
}

impl OrderIdCounter {
    pub fn new(start: u64, end: u64) -> OrderIdCounter {
        let name = LocalStr::from("KrxOrderIdCounter");
        OrderIdCounter { 
            start, 
            end,
            name,
         }
    }

    #[inline]
    pub fn get_name(&self) -> &LocalStr {
        &self.name
    }

    pub fn next(&self) -> Result<OrderId> {
        static ORDER_ID_COUNTER: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
        let res = ORDER_ID_COUNTER.fetch_add(1, Ordering::SeqCst) + self.start;
        if res > self.end {
            let lazy_error = || {
                anyhow!(
                    "CounterU64: counter overflow, start: {}, end: {}",
                    self.start,
                    self.end
                )
            };
            Err(lazy_error())
        } else {
            Ok(res)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct MockOrderIdCounter {
    start: u64,
    end: u64,
    name: LocalStr,
}

impl MockOrderIdCounter {
    pub fn new(start: u64, end: u64) -> MockOrderIdCounter {
        let name = LocalStr::from("MockOrderIdCounter");
        MockOrderIdCounter { 
            start, 
            end,
            name,
         }
    }

    #[inline]
    pub fn get_name(&self) -> &LocalStr {
        &self.name
    }

    pub fn next(&self) -> Result<OrderId> {
        static ORDER_ID_COUNTER: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
        let res = ORDER_ID_COUNTER.fetch_add(1, Ordering::SeqCst) + self.start;
        if res > self.end {
            let lazy_error = || {
                anyhow!(
                    "CounterU64: counter overflow, start: {}, end: {}",
                    self.start,
                    self.end
                )
            };
            Err(lazy_error())
        } else {
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_id_counter() {
        let counter = OrderIdCounter::new(0, 100);
        for _ in 0..100 {
            let _ = counter.next().unwrap();
        }
        assert!(counter.next().is_err());
    }

    #[test]
    fn multi_thread_id_counter() -> Result<()> {
        let mut handles = vec![];
        for _ in 0..5 {
            let handle = std::thread::spawn(move || {
                let counter = OrderIdCounter::new(0, 100);
                let _ = counter.next().unwrap();
                let _ = counter.next().unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        let res = OrderIdCounter::new(0, 100).next()?;
        assert_eq!(res, 10);
        Ok(())
    }
}