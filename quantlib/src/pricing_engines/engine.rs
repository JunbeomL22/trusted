use crate::instruments::instrument_info::InstrumentInfo;
use crate::instruments::stock_futures::StockFutures;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use crate::parameters::zero_curve::{self, ZeroCurve};
use crate::util::type_name;
use crate::instruments::fixed_coupon_bond::FixedCouponBond;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::Instrument;
use crate::pricing_engines::calculation_result::CalculationResult;
use crate::definitions::{Real, FX};
use crate::assets::stock::Stock;
use crate::data::vector_data::VectorData;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::pricing_engines::pricer::Pricer;

/// Engine typically handles a bunch of instruments and calculate the pricing of the instruments.
/// Therefore, the result of calculations is a hashmap with the key being the code of the instrument
/// Engine is a struct that holds the calculation results of the instruments
pub struct Engine {
    calculation_result: HashMap<String, CalculationResult>,
    calculation_configuration: CalculationConfiguration,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fxs: HashMap<FX, Rc<RefCell<Real>>>,
    stocks: HashMap<String, Rc<RefCell<Stock>>>,
    curve_data: HashMap<String, Rc<RefCell<VectorData>>>,
    dividend_data: HashMap<String, Rc<RefCell<VectorData>>>,
    instruments: Vec<Instrument>,
    instruments_in_action: Vec<Instrument>,
    pricer: Option<Box<dyn Pricer>>,
}

impl Engine {
    pub fn initialize(
        calculation_configuration: CalculationConfiguration,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fxs: HashMap<FX, Rc<RefCell<Real>>>,
        stocks: HashMap<String, Rc<RefCell<Stock>>>,
        curve_data: HashMap<String, Rc<RefCell<VectorData>>>,
        dividend_data: HashMap<String, Rc<RefCell<VectorData>>>,
    ) -> Engine {
        Engine {
            calculation_result: HashMap::new(),
            calculation_configuration: calculation_configuration.clone(),
            evaluation_date: evaluation_date.clone(),
            fxs: fxs.clone(),
            stocks: stocks.clone(),
            curve_data: curve_data.clone(),
            dividend_data: dividend_data.clone(),
            instruments: vec![],
            instruments_in_action: vec![],
            pricer: None,
        }
    }
        
    pub fn set_instuments(&mut self, instruments: Vec<Instrument>) {
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
                inst.get_name().clone(),
                inst.get_code().clone(),
                inst.get_currency().clone(),
                inst.get_type_name().to_string(),
                inst.get_unit_notional(),
                inst.get_maturity().clone(),
            );

            let init_res = CalculationResult::new(
                instrument_information,
                self.evaluation_date.borrow().get_date_clone(),
            );

            self.calculation_result.insert(
                inst.get_code().clone(),
                init_res
            );
        }
    }

    pub fn with_instruments(mut self, instruments: Vec<Instrument>) -> Engine {
        self.set_instuments(instruments);
        self
    }

    pub fn with_pricer(mut self, pricer: Box<dyn Pricer>) -> Engine {
        self.pricer = Some(pricer);
        self
    }

    pub fn set_npv(&mut self) {
        let npv: HashMap<String, Real> = self.pricer.as_ref().unwrap().npv(&self.instruments);
        for (code, value) in npv.iter() {
            self.calculation_result.get_mut(code).unwrap().set_npv(*value);
        }
    }

    pub fn get_calculation_result(&self) -> &HashMap<String, CalculationResult> {
        &self.calculation_result
    }

    pub fn get_calculation_result_clone(&self) -> HashMap<String, CalculationResult> {
        self.calculation_result.clone()
    }
}