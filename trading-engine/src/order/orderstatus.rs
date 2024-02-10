//use serde::{Deserialize, Deserializer, Serialize, Serializer};
//use strum::{EnumString, EnumVariantNames, EnumIter, IntoStaticStr};
#[derive(
    Debug, 
    PartialEq, 
    Eq,
    Clone,
    Copy,
    Hash)]
pub enum OrderStatus {
    NoStatus = 0,
    Initialized = 1,
    Accepted = 2,
    Rejected = 3,
    PartiallyFilled = 4,
    Filled = 5,
}