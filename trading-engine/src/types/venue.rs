use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
pub enum Venue {
    Undefined,
    CCP,
    #[default]
    KRX,
    KIS,
    SI,
    OTC,
}

impl Venue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Venue::Undefined => "Undefined",
            Venue::CCP => "CCP",
            Venue::KRX => "KRX",
            Venue::KIS => "KIS",
            Venue::SI => "SI",
            Venue::OTC => "OTC",
        }
    }
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
