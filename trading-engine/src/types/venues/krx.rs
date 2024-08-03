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
