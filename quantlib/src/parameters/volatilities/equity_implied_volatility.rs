use crate::definitions::{Time, Real};
use crate::pricing_engines::equity_futures_pricer::EquityFuturesPricer;
use crate::enums::StickynessType;
use ndarray::{Array1, Array2};

#[derive(Clone, Debug)]
pub struct EquityImpliedVolatiltiySurface {
    initial_underlying_spot: Real,
    expiries: Array1<Time>,
    spot_moneyness: Array1<Real>,
    volatility_on_spot: Array1<Real>,
}

impl EquityImpliedVolatiltiySurface {
    pub fn new(
        initial_underlying_spot: Real,
        expiries: Array1<Time>,
        spot_moneyness: Array1<Real>,
        volatility_on_spot: Array1<Real>,
    ) -> EquityImpliedVolatiltiySurface {
        EquityImpliedVolatiltiySurface {
            initial_underlying_spot,
            expiries,
            spot_moneyness,
            volatility_on_spot,
        }
    }

    pub fn get_initial_underlying_spot(&self) -> Real {
        self.initial_underlying_spot
    }

    pub fn get_expiries(&self) -> &Array1<Time> {
        &self.expiries
    }

    pub fn spot_moneyness(&self) -> &Array1<Real> {
        &self.spot_moneyness
    }

    pub fn get_volatility_on_spot(&self) -> &Array1<Real> {
        &self.volatility_on_spot
    }

}
