use crate::types::id::isin_code::IsinCode;
use crate::types::base::Real;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub type AccountId = u64;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub isin: IsinCode,
    pub balance: f64,
}