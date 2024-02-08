#[derive(
    Debug, 
    PartialEq, 
    Eq,
    Clone,
    Copy,
    Eq,
    Hash,
    Serialize,
    Deserialize,)]
pub enum OrderStatus {
    NoStatus = 0,
    Initialized = 1,
    Accepted = 2,
    Rejected = 3,
    PartiallyFilled = 4,
    Filled = 5,
}