use crate::assets::currency::Currency;
use crate::definitions::Real;
//
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use time::OffsetDateTime;
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct FxCode {
    currency1: Currency,
    currency2: Currency,
}

impl FxCode {
    pub fn new(currency1: Currency, currency2: Currency) -> FxCode {
        FxCode {
            currency1,
            currency2,
        }
    }

    pub fn get_currency1(&self) -> &Currency {
        &self.currency1
    }

    pub fn get_currency2(&self) -> &Currency {
        &self.currency2
    }
    
    pub fn reciprocal(self) -> Self {
        FxCode {
            currency1: self.currency2,
            currency2: self.currency1,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.currency1.as_str(), self.currency2.as_str())
    }
}

impl From<&str> for FxCode {
    fn from(code: &str) -> FxCode {
        let currency1 = Currency::from(&code[0..3]);
        let currency2 = Currency::from(&code[3..6]);

        FxCode {
            currency1,
            currency2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FX {
    rate: Real,
    market_datetime: OffsetDateTime,
    code: FxCode,
}

impl FX {
    pub fn new(rate: Real, code: FxCode, market_datetime: OffsetDateTime) -> FX {
        FX {
            rate,
            market_datetime,
            code,
        }
    }

    pub fn new_from_str(rate: Real, code: &str, market_datetime: OffsetDateTime) -> Result<FX> {
        if code.len() != 6 {
            return Err(anyhow!("({}:{}) {} is an invalid FX code", file!(), line!(), code));
        }

        let code = FxCode::from(code);

        Ok(FX {
            rate,
            market_datetime,
            code,
        })
    }

    pub fn get_rate(&self) -> Real {
        self.rate
    }

    
    pub fn get_code(&self) -> &FxCode {
        &self.code
    }

    pub fn reciprocal(self) -> Self {
        FX {
            rate: 1.0 / self.rate,
            market_datetime: self.market_datetime.clone(),
            code: self.code.reciprocal(),
        }
    }
}

impl AddAssign<Real> for FX {
    fn add_assign(&mut self, rhs: Real) {
        self.rate += rhs;
    }
}

impl SubAssign<Real> for FX {
    fn sub_assign(&mut self, rhs: Real) {
        self.rate -= rhs;
    }
}

impl MulAssign<Real> for FX {
    fn mul_assign(&mut self, rhs: Real) {
        self.rate *= rhs;
    }
}

impl DivAssign<Real> for FX {
    fn div_assign(&mut self, rhs: Real) {
        self.rate /= rhs;
    }
}


