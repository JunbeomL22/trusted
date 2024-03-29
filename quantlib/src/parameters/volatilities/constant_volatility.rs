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

impl Default for ConstantVolatility {
    fn default() -> ConstantVolatility {
        ConstantVolatility {
            value: 0.0,
            name: "".to_string(),
            code: "".to_string(),
        }
    }
}

impl VolatilityTrait for ConstantVolatility {
    fn get_value(&self, _t: Time, _x: Real) -> Real {
        self.value
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn total_variance(&self, t: Time, _x: Real) -> Real {
        self.value * self.value * t
    }

    fn total_deviation(&self, t: Time, _x: Real) -> Real {
        self.value * t.sqrt()
    }
}