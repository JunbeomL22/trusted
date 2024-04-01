use crate::definitions::Real;
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;

pub trait VolatilityInterplatorTrait {
    fn interpolate(&self) -> Real {0.0}
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AndreasenHuge {}
impl VolatilityInterplatorTrait for AndreasenHuge {
    fn interpolate(&self) -> Real {
        0.0
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Dupire {}
impl VolatilityInterplatorTrait for Dupire {
    fn interpolate(&self) -> Real {
        0.0
    }
}

#[enum_dispatch(VolatilityInterplatorTrait)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolatilityInterplator {
    AndreasenHuge(AndreasenHuge),
    Dupire(Dupire),
}

impl Default for VolatilityInterplator {
    fn default() -> VolatilityInterplator {
        VolatilityInterplator::AndreasenHuge(AndreasenHuge {})
    }
}