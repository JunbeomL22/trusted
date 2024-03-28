use crate::parameters::volatilities::volatility::VolatilityTrait;
use crate::definitions::{Real, Time};
use serde::{Serialize, Deserialize};
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantVolatility {
    value: Real,
    name: String,
    code: String,
}

impl ConstantVolatility {
    pub fn new(value: Real, name: String, code: String) -> ConstantVolatility {
        ConstantVolatility {
            value,
            name,
            code,
        }
    }
}

impl VolatilityTrait for ConstantVolatility {
    fn value(&self, _t: Time, _x: Real) -> Real {
        self.value
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn code(&self) -> &String {
        &self.code
    }

    fn total_variance(&self, t: Time, _x: Real) -> Real {
        self.value * t
    }

    fn total_deviation(&self, t: Time, _x: Real) -> Real {
        self.value * t.sqrt()
    }
}