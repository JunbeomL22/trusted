use crate::assets::{
    equity::Equity,
    fx::{FX, FxCode},
};
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend,
    zero_curve::ZeroCurve,
    rate_index::RateIndex,
    quanto::Quanto,
    volatility::Volatility,
};
use crate::data::history_data::CloseData;
use crate::definitions::Real;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, InstrumentTrait};
use crate::pricing_engines::{
    match_parameter::MatchParameter,
    pricer::Pricer,
    equity_futures_pricer::EquityFuturesPricer,
    equity_option_analytic_pricer::EquityOptionAnalyticPricer,
    bond_pricer::BondPricer,
    ktbf_pricer::KtbfPricer,
    fx_futures_pricer::FxFuturesPricer,
    plain_swap_pricer::PlainSwapPricer,
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
    underlying_volatilities: HashMap<String, Rc<RefCell<Volatility>>>,
    quantos: HashMap<(String, FxCode), Rc<RefCell<Quanto>>>, // (underlying_code, fx_code) -> Quanto
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
        underlying_volatilities: HashMap<String, Rc<RefCell<Volatility>>>,
        quantos: HashMap<(String, FxCode), Rc<RefCell<Quanto>>>,
        past_close_data: HashMap<String, Rc<CloseData>>,
        match_parameter: Rc<MatchParameter>,
    ) -> PricerFactory {
        PricerFactory {
            evaluation_date,
            fxs,
            equities,
            zero_curves,
            dividends,
            underlying_volatilities,
            quantos,
            past_close_data,
            match_parameter,
        }
    }

    pub fn create_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let pricer = match Rc::as_ref(instrument) {
            Instrument::EquityFutures(_) => self.get_equity_futures_pricer(instrument)?,
            Instrument::EquityVanillaOption(_) => self.get_equity_vanilla_option_pricer(instrument)?,
            Instrument::Bond(_) => self.get_bond_pricer(instrument)?,
            Instrument::KTBF(_) => self.get_ktbf_pricer(instrument)?,
            Instrument::FxFutures(_) => self.get_fx_futures_pricer(instrument)?,
            Instrument::PlainSwap(_) => self.get_plain_swap_pricer(instrument)?,
            //
            //
            _ => {
                return Err(anyhow!(
                    "({}:{})   pricer for {} ({}) is not implemented yet", 
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
    fn get_equity_futures_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let underlying_codes = instrument.get_underlying_codes();
        let equity = self.equities.get(underlying_codes[0]).unwrap().clone();
        let collatral_curve_name = self.match_parameter.get_collateral_curve_names(instrument)?[0];
        let borrowing_curve_name = self.match_parameter.get_borrowing_curve_names(instrument)?[0];
        let core = EquityFuturesPricer::new(
            self.evaluation_date.clone(),
            equity,
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

    fn get_equity_vanilla_option_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let equity = self.equities.get(instrument.get_underlying_codes()[0])
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get equity of {}.\nself.equities does not have {}",
                file!(), line!(), instrument.get_code(), instrument.get_underlying_codes()[0],
            ))?.clone();
        let volatility = self.underlying_volatilities.get(instrument.get_underlying_codes()[0])
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get volatility of {}.\nself.equity_volatilities does not have {}",
                file!(), line!(), instrument.get_code(), instrument.get_underlying_codes()[0],
            ))?.clone();
        let discount_curve_name = self.match_parameter.get_discount_curve_name(instrument)?;
        let discount_curve = self.zero_curves.get(discount_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), discount_curve_name,
            ))?.clone();

        let collateral_curve_name = self.match_parameter.get_collateral_curve_names(instrument)?[0];
        let collatral_curve = self.zero_curves.get(collateral_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), collateral_curve_name,
            ))?.clone();
        let borrowing_curve_name = self.match_parameter.get_borrowing_curve_names(instrument)?[0];
        let borrowing_curve = self.zero_curves.get(borrowing_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get borrowing curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), borrowing_curve_name,
            ))?.clone();

        let curr = instrument.get_currency();
        let und_curr = instrument.get_underlying_currency()?;
        let quanto = match und_curr == curr {
            false => {
                let fx_code = FxCode::new(und_curr.clone(), curr.clone());
                let underlying_code = instrument.get_underlying_codes()[0].clone();
                let key = (underlying_code, fx_code);
                let quanto = self.quantos.get(&key)
                    .ok_or_else(|| anyhow::anyhow!(
                        "({}:{}) failed to get quanto of {}.\nself.quantos does not have {:?}",
                        file!(), line!(), instrument.get_code(), key,
                    ))?.clone();
                Some(quanto)
            },
            true => None,
        };
        let core = EquityOptionAnalyticPricer::new(
            self.evaluation_date.clone(),
            equity,
            collatral_curve,
            borrowing_curve,
            discount_curve,
            volatility,
            quanto,
        );
        Ok(Pricer::EquityOptionAnalyticPricer(core))
    }

    fn get_ktbf_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let discount_curve_name = String::from("KRWGOV");
        let discount_curve = self.zero_curves.get(&discount_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), discount_curve_name,
            ))?.clone();
        
        let core = KtbfPricer::new(
            self.evaluation_date.clone(),
            discount_curve,
        );

        Ok(Pricer::KtbfPricer(core))
    }
    
    fn get_fx_futures_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let fx_code = instrument.get_fx_code()?;

        let fx = self.fxs.get(fx_code)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get FX of {}.\nself.fxs does not have {:?}",
                file!(), line!(), instrument.get_code(), fx_code,
            ))?.clone();
        let underlying_currency_curve_name = self.match_parameter.get_floating_crs_curve_name(instrument)?;
        let underlying_currency_curve = self.zero_curves.get(underlying_currency_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get underlying currency curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), underlying_currency_curve_name,
            ))?.clone();
        let futures_currency_curve_name = self.match_parameter.get_crs_curve_name(instrument)?;
        let futures_currency_curve = self.zero_curves.get(futures_currency_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get futures currency curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), futures_currency_curve_name,
            ))?.clone();
        
        let core = FxFuturesPricer::new(
            self.evaluation_date.clone(),
            fx,
            underlying_currency_curve,
            futures_currency_curve,
        );
        Ok(Pricer::FxFuturesPricer(core))
    }

    fn get_plain_swap_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let fixed_leg_discount_curve_name = self.match_parameter.get_crs_curve_name(instrument)?;
        let fixed_leg_discount_curve = self.zero_curves.get(fixed_leg_discount_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get fixed leg discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), fixed_leg_discount_curve_name,
            ))?.clone();

        let floating_leg_discount_curve_name = self.match_parameter.get_floating_crs_curve_name(instrument)?;
        let floating_leg_discount_curve = self.zero_curves.get(floating_leg_discount_curve_name)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get floating leg discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_code(), floating_leg_discount_curve_name,
            ))?.clone();
        
        let rate_index = instrument.get_rate_index()?;
        let forward_curve = match rate_index {
            Some(rate_index) => {
                let forward_curve_name = self.match_parameter.get_rate_index_curve_name(instrument)?;
                let res = self.zero_curves.get(forward_curve_name)
                    .ok_or_else(|| anyhow::anyhow!(
                        "({}:{}) failed to get forward curve of {}.\nself.zero_curves does not have {}",
                        file!(), line!(), instrument.get_code(), forward_curve_name,
                    ))?.clone();
                Some(res)
            },
            None => None,
        };

        let past_fixig_data = match rate_index {
            Some(rate_index) => {
                let past_fixing_data = self.past_close_data.get(rate_index.get_name())
                    .ok_or_else(|| anyhow::anyhow!(
                        "({}:{}) failed to get past fixing data of {}.\nself.past_close_data does not have {}",
                        file!(), line!(), instrument.get_code(), rate_index.get_code(),
                    ))?.clone();
                Some(past_fixing_data)
            },
            None => None,
        };
        
        let fixed_currency = instrument.get_currency();
        let floating_currency = instrument.get_underlying_currency()?;
        let floating_to_fixed_fx = match fixed_currency == floating_currency {
            true => None,
            false => {
                let fx_code = FxCode::new(floating_currency.clone(), fixed_currency.clone());
                let key = (floating_currency.clone(), fixed_currency.clone());
                let fx = self.fxs.get(&fx_code)
                    .ok_or_else(|| anyhow::anyhow!(
                        "({}:{}) failed to get FX of {}.\nself.fxs does not have {:?}",
                        file!(), line!(), instrument.get_code(), fx_code,
                    ))?.clone();
                Some(fx)
            },
        };

        let core = PlainSwapPricer::new(
            self.evaluation_date.clone(),
            fixed_leg_discount_curve,
            floating_leg_discount_curve,
            forward_curve,
            past_fixig_data,
            floating_to_fixed_fx,
        )?;

        Ok(Pricer::PlainSwapPricer(core))

    }
}