use enum_dispatch::enum_dispatch;
use crate::definitions::Real;

pub trait VolatilityInterplatorTrait {
    fn interpolate(&self) -> Real {0.0}
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AndreasenHuge {}
impl VolatilityInterplatorTrait for AndreasenHuge {
    fn interpolate(&self) -> Real {
        0.0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dupire {}
impl VolatilityInterplatorTrait for Dupire {
    fn interpolate(&self) -> Real {
        0.0
    }
}

#[enum_dispatch(VolatilityInterplatorTrait)]
#[derive(Debug, Clone, PartialEq)]
pub enum VolatilityInterplator {
    AndreasenHuge(AndreasenHuge),
    Dupire(Dupire),
}

impl Default for VolatilityInterplator {
    fn default() -> VolatilityInterplator {
        VolatilityInterplator::AndreasenHuge(AndreasenHuge {})
    }
}