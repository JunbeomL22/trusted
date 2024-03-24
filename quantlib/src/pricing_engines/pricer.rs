use crate::instrument::Instrument;
use crate::definitions::Real;
use crate::pricing_engines::npv_result::NpvResult;
use crate::pricing_engines::{
    bond_pricer::BondPricer,
    stock_futures_pricer::StockFuturesPricer,
    ktbf_pricer::KtbfPricer,
    krx_yield_pricer::KrxYieldPricer,
    irs_pricer::IrsPricer,
};
//
use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait PricerTrait {
    // Code -> NPV
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult>;
    fn npv(&self, instrument: &Instrument) -> Result<Real>;
    fn fx_exposure(&self, _instrument: &Instrument, npv: Real) -> Result<Real> { Ok(npv) }
}

#[enum_dispatch(PricerTrait)]
pub enum Pricer {
    StockFuturesPricer(StockFuturesPricer),
    BondPricer(BondPricer),
    KtbfPricer(KtbfPricer),
    KrxYieldPricer(KrxYieldPricer),
    IrsPricer(IrsPricer),
}
