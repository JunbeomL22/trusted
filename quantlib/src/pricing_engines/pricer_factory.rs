use crate::assets::{
    equity::Equity,
    fx::{FX, FxCode},
};
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend,
    zero_curve::ZeroCurve,
    rate_index::RateIndex,
};
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, InstrumentTrait};
use crate::pricing_engines::{
    match_parameter::MatchParameter,
    pricer::Pricer,
    equity_futures_pricer::EquityFuturesPricer,
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
    fxs: HashMap<FxCode, Rc<RefCell<FX>>>,
    equities: HashMap<String, Rc<RefCell<Equity>>>,
    zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
    dividends: HashMap<String, Rc<RefCell<DiscreteRatioDividend>>>,
    past_close_data: HashMap<String, Rc<CloseData>>,
    match_parameter: Rc<MatchParameter>,
}

impl PricerFactory {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fxs: HashMap<FxCode, Rc<RefCell<FX>>>,
        equities: HashMap<String, Rc<RefCell<Equity>>>,
        zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
        dividends: HashMap<String, Rc<RefCell<DiscreteRatioDividend>>>,
        past_close_data: HashMap<String, Rc<CloseData>>,
        match_parameter: Rc<MatchParameter>,
    ) -> PricerFactory {
        PricerFactory {
            evaluation_date,
            fxs,
            equities,
            zero_curves,
            dividends,
            past_close_data,
            match_parameter,
        }
    }

    pub fn create_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let pricer = match Rc::as_ref(instrument) {
            Instrument::EquityFutures(_) => self.get_stock_futures_pricer(instrument)?,
            Instrument::Bond(_) => self.get_bond_pricer(instrument)?,
            //
            //
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

    fn get_bond_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let discount_curve_name = self.match_parameter.get_discount_curve_name(instrument)?;
        let discount_curve = self.zero_curves.get(discount_curve_name)
            .ok_or_else(
                || anyhow::anyhow!(
                    "{}:{} (PricerFactory::get_bond_pricer)\n\
                    failed to get discount curve of {}. self.zero_curves does not have {}",
                    file!(), line!(),
                    instrument.get_code(),
                    discount_curve_name,
                ))?.clone();

        let rate_index: Option<&RateIndex> = instrument.get_rate_index()?;
        let forward_curve = match rate_index {
            None => { // the case of fixed coupon bond
                None
            },
            Some(_) => {
                let forward_curve_name = self.match_parameter.get_rate_index_curve_name(instrument)?;
                let res = self.zero_curves.get(forward_curve_name)
                    .ok_or_else(
                        || anyhow::anyhow!(
                            "failed to get forward curve of {}.\nself.zero_curves does not have {}",
                            instrument.get_code(),
                            forward_curve_name,
                        ))?.clone();
                Some(res)
            },
        }; // the end of the forward curve construction which is optional

        let past_fixing_data = match rate_index {
            None => {
                None
            },
            Some(rate_index) => {
                let past_fixing_data = self.past_close_data.get(rate_index.get_name())
                    .ok_or_else(
                        || anyhow::anyhow!(
                            "failed to get past fixing data of {}.\nself.past_close_data does not have {}",
                            instrument.get_code(),
                            rate_index.get_code(),
                        ))?.clone();
                Some(past_fixing_data)
            },
        }; // the end of the past fixing data construction which is optional
        
        let core = BondPricer::new(
            self.evaluation_date.clone(),
            discount_curve,
            forward_curve,
            past_fixing_data,    
        );
        Ok(Pricer::BondPricer(core))

    }
    fn get_stock_futures_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let underlying_codes = instrument.get_underlying_codes();
        let stock = self.equities.get(underlying_codes[0]).unwrap().clone();
        let collatral_curve_name = self.match_parameter.get_collateral_curve_names(instrument)?[0];
        let borrowing_curve_name = self.match_parameter.get_borrowing_curve_names(instrument)?[0];
        let core = EquityFuturesPricer::new(
            self.evaluation_date.clone(),
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
        );
        Ok(Pricer::EquityFuturesPricer(core))
    }
}