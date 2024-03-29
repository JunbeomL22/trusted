use crate::parameters::volatilities::constant_volatility::ConstantVolatility;
use crate::definitions::{Real, Time};
use anyhow::Result;

pub trait VolatilityTrait {
    fn get_value(&self, t: Time, forward_moneyness: Real) -> Real;
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn total_variance(&self, t: Time, forward_moneyness: Real) -> Real;
    fn total_deviation(&self, t: Time, forward_moneyness: Real) -> Real;
    fn bump_volatility(
        &mut self, 
        time1: Option<Time>,
        time2: Option<Time>,
        left_spot_moneyness: Option<Real>,
        right_spot_moneyness: Option<Real>,
        bump: Real
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum Volatility {
    ConstantVolatility(ConstantVolatility),
}

impl Volatility {
    pub fn get_name(&self) -> &String {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.get_name(),
        }
    }

    pub fn get_code(&self) -> &String {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.get_code(),
        }
    }

    pub fn get_value(&self, t: Time, forward_moneyness: Real) -> Real {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.get_value(t, forward_moneyness),
        }
    }

    pub fn total_variance(&self, t: Time, forward_moneyness: Real) -> Real {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.total_variance(t, forward_moneyness),
        }
    }

    pub fn total_deviation(&self, t: Time, forward_moneyness: Real) -> Real {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.total_deviation(t, forward_moneyness),
        }
    }

    pub fn bump_volatility(
        &mut self, 
        time1: Option<Time>,
        time2: Option<Time>,
        left_spot_moneyness: Option<Real>,
        right_spot_moneyness: Option<Real>,
        bump: Real,
    ) -> Result<()> {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.bump_volatility(time1, time2, left_spot_moneyness, right_spot_moneyness, bump),
        }
    }
}