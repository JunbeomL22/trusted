use crate::parameters::volatilities::constant_volatility::ConstantVolatility
use crate::definitions::{Real, Time};
use enum_dispatch::enum_dispatch;


#[enum_dispatch]
pub trait VolatilityTrait {
    fn value(&self, t: Time, forward_moneyness: Real) -> Real;
    fn name(&self) -> &String;
    fn code(&self) -> &String;
    fn total_variance(&self, t: Time, forward_moneyness: Real) -> Real;
    fn total_deviation(&self, t: Time, forward_moneyness: Real) -> Real;
}

#[enum_dispatch(VolatilityTrait)]
pub enum Volatility {
    ConstantVolatility(ConstantVolatility),
}