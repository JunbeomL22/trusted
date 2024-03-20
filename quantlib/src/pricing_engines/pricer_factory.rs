use crate::assets::stock::Stock;
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend,
    zero_curve::ZeroCurve,
};
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
    match_parameter: Rc<MatchParameter>,
}

impl PricerFactory {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fxs: HashMap<String, Rc<RefCell<Real>>>,
        stocks: HashMap<String, Rc<RefCell<Stock>>>,
        zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
        dividends: HashMap<String, Rc<RefCell<DiscreteRatioDividend>>>,
        match_parameter: Rc<MatchParameter>,
    ) -> PricerFactory {
        PricerFactory {
            evaluation_date,
            fxs,
            stocks,
            zero_curves,
            dividends,
            match_parameter,
        }
    }

    pub fn create_pricer(&self, inst: &Rc<Instrument>) -> Result<Pricer> {
        let inst_type = inst.get_type_name();
        let inst_code = inst.get_code();
        let undertlying_codes = inst.get_underlying_codes(); 

        let pricer = match Rc::as_ref(inst) {
            Instrument::StockFutures(_) => {
                let stock = self.stocks.get(undertlying_codes[0]).unwrap().clone();
                let collatral_curve_name = self.match_parameter.get_collateral_curve_names(inst)[0];
                let borrowing_curve_name = self.match_parameter.get_borrowing_curve_names(inst)[0];
                let core = StockFuturesPricer::new(
                    stock,
                    self.zero_curves.get(collatral_curve_name)
                    .ok_or_else(|| anyhow::anyhow!(
                        "failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                        inst_code,
                        collatral_curve_name,
                    ))?.clone(),
                    self.zero_curves.get(borrowing_curve_name)
                    .ok_or_else(|| anyhow::anyhow!(
                        "failed to get borrowing curve of {}.\nself.zero_curves does not have {}",
                        inst_code,
                        borrowing_curve_name,
                    ))?.clone(),
                    self.evaluation_date.clone(),
                );
                Pricer::StockFuturesPricer(Box::new(core))
            },

            Instrument::FixedCouponBond(_) => {
                let discount_curve = self.match_parameter.get_discount_curve_name(inst);
                let core = BondPricer::new(
                    self.zero_curves.get(discount_curve)
                    .ok_or_else(|| anyhow::anyhow!(
                        "failed to get discount curve of {}.\nself.zero_curves does not have {}",
                        inst_code,
                        discount_curve,
                    ))?.clone(),
                    None,
                    self.evaluation_date.clone(),
                );
                Pricer::FixedCouponBondPricer(Box::new(core))
            },

            _ => {
                return Err(anyhow!(
                    "{}:{}  pricer for {} ({}) is not implemented yet", 
                    file!(), line!(), inst_code, inst_type
                ));
            }
        };
        Ok(pricer)
    }
}