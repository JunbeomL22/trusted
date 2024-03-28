use crate::evaluation_date::EvaluationDate;
use crate::assets::{
    stock::Stock,
    currency::Currency,
};
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use crate::parameters::{
    zero_curve::ZeroCurve,
    volatilities::volatility::Volatility,
};
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

pub struct EquityFuturesPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    stock: Rc<RefCell<Stock>>,
    collateral_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    borrowing_curve: Rc<RefCell<ZeroCurve>>, // or repo
    volatility: Rc<RefCell<Volatility>>,
}

impl EquityFuturesPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        stock: Rc<RefCell<Stock>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
        volatility: Rc<RefCell<Volatility>>,
    ) -> EquityFuturesPricer {
        EquityFuturesPricer {
            evaluation_date,
            stock,
            collateral_curve,
            borrowing_curve,
            volatility,
        }
    }
}

impl PricerTrait for EquityFuturesPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let stock = self.stock.borrow();
        let collateral_curve = self.collateral_curve.borrow();
        let borrowing_curve = self.borrowing_curve.borrow();
        let volatility = self.volatility.borrow();
        let evaluation_date = self.evaluation_date.borrow();
        let maturity = instrument.get_maturity();
        let strike = instrument.get_strike();
        let unit_notional = instrument.get_unit_notional();
        let currency = instrument.get_currency();
        let spot = stock.get_spot();
        let ttm = evaluation_date.get_ttm(maturity);
        let r = collateral_curve.get_rate(ttm)?;
        let q = borrowing_curve.get_rate(ttm)?;
        let sigma = volatility.get_volatility(ttm)?;
        let npv = self.calculate_npv(spot, strike, r, q, sigma, ttm, unit_notional);
        Ok(NpvResult::new(npv, currency.clone()))
    }

    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let stock = self.stock.borrow();
        let collateral_curve = self.collateral_curve.borrow();
        let borrowing_curve = self.borrowing_curve.borrow();
        let volatility = self.volatility.borrow();
        let evaluation_date = self.evaluation_date.borrow();
        let maturity = instrument.get_maturity();
        let strike = instrument.get_strike();
        let unit_notional = instrument.get_unit_notional();
        let spot = stock.get_spot();
        let ttm = evaluation_date.get_ttm(maturity);
        let r = collateral_curve.get_rate(ttm)?;
        let q = borrowing_curve.get_rate(ttm)?;
        let sigma = volatility.get_volatility(ttm)?;
        Ok(self.calculate_npv(spot, strike, r, q, sigma, ttm, unit_notional))
    }
}