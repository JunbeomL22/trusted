use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
pub enum Venue {
    Undefined,
    #[default]
    KRX,
}

pub trait VenueTrait {
    fn check_account_id(&self, _: &str) -> bool {
        unimplemented!("check_account not implemented")
    }

    fn check_trader_id(&self, _: &str) -> bool {
        unimplemented!("check_trader_id not implemented")
    }

    fn check_order_id(&self, _: &str) -> bool {
        unimplemented!("check_order_id not implemented")
    }
}
