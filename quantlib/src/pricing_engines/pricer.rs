use crate::assets::currency::Currency;
use crate::instrument::{
    Instrument,
    InstrumentTrait,
};
use crate::definitions::Real;
use crate::pricing_engines::npv_result::NpvResult;
use crate::pricing_engines::{
    bond_pricer::BondPricer,
    equity_futures_pricer::EquityFuturesPricer,
    ktbf_pricer::KtbfPricer,
    krx_yield_pricer::KrxYieldPricer,
    plain_swap_pricer::PlainSwapPricer,
    fx_futures_pricer::FxFuturesPricer,
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
        map.insert(instrument.get_currency().clone(), npv * instrument.get_unit_notional());
        Ok(map)
    
    }
}

#[enum_dispatch(PricerTrait)]
pub enum Pricer {
    EquityFuturesPricer(EquityFuturesPricer),
    BondPricer(BondPricer),
    KtbfPricer(KtbfPricer),
    KrxYieldPricer(KrxYieldPricer),
    PlainSwapPricer(PlainSwapPricer),
    FxFuturesPricer(FxFuturesPricer),
}
