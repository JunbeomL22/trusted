use crate::types::venues::{
    krx::KRX,
    mock::Mock,
};
use enum_dispatch::enum_dispatch;


#[derive(Debug, Clone, Copy)]
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
}