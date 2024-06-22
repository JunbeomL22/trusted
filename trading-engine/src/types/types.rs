use ustr::Ustr;
use serde::{
    Serialize, 
    Deserialize,
    de::Deserializer,
};

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderId {
    #[serde(deserialize_with = "from_str")]
    id: Ustr,
}

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraderId {
    #[serde(deserialize_with = "from_str")]
    id: Ustr,
}

impl Serialize for OrderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

impl Serialize for TraderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

fn from_str<'de, D>(deserializer: D) -> Result<Ustr, D::Error>
where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Ustr::from(&*s))
}

