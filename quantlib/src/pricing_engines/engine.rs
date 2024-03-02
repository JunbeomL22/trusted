use crate::data::observable::Observable;
use crate::instruments::instrument_info::InstrumentInfo;
use crate::instruments::stock_futures::StockFutures;
use crate::parameters::zero_curve_code::ZeroCurveCode;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::parameters::zero_curve::{self, ZeroCurve};
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, Instruments};
use crate::pricing_engines::calculation_result::CalculationResult;
use crate::definitions::{Real, FX};
use crate::assets::stock::Stock;
use crate::data::vector_data::VectorData;
use crate::pricing_engines::stock_futures_pricer::StockFuturesPricer;
use crate::pricing_engines::match_parameter::MatchPrameter;
use core::borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::pricing_engines::pricer::Pricer;
use time::OffsetDateTime;

/// Engine typically handles a bunch of instruments and calculate the pricing of the instruments.
/// Therefore, the result of calculations is a hashmap with the key being the code of the instrument
/// Engine is a struct that holds the calculation results of the instruments
pub struct Engine<'a> {
    calculation_result: HashMap<&'a str, CalculationResult<'a>>,
    calculation_configuration: CalculationConfiguration,
    curve_data: HashMap<&'a str, VectorData>,
    dividend_data: HashMap<&'a str, VectorData>,
    //
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fxs: HashMap<FX, Rc<RefCell<Real>>>,
    stocks: HashMap<&'a str, Rc<RefCell<Stock>>>,
    zero_curves: HashMap<&'a str, Rc<RefCell<ZeroCurve>>>,
    dividends: HashMap<&'a str, Rc<RefCell<VectorData>>>,
    // instruments
    instruments: Option<Instruments<'a>>, // all instruments
    pricers: HashMap<&'a str, Pricer>, // pricers for each instrument
    // selected instuments for calculation,
    // e.g., if we calcualte a delta of a single stock, we do not need calculate all instruments
    instruments_in_action: Vec<&'a Instrument<'a>>, 
    //
    match_parameter: MatchPrameter<'a>,
}

impl<'a> Engine<'a> {
    pub fn initialize(
        calculation_configuration: CalculationConfiguration,
        evaluation_date: EvaluationDate,
        //
        fx_input: HashMap<FX, (OffsetDateTime, Real)>,
        stock_input: HashMap<&'a str, (OffsetDateTime, Real)>,
        curve_data: HashMap<&'a str, VectorData>,
        dividend_data: HashMap<&'a str, VectorData>,
        //
        instruments: Instruments<'a>,
        match_parameter: MatchPrameter<'a>,
    ) -> Engine<'a> {
        let evaluation_date = Rc::new(RefCell::new(
            evaluation_date
        ));

        let zero_curves = HashMap::new();
        for (key, data) in curve_data.iter() {
            let zero_curve = Rc::new(RefCell::new(
                ZeroCurve::new(
                    evaluation_date.clone(),
                    data,
                    ZeroCurveCode::from_str(key).unwrap(),
                    key.to_string(),
                )));
            data.borrow_mut().add_observer(zero_curve.clone());
        }

        let dividends = HashMap::new();
        for (key, data) in dividend_data.iter() {
            let spot = stock_input.get(key).unwrap();
            let dividend = Rc::new(RefCell::new(
                DiscreteRatioDividend::new(
                    evaluation_date.clone(),
                    data,
                    spot,
                    key.to_string(),
                )));
            data.borrow_mut().add_observer(dividend.clone());
        }
        // making fx Rc -> RefCell for pricing
        let fxs: HashMap<FX, Rc<RefCell<Real>>> = fx_input
            .iter()
            .map(|(key, value)| {
                let rc = Rc::new(RefCell::new(*value));
                (key.clone(), rc)
            })
            .collect();
        // making stock Rc -> RefCell for pricing
        let stocks = HashMap::new();
        for (key, value) in stock_input.iter() {
            let rc = Rc::new(RefCell::new(
                Stock::new(
                    *value,
                    evaluation_date.clone(),
                    dividends.get(key).unwrap().clone(),
                    key.to_string(),

                )));
            stocks.insert(key, rc);
        }
        
        Engine {
            calculation_result: HashMap::new(),
            calculation_configuration: calculation_configuration.clone(),
            evaluation_date: evaluation_date.clone(),
            fxs: fxs.clone(),
            stocks: stocks.clone(),
            curve_data: curve_data.clone(),
            zero_curves: zero_curves,
            dividend_data: dividend_data.clone(),
            dividends: dividends,
            instruments: None,
            instruments_in_action: vec![],
            pricers: HashMap::new(),
            match_parameter: match_parameter,
        }
    }
        
    pub fn set_instuments(&mut self, instruments: Instruments<'a>) {
        self.instruments = Some(instruments);
    
        for instrument in self.instruments.iter() {
            let inst = instrument.as_trait();
            let code = inst.get_code();
            let inst_type = inst.get_type_name();
            let instrument_information = InstrumentInfo::new(
                inst.get_name(),
                code,
                inst_type,
                inst.get_currency().clone(),
                inst.get_unit_notional(),
                inst.get_maturity().clone(),
            );

            let init_res = CalculationResult::new(
                instrument_information,
                self.evaluation_date.borrow().get_date_clone(),
            );

            self.calculation_result.insert(
                inst.get_code(),
                init_res
            );
        }
    }

    pub fn with_instruments(mut self, instruments: Vec<&'a Instrument<'a>>) -> Engine<'a> {
        self.set_instuments(instruments);
        self.instruments_in_action = self.instruments.unwrap().get_instruments();
        self
    }

    pub fn initialize_pricers(mut self) {
        let inst_vec = self.instruments.as_ref().unwrap().get_instruments();
        for inst in inst_vec.iter() {
            let inst_type = inst.as_trait().get_type_name();
            let inst_name = inst.as_trait().get_name();
            let inst_code = inst.as_trait().get_code();
            let inst_curr = inst.as_trait().get_currency();
            let undertlying_names = inst.as_trait().get_underlying_names();

            let pricer = match inst {
                Instrument::StockFutures(instrument) => {
                    let stock = self.stocks.get(undertlying_names[0]).unwrap().clone();
                    let collatral_curve_name = self.match_parameter.get_collateral_curve_name(inst);
                    let borrowing_curve_name = self.match_parameter.get_borrowing_curve_name(inst);
                    StockFuturesPricer::new(
                        stock,
                        self.zero_curves.get(collatral_curve_name).unwrap().clone(),
                        self.zero_curves.get(borrowing_curve_name).unwrap().clone(),
                        self.evaluation_date.clone(),
                    )
                },
                _ => {
                    panic!(
                        "Not implemented yet (type = {}, name =  {})", 
                        inst_type,
                        inst_name
                    );
                }
            };
            self.pricers.insert(inst_code, pricer);
        }
    }

    pub fn get_npv(&self) -> HashMap<&str, Real> {
        let mut npv = HashMap::new();
        for inst in self.instruments_in_action {
            let pricer = self.pricers.get(inst.get_code()).unwrap();
            let npv = pricer.npv(inst);
            npv.insert(inst.get_code(), npv);
        }
        npv
    }
    pub fn set_npv(&mut self) {
        let npv = self.get_npv();
        for (code, result) in self.calculation_result.iter_mut() {
            result.set_npv(npv.get(code).unwrap());
        }
    }

    /// Set the value of the instruments which means npv * unit_notional
    pub fn set_value(&mut self) {
        for (_code, result) in self.calculation_result.iter_mut() {
            result.set_value();
        }
    }

    pub fn set_delta(&mut self) {
        let all_underlying_codes = self.instruments.get_underlying_codes();
        let delta_bump_ratio = self.calculation_configuration.get_delta_bump();
        for und_code in all_underlying_codes.iter() {
            self.instruments_in_action = self.instruments.instruments_underlying_includes(und_code);
            let npv = self.get_npv();
    }

    pub fn calculate(&mut self) {
        self.set_npv();
        self.set_value();
        // if delta is true, calculate delta
        if self.calculation_configuration.get_delta_calculation() {
            self.set_delta();
        }

    }
    pub fn get_calculation_result(&self) -> &HashMap<&str, CalculationResult> {
        &self.calculation_result
    }

    pub fn get_calculation_result_clone(&self) -> HashMap<&str, CalculationResult> {
        self.calculation_result.clone()
    }
}