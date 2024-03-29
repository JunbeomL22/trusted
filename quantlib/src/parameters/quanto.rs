use crate::assets::fx::FxCode;
use crate::parameters::{
    volatility::{
        Volatility,
        VolatilityTrait,
    },
    volatilities::constant_volatility::ConstantVolatility,
};
use crate::definitions::{Time, Real};
use serde::{forward_to_deserialize_any, Deserialize, Serialize};
use std::{
    rc::Rc,
    cell::RefCell,
};


/// Quanto parameter. 
/// It is assumed that the correlation are constant.
#[derive(Debug, Clone)]
pub struct Quanto {
    fx_volatility: Rc<RefCell<Volatility>>,
    correlation: Real,
    fx_code: FxCode,
    underlying_code: String,
}

impl Quanto {
    pub fn new(
        fx_volatility: Rc<RefCell<Volatility>>, 
        correlation: Real, 
        fx_code: FxCode,
        underlying_code: String,
    ) -> Quanto {
        Quanto {
            fx_volatility,
            correlation,
            fx_code,
            underlying_code,
        }
    }

    pub fn quanto_adjust(
        &self, 
        t: Time,
        forward_moneyness: Real,
    ) -> Real {
        self.fx_volatility.borrow().get_value(t, forward_moneyness) * self.correlation
    }

    pub fn get_underlying_code(&self) -> &String {
        &self.underlying_code
    }

    pub fn get_fx_code(&self) -> &FxCode {
        &self.fx_code
    }
}

impl Default for Quanto {
    fn default() -> Quanto {
        Quanto {
            fx_volatility: Rc::new(RefCell::new(
                Volatility::ConstantVolatility(ConstantVolatility::default())
            )),
            correlation: 0.0,
            fx_code: FxCode::default(),
            underlying_code: "".to_string(),
        }
    }
}