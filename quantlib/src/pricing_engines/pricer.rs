use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::{Instrument, InstrumentTrait};
use crate::pricing_engines::npv_result::NpvResult;
use crate::pricing_engines::{
    bond_pricer::BondPricer, futures_pricer::FuturesPricer, fx_futures_pricer::FxFuturesPricer,
    identity_pricer::IdentityPricer, krx_yield_pricer::KrxYieldPricer, ktbf_pricer::KtbfPricer,
    option_analytic_pricer::OptionAnalyticPricer, plain_swap_pricer::PlainSwapPricer,
    unit_pricer::UnitPricer,
};
//
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use std::collections::HashMap;

#[enum_dispatch]
pub trait PricerTrait {
    // Code -> NPV
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult>;

    fn npv(&self, instrument: &Instrument) -> Result<Real>;
    /// unit_notional is considered
    fn fx_exposure(&self, instrument: &Instrument, npv: Real) -> Result<HashMap<Currency, Real>> {
        let mut map = HashMap::new();
        map.insert(
            *instrument.get_currency(),
            npv * instrument.get_unit_notional(),
        );
        Ok(map)
    }
}

#[enum_dispatch(PricerTrait)]
pub enum Pricer {
    FuturesPricer(FuturesPricer),
    OptionAnalyticPricer(OptionAnalyticPricer),
    BondPricer(BondPricer),
    KtbfPricer(KtbfPricer),
    KrxYieldPricer(KrxYieldPricer),
    PlainSwapPricer(PlainSwapPricer),
    FxFuturesPricer(FxFuturesPricer),
    IdentityPricer(IdentityPricer),
    UnitPricer(UnitPricer),
}
