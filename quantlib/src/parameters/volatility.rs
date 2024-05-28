use crate::parameters::volatilities::{
    constant_volatility::ConstantVolatility,
    local_volatility_surface::LocalVolatilitySurface,
};
use crate::definitions::{Real, Time};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VolatilityType {
    ConstantVolatility,
    LocalVolatilitySurface,
}

pub trait VolatilityTrait {
    fn get_value(&self, t: Time, forward_moneyness: Real) -> Real;
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn total_variance(&self, t: Time, forward_moneyness: Real) -> Result<Real>;
    fn total_deviation(&self, t: Time, forward_moneyness: Real) -> Result<Real>;
    fn bump_volatility(
        &mut self, 
        time1: Option<Time>,
        time2: Option<Time>,
        left_spot_moneyness: Option<Real>,
        right_spot_moneyness: Option<Real>,
        bump: Real
    ) -> Result<()>;
    
    fn get_local_volatility(&self, t: Time, forward_moneyness: Real) -> Real {
        self.get_value(t, forward_moneyness)
    }
}

#[derive(Debug, Clone)]
pub enum Volatility {
    ConstantVolatility(ConstantVolatility),
    LocalVolatilitySurface(LocalVolatilitySurface),
}

impl Volatility {
    pub fn get_name(&self) -> &String {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.get_name(),
            Volatility::LocalVolatilitySurface(volatility) => volatility.get_name(),
        }
    }

    pub fn get_code(&self) -> &String {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.get_code(),
            Volatility::LocalVolatilitySurface(volatility) => volatility.get_code(),
        }
    }

    pub fn get_value(&self, t: Time, forward_moneyness: Real) -> Real {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.get_value(t, forward_moneyness),
            Volatility::LocalVolatilitySurface(volatility) => volatility.get_value(t, forward_moneyness),
        }
    }

    pub fn total_variance(&self, t: Time, forward_moneyness: Real) -> Result<Real> {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.total_variance(t, forward_moneyness),
            Volatility::LocalVolatilitySurface(volatility) => volatility.total_variance(t, forward_moneyness),
        }
    }

    pub fn total_deviation(&self, t: Time, forward_moneyness: Real) -> Result<Real> {
        match self {
            Volatility::ConstantVolatility(volatility) => volatility.total_deviation(t, forward_moneyness),
            Volatility::LocalVolatilitySurface(volatility) => volatility.total_deviation(t, forward_moneyness),
        }
    }

    pub fn build(&mut self) -> Result<()> {
        match self {
            Volatility::ConstantVolatility(_volatility) => {
                Ok(())
            }
            Volatility::LocalVolatilitySurface(volatility) => {
                volatility.build()?;
                Ok(())
            }
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
            Volatility::LocalVolatilitySurface(volatility) => volatility.bump_volatility(time1, time2, left_spot_moneyness, right_spot_moneyness, bump),
        }
    }

    pub fn get_volatility_type(&self) -> VolatilityType {
        match self {
            Volatility::ConstantVolatility(_) => VolatilityType::ConstantVolatility,
            Volatility::LocalVolatilitySurface(_) => VolatilityType::LocalVolatilitySurface,
        }
    }
}