use crate::types::venues::{
    krx::{
        KRX,
        KrxTraderId,
        KrxAccountId,
    },
    mock_exchange::{
        Mock,
        MockTraderId,
        MockAccountId,
    },
};
use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
#[enum_dispatch(VenueTrait)]
pub enum Venue {
    Mock(Mock),
    KRX(KRX),
}

#[enum_dispatch]
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