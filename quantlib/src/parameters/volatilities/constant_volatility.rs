use crate::parameters::volatilities::volatility::VolatilityTrait;
use crate::definitions::{Real, Time};

pub struct ConstantVolatility {
    value: Real,
    name: String,
    code: String,
}

impl ConstantVolatility {
    pub fn new(value: Real, name: &str, code: &str) -> ConstantVolatility {
        ConstantVolatility {
            value,
            name: name.to_string(),
            code: code.to_string(),
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