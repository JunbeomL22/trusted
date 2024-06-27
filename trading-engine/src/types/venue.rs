use crate::types::venues::{
    krx::{
        KRX,
        KrxOrderId,
        KrxTraderId,
        KrxAccountId,
    },
    mock_exchange::{
        Mock,
        MockOrderId,
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

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum OrderId {
    MockOrderId(MockOrderId),
    KrxOrderId(KrxOrderId),
}

impl PartialEq<u64> for OrderId {
    fn eq(&self, other: &u64) -> bool {
        match self {
            OrderId::MockOrderId(id) => id == other,
            OrderId::KrxOrderId(id) => id == other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum AccountId {
    MockAccountId(MockAccountId),
    KrxAccountId(KrxAccountId),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum TraderId {
    MockTraderId(MockTraderId),
    KrxTraderId(KrxTraderId),
}

