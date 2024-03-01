use crate::data::observable::Observable;
use crate::instruments::instrument_info::InstrumentInfo;
use crate::parameters::enums::ZeroCurveCode;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::parameters::zero_curve::{self, ZeroCurve};
use crate::parameters::zero_curve_code::ZeroCurveCode;
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, Instruments};
use crate::pricing_engines::calculation_result::CalculationResult;
use crate::definitions::{Real, FX};
use crate::assets::stock::Stock;
use crate::data::vector_data::VectorData;
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
    evaluation_date: EvaluationDate,
    curve_data: HashMap<&'a str, VectorData>,
    dividend_data: HashMap<&'a str, VectorData>,
    //
    fxs: HashMap<FX, Rc<RefCell<Real>>>,
    stocks: HashMap<&'a str, Rc<RefCell<Stock>>>,
    zero_curves: HashMap<&'a str, Rc<RefCell<ZeroCurve>>>,
    dividends: HashMap<&'a str, Rc<RefCell<VectorData>>>,
    // instruments
    instruments: Instruments<'a>, // all instruments
    pricers: HashMap<&'a str, Pricer>, // pricers for each instrument
    // selected instuments for calculation,
    // e.g., if we calcualte a delta of a single stock, we do not need calculate all instruments
    instruments_in_action: Vec<&'a Instrument<'a>>, 
}

impl<'a> Engine<'a> {
    pub fn initialize(
        calculation_configuration: CalculationConfiguration,
        evaluation_date: EvaluationDate,
        fx_value: HashMap<FX, (OffsetDateTime, Real)>,
        stock_value: HashMap<&'a str, (OffsetDateTime, Real)>,
        curve_data: HashMap<&'a str, VectorData>,
        dividend_data: HashMap<&'a str, VectorData>,
    ) -> Engine<'a> {
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
            instruments: vec![],
            instruments_in_action: vec![],
            pricer: None,
        }
    }
        
    pub fn set_instuments(&mut self, instruments: Vec<&'a Instrument<'a>>) {
        self.instruments = instruments;
        // check if all instruments are of the same type
        let mut instrument_type = String::new();
        for instrument in self.instruments.iter() {
            if instrument_type.is_empty() {
                instrument_type = instrument.as_trait().get_type_name().to_string();
            } else {
                if instrument_type != instrument.as_trait().get_type_name() {
                    assert_eq!(
                        instrument_type,
                        instrument.as_trait().get_type_name(),
                        "All instruments must be of the same type: {} and {} are different types",
                        instrument_type,
                        instrument.as_trait().get_type_name()
                    );
                }
            }
        }

        for instrument in self.instruments.iter() {
            let inst = instrument.as_trait();
            let instrument_information = InstrumentInfo::new(
                inst.get_name(),
                inst.get_code(),
                inst.get_type_name(),
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
        self
    }

    pub fn with_pricer(mut self, pricer: Box<dyn Pricer>) -> Engine<'a> {
        self.pricer = Some(pricer);
        self
    }

    pub fn calculate_npv(&mut self) {
        self.set_npv();
        self.set_value();
    }
    pub fn set_npv(&mut self) {
        let npv: HashMap<&str, Real> = self.pricer.as_ref().unwrap().npv(&self.instruments);
        for (code, value) in npv.iter() {
            self.calculation_result.get_mut(code).unwrap().set_npv(*value);
        }
    }

    /// Set the value of the instruments which means npv * unit_notional
    pub fn set_value(&mut self) {
        for (_code, result) in self.calculation_result.iter_mut() {
            result.set_value();
        }
    }

    pub fn set_delta(&mut self) {
        let delta: HashMap<&str, HashMap<&str, Real>> = self.pricer.as_ref().unwrap().delta(&self.instruments);
        for (code, value) in delta.iter() {
            self.calculation_result.get_mut(code).unwrap().set_delta(value.clone());
        }
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