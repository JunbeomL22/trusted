use crate::types::venue::VenueTrait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KRX;

impl VenueTrait for KRX {
    fn check_account_id(&self, _: &str) -> bool { unimplemented!("KRX::check_account_id") }
    fn check_trader_id(&self, _: &str) -> bool { unimplemented!("KRX::check_trader_id") }
}