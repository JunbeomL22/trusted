use serde::{
    Serialize, 
    Deserialize,
    de::Deserializer,
};
use crate::types::venue::VenueTrait;
use ustr::Ustr;
use anyhow::{Result, anyhow};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Mock;

impl VenueTrait for Mock {
    fn check_account_id(&self, _: &str) -> bool { true }
    fn check_trader_id(&self, _: &str) -> bool { true }
    fn check_order_id(&self, _: &str) -> bool { true }
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct MockOrderId {
    id: u64,
}

impl MockOrderId {
    pub fn new(id: u64) -> MockOrderId {
        MockOrderId { id }
    }
}

impl PartialEq<u64> for MockOrderId {
    fn eq(&self, other: &u64) -> bool {
        self.id == *other
    }
}


impl Default for MockOrderId {
    fn default() -> Self {
        MockOrderId {
            id: 0,
        }
    }
}


impl Serialize for MockOrderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

fn order_id_from_str<'de, D>(deserializer: D) -> Result<Ustr, D::Error>
where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Ustr::from(&*s))
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct MockAccountId(u64);

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct MockTraderId { 
    #[serde(deserialize_with = "account_id_from_str")]
    id: Ustr,
}

impl Serialize for MockTraderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

fn account_id_from_str<'de, D>(deserializer: D) -> Result<Ustr, D::Error>
where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Ustr::from(&*s))
}
