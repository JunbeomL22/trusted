use crate::types::venue::VenueTrait;
use serde::{de::Deserializer, Deserialize, Serialize};
use ustr::Ustr;

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
