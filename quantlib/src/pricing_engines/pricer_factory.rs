use crate::assets::stock::Stock;
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend,
    zero_curve::ZeroCurve,
};
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, InstrumentTriat};
use crate::pricing_engines::{
    match_parameter::MatchParameter,
    pricer::Pricer,
    stock_futures_pricer::StockFuturesPricer,
    bond_pricer::BondPricer,
};
//
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use anyhow::{Result, anyhow};

pub struct PricerFactory {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fxs: HashMap<String, Rc<RefCell<Real>>>,
    stocks: HashMap<String, Rc<RefCell<Stock>>>,
    zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
    dividends: HashMap<String, Rc<RefCell<DiscreteRatioDividend>>>,
    past_close_data: HashMap<String, Rc<CloseData>>,
    match_parameter: Rc<MatchParameter>,
}

impl PricerFactory {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fxs: HashMap<String, Rc<RefCell<Real>>>,
        stocks: HashMap<String, Rc<RefCell<Stock>>>,
        zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
        dividends: HashMap<String, Rc<RefCell<DiscreteRatioDividend>>>,
        past_close_data: HashMap<String, Rc<CloseData>>,
        match_parameter: Rc<MatchParameter>,
    ) -> PricerFactory {
        PricerFactory {
            evaluation_date,
            fxs,
            stocks,
            zero_curves,
            dividends,
            past_close_data,
            match_parameter,
        }
    }

    pub fn create_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let pricer = match Rc::as_ref(instrument) {
            Instrument::StockFutures(_) => self.get_stock_futures_pricer(instrument)?,
            //
            Instrument::FixedCouponBond(_) => self.get_fixed_coupon_bond_pricer(instrument)?,
            //
            Instrument::FloatingRateNote(_) => self.get_frn_pricer(instrument)?,
            //
            _ => {
                return Err(anyhow!(
                    "{}:{}  pricer for {} ({}) is not implemented yet", 
                    file!(), line!(), 
                    instrument.get_code(),
                    instrument.get_type_name(),
                ));
            }
        };
        Ok(pricer)
    }

    fn get_frn_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let discount_curve_name = self.match_parameter.get_discount_curve_name(instrument);
        let forward_curve_name = self.match_parameter.get_rate_index_curve_name(instrument);
        let rate_index_name = instrument.get_rate_index()?.get_name();
        let core = BondPricer::new(
            self.zero_curves.get(discount_curve_name)
            .ok_or_else(
                || anyhow::anyhow!(
                    "failed to get discount curve of {}.\nself.zero_curves does not have {}",
                    instrument.get_code(),
                    discount_curve_name,
                ))?.clone(),
            Some(self.zero_curves.get(forward_curve_name)
            .ok_or_else(
                || anyhow::anyhow!(
                    "failed to get forward curve of {}.\nself.zero_curves does not have {}",
                    instrument.get_code(),
                    forward_curve_name,
                ))?.clone()),
            self.evaluation_date.clone(),
            self.past_close_data.get(rate_index_name).cloned(),
        );

        Ok(Pricer::BondPricer(Box::new(core)))
    }
    fn get_stock_futures_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let underlying_codes = instrument.get_underlying_codes();
        let stock = self.stocks.get(underlying_codes[0]).unwrap().clone();
        let collatral_curve_name = self.match_parameter.get_collateral_curve_names(instrument)[0];
        let borrowing_curve_name = self.match_parameter.get_borrowing_curve_names(instrument)[0];
        let core = StockFuturesPricer::new(
            stock,
            self.zero_curves.get(collatral_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                instrument.get_code(),
                collatral_curve_name,
            ))?.clone(),
            self.zero_curves.get(borrowing_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "failed to get borrowing curve of {}.\nself.zero_curves does not have {}",
                instrument.get_code(),
                borrowing_curve_name,
            ))?.clone(),
            self.evaluation_date.clone(),
        );
        Ok(Pricer::StockFuturesPricer(Box::new(core)))
    }

    fn get_fixed_coupon_bond_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let discount_curve = self.match_parameter.get_discount_curve_name(instrument);
        let core = BondPricer::new(
            self.zero_curves.get(discount_curve)
            .ok_or_else(
                || anyhow::anyhow!(
                    "failed to get discount curve of {}.\nself.zero_curves does not have {}",
                    instrument.get_code(),
                    discount_curve,
                ))?.clone(),
                None,
                self.evaluation_date.clone(),
                None,
            );
        Ok(Pricer::BondPricer(Box::new(core)))
    }
}