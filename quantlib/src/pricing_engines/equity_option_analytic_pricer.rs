use crate::assets::currency;
use crate::time::{
    calendars::nullcalendar::NullCalendar,
    calendar_trait::CalendarTrait,
};
use crate::evaluation_date::EvaluationDate;
use crate::assets::{
    equity::Equity,
    currency::Currency,
};
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::parameters::quanto;
use crate::pricing_engines::{
    pricer::PricerTrait,
    equity_futures_pricer::EquityFuturesPricer,
};
use crate::parameters::{
    zero_curve::ZeroCurve,
    volatility::Volatility,
    quanto::Quanto,
};
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::InstrumentTrait;
use crate::enums::OptionType;
//
use time::OffsetDateTime;
use anyhow::{anyhow, Context, Result};
use core::borrow;
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};
use statrs::distribution::{Normal, ContinuousCDF};

pub struct EquityOptionAnalyticPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    equity: Rc<RefCell<Equity>>,   
    futures_helper: EquityFuturesPricer,
    discount_curve: Rc<RefCell<ZeroCurve>>,
    volatility: Rc<RefCell<Volatility>>,
    quanto: Option<Rc<RefCell<Quanto>>>,
    time_calculator: NullCalendar,
}

impl EquityOptionAnalyticPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        equity: Rc<RefCell<Equity>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>, 
        discount_curve: Rc<RefCell<ZeroCurve>>,
        volatility: Rc<RefCell<Volatility>>,
        quanto: Option<Rc<RefCell<Quanto>>>,
    ) -> EquityOptionAnalyticPricer {
        let futures_helper = EquityFuturesPricer::new(
            evaluation_date.clone(),
            equity.clone(),
            collateral_curve.clone(),
            borrowing_curve.clone(),
        );

        EquityOptionAnalyticPricer {
            evaluation_date,
            equity,
            futures_helper,
            discount_curve,
            volatility,
            quanto,
            time_calculator: NullCalendar::new(),
        }
    }
}

impl PricerTrait for EquityOptionAnalyticPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let maturity = instrument.get_maturity()
            .context("(OptionAnalyticPricer:npv) Failed to get maturity")?;
        let fwd = self.futures_helper.fair_forward(&maturity)?;
        let strike = instrument.get_strike()?;
        let forward_moneyness = strike / fwd;
        let t = self.time_calculator.get_time_difference(
            self.evaluation_date.borrow().get_date(),
            &maturity,
        );
        
        let total_variance = self.volatility
            .borrow()
            .total_variance(t, forward_moneyness)?;
        let total_deviation = self.volatility
            .borrow()
            .total_deviation(t, forward_moneyness)?;

        if instrument.get_currency() != instrument.get_underlying_currency()? &&
        self.quanto.is_none() 
        {
            return Err(anyhow!(
                "({}:{}) {} ({}) has different currency from underlying equity ({}) but no quanto is provided",
                file!(), line!(),
                instrument.get_name(), instrument.get_code(), self.equity.borrow().get_name(),
            ));
        }

        let vol = self.volatility.borrow()
            .get_value(t, forward_moneyness);
        let quanto_adjustment = match &self.quanto {
            Some(quanto) => {
                vol * t * quanto.borrow().quanto_adjust(t, forward_moneyness)
            }
            None => 0.0,
        };

        let y = forward_moneyness.ln();
        let option_type = instrument.get_option_type()?;

        let dsc = self.discount_curve.borrow().get_discount_factor(t)?;

        let d1 = (-y + total_variance / 2.0 - quanto_adjustment) / total_deviation;
        let d2 = d1 - total_deviation;

        let normal = Normal::new(0.0, 1.0).unwrap(); 
        let nd1 = normal.cdf(d1 as f64) as Real;
        let nd2 = normal.cdf(d2 as f64) as Real;

        match option_type {
            OptionType::Call => {
                Ok(dsc * (fwd * nd1 - strike * nd2))
            }
            OptionType::Put => {
                Ok(dsc * (strike * (1.0 - nd2) - fwd * (1.0 - nd1)))
            }
        }
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }
}