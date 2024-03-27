use crate::evaluation_date::EvaluationDate;
use crate::assets::{
    fx::FX,
    currency::Currency,
};
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::InstrumentTrait;
//
use time::OffsetDateTime;
use anyhow::{anyhow, Context, Result};
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

pub struct FxFuturesPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fx: Rc<RefCell<FX>>, // floationg to fixed fx as in PlainSwapPricer. 
    underlying_currency_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    futures_currency_curve: Rc<RefCell<ZeroCurve>>, // or repo
}

impl FxFuturesPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fx: Rc<RefCell<FX>>,
        underlying_currency_curve: Rc<RefCell<ZeroCurve>>,
        futures_currency_curve: Rc<RefCell<ZeroCurve>>,
    ) -> Self {
        FxFuturesPricer {
            evaluation_date,
            fx,
            underlying_currency_curve,
            futures_currency_curve,
        }
    }
}

impl PricerTrait for FxFuturesPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let fx_rate = self.fx.borrow().get_rate();
        let maturity = match instrument.get_maturity() {
            Some(maturity) => maturity,
            None => return Err(anyhow!(
                "({}:{}) Maturity of {} ({}) is not set",
                file!(),
                line!(),
                instrument.get_name(),
                instrument.get_code(),
            )),
        };
        
        let underlying_discount = self.underlying_currency_curve.borrow().get_discount_factor_at_date(&maturity)?;
        let futures_discount = self.futures_currency_curve.borrow().get_discount_factor_at_date(&maturity)?;

        let npv = fx_rate * underlying_discount / futures_discount;
        Ok(npv)
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }

    fn fx_exposure(&self, instrument: &Instrument, npv: Real) -> Result<HashMap<Currency, Real>> {
        let average_trade_price = instrument.get_average_trade_price();
        let futures_currency = instrument.get_currency().clone();
        let underlying_currency = instrument.get_underlying_currency()?.clone();
        let maturity = match instrument.get_maturity() {
            Some(maturity) => maturity,
            None => return Err(anyhow!(
                "({}:{}) Maturity of {} ({}) is not set",
                file!(),
                line!(),
                instrument.get_name(),
                instrument.get_code(),
            )),
        };

        let underlying_discount = self.underlying_currency_curve.borrow().get_discount_factor_at_date(&maturity)?;
        let futures_discount = self.futures_currency_curve.borrow().get_discount_factor_at_date(&maturity)?;

        let mut res: HashMap<Currency, Real> = HashMap::new();
        res.insert(futures_currency, - futures_discount * average_trade_price);
        res.insert(underlying_currency, - underlying_discount);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::{
        fx::FX,
        currency::Currency,
    };
    use crate::definitions::Real;
    use crate::instruments::fx_futures::FxFutures;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::pricing_engines::fx_futures_pricer::FxFuturesPricer;
    use crate::evaluation_date::EvaluationDate;
    use crate::instrument::InstrumentTrait;
    use crate::instrument::Instrument;
    use crate::pricing_engines::pricer::PricerTrait;
    use crate::pricing_engines::npv_result::NpvResult;
    use std::rc::Rc;
    use std::cell::RefCell;
    use time::OffsetDateTime;
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn test_fx_futures_pricer() -> Result<()> {
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(OffsetDateTime::unix_epoch())));
        let fx = Rc::new(RefCell::new(FX::new("USD/KRW".to_string(), 1100.0)));
        let underlying_currency_curve = Rc::new(RefCell::new(ZeroCurve::new("USD".to_string(), 0.02)));
        let futures_currency_curve = Rc::new(RefCell::new(ZeroCurve::new("KRW".to_string(), 0.03)));

        let pricer = FxFuturesPricer::new(
            evaluation_date.clone(),
            fx.clone(),
            underlying_currency_curve.clone(),
            futures_currency_curve.clone(),
        );

        let fxfutures = FxFutures::new(
            1100.0,
            OffsetDateTime::unix_epoch(),
            OffsetDateTime::unix_epoch(),
            OffsetDateTime::unix_epoch(),
            OffsetDateTime::unix_epoch(),
            1000000.0,
            Currency::USD,
            Currency::KRW,
            "USD/KRW".to_string(),
            "USD/KRW".to_string(),
        );

        let inst = Instrument::FxFutures(fxfutures);
        let npv = pricer.npv(&inst)?;
        println!("npv: {}", npv);
    }
}

