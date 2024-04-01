use crate::currency::Currency;
use crate::definitions::Real;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    previous_value: Real,
    portfolio: HashMap<String, Real>,
    name: String,
    code: String,
    currency: Currency,
}

pub struct ETF {
    previous_value: Real,
    portfolio: HashMap<String, Real>,
    name: String,
    code: String,
    currency: Currency,
}