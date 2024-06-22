use serde::{Serialize, Deserialize};
use crate::types::venue::VenueTrait;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Mock;

impl VenueTrait for Mock {
    fn check_account_id(&self, _: &str) -> bool { true }
    fn check_trader_id(&self, _: &str) -> bool { true }
}