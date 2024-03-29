use crate::parameters::volatility::VolatilityTrait;
use crate::definitions::{Real, Time};
use serde::{Serialize, Deserialize};
use anyhow::Result;
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

    fn bump_volatility(
        &mut self, 
        _time1: Option<Time>,
        _time2: Option<Time>,
        _left_spot_moneyness: Option<Real>,
        _right_spot_moneyness: Option<Real>,
        bump: Real
    ) -> Result<()> {
        self.value += bump;
        Ok(())
    }
}