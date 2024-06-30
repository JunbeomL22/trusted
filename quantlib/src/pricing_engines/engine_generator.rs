use crate::currency::{
    Currency,
    FxCode,
};
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{
    InstrumentTrait,
    Instrument,
    Instruments,
};
use crate::pricing_engines::{
    calculation_configuration::CalculationConfiguration,
    calculation_result::CalculationResult,
    match_parameter::MatchParameter,
    engine::Engine,
};
use crate::data::{
    value_data::ValueData,
    vector_data::VectorData,
    surface_data::SurfaceData,
    daily_value_data::DailyValueData,
};
//
use std::{
    sync::{
        Arc,
        Mutex,
    },
    collections::HashMap,
    //thread,
};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use anyhow::{
    Result,
    anyhow,
};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstrumentCategory {
    pub type_names: Option<Vec<String>>,
    pub currency: Option<Vec<Currency>>,
    pub underlying_codes: Option<Vec<String>>,
}

impl InstrumentCategory {
    pub fn new(
        type_names: Option<Vec<String>>,
        currency: Option<Vec<Currency>>,
        underlying_codes: Option<Vec<String>>,
    ) -> InstrumentCategory {
        InstrumentCategory {
            type_names,
            currency,
            underlying_codes,
        }
    }

    pub fn contains(&self, instrument: &Instrument) -> Result<bool> {
        let instrument_type_inp = instrument.get_type_name().to_string();
        let currency_inp = instrument.get_currency();
        let underlying_codes_inp = instrument.get_underlying_codes();

        let mut res: bool = true;
        // check instrument type is in type_names
        if let Some(type_names) = &self.type_names {
            if !type_names.contains(&instrument_type_inp) {
                res = false;
            }
        }
        // check currency is in currency
        if let Some(currency) = &self.currency {
            if !currency.contains(currency_inp) {
                res = false;
            }
        }
        // check underlying codes are the same (not inclusion)
        if let Some(underlying_codes) = &self.underlying_codes {
            if !underlying_codes_inp.is_empty() &&
            underlying_codes.iter().collect::<Vec<&String>>() != underlying_codes_inp {
                res = false;
            }
        }
        // check the utc_offset
        Ok(res)
    }
}

pub struct EngineGenerator {
    instruments: Instruments,
    instrument_group_vec: Vec<Vec<Instrument>>,
    instrument_categories: Vec<InstrumentCategory>,
    //
    calculation_configuration: CalculationConfiguration,
    match_parameter: MatchParameter,
    //
    calculation_results: HashMap<String, CalculationResult>,
    // evaluation date
    evaluation_date: EvaluationDate,
    // data
    fx_data: Arc<HashMap<FxCode, ValueData>>,
    stock_data: Arc<HashMap<String, ValueData>>,
    curve_data: Arc<HashMap<String, VectorData>>,
    dividend_data: Arc<HashMap<String, VectorData>>,
    equity_constant_volatility_data: Arc<HashMap<String, ValueData>>,
    equity_volatility_surface_data: Arc<HashMap<String, SurfaceData>>,
    fx_constant_volatility_data: Arc<HashMap<FxCode, ValueData>>,
    quanto_correlation_data: Arc<HashMap<(String, FxCode), ValueData>>,
    past_daily_value_data: Arc<HashMap<String, DailyValueData>>,
}

impl Default for EngineGenerator {
    fn default() -> Self {
        EngineGenerator {
            instruments: Instruments::default(),
            instrument_group_vec: vec![],
            instrument_categories: vec![],
            //
            calculation_configuration: CalculationConfiguration::default(),
            match_parameter: MatchParameter::default(),
            //
            calculation_results: HashMap::new(),
            //
            evaluation_date: EvaluationDate::default(),
            //
            fx_data: Arc::new(HashMap::new()),
            stock_data: Arc::new(HashMap::new()),
            curve_data: Arc::new(HashMap::new()),
            dividend_data: Arc::new(HashMap::new()),
            equity_constant_volatility_data: Arc::new(HashMap::new()),
            equity_volatility_surface_data: Arc::new(HashMap::new()),
            fx_constant_volatility_data: Arc::new(HashMap::new()),
            quanto_correlation_data: Arc::new(HashMap::new()),
            past_daily_value_data: Arc::new(HashMap::new()),
        }   
    }
}

impl EngineGenerator {
    pub fn builder() -> EngineGenerator {
        EngineGenerator::default()
    }

    pub fn with_configuration(
        &mut self,
        calculation_configuration: CalculationConfiguration,
        evalutation_datetime: OffsetDateTime,
        match_parameter: MatchParameter,
    ) -> Result<&mut Self> {
        self.calculation_configuration = calculation_configuration;
        self.evaluation_date = EvaluationDate::new(evalutation_datetime);
        self.match_parameter = match_parameter;
        Ok(self)
    }

    pub fn with_instruments(&mut self, instruments: Instruments) -> Result<&mut Self> {
        self.instruments = instruments;
        Ok(self)
    }

    pub fn with_instrument_categories(&mut self, instrument_categories: Vec<InstrumentCategory>) -> Result<&mut Self> {
        self.instrument_categories = instrument_categories;
        Ok(self)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_data(
        &mut self,
        fx_data: HashMap<FxCode, ValueData>,
        stock_data: HashMap<String, ValueData>,
        curve_data: HashMap<String, VectorData>,
        dividend_data: HashMap<String, VectorData>,
        equity_constant_volatility_data: HashMap<String, ValueData>,
        equity_volatility_surface_data: HashMap<String, SurfaceData>,
        fx_constant_volatility_data: HashMap<FxCode, ValueData>,
        quanto_correlation_data: HashMap<(String, FxCode), ValueData>,
        past_daily_value_data: HashMap<String, DailyValueData>,
    ) -> Result<&mut Self> {
        self.fx_data = Arc::new(fx_data);
        self.stock_data = Arc::new(stock_data);
        self.curve_data = Arc::new(curve_data);
        self.dividend_data = Arc::new(dividend_data);
        self.equity_constant_volatility_data = Arc::new(equity_constant_volatility_data);
        self.equity_volatility_surface_data = Arc::new(equity_volatility_surface_data);
        self.fx_constant_volatility_data = Arc::new(fx_constant_volatility_data);
        self.quanto_correlation_data = Arc::new(quanto_correlation_data);
        self.past_daily_value_data = Arc::new(past_daily_value_data);
        Ok(self)
    }

    pub fn distribute_instruments(&mut self) -> Result<()> {
        let mut distribution_checker: Vec<bool> = vec![false; self.instruments.len()];

        let mut instrument_group_vec: Vec<Vec<Instrument>> = vec![];
        for instrument_category in &self.instrument_categories {
            let mut instrument_group: Vec<Instrument> = vec![];
            for (inst_id, instrument) in self.instruments.iter().enumerate() {
                if !distribution_checker[inst_id] && instrument_category.contains(instrument)? {
                    instrument_group.push(instrument.as_ref().clone());
                    distribution_checker[inst_id] = true;
                }
            }
            if !instrument_group.is_empty() {
                instrument_group_vec.push(instrument_group);
            }
        }
        
        let mut inst_name_code: Vec<String> = vec![];
        for (inst_id, is_distributed) in distribution_checker.iter().enumerate() {
            if !is_distributed {
                let msg = format!(
                    "{} ({})\n\
                    type: {}\n\
                    currency: {}\n\
                    underlying_codes: {:?}\n",
                    self.instruments[inst_id].get_name(),
                    self.instruments[inst_id].get_code(),
                    self.instruments[inst_id].get_type_name(),
                    self.instruments[inst_id].get_currency(),
                    self.instruments[inst_id].get_underlying_codes(),
                );

                inst_name_code.push(msg);
            }
        }

        if !inst_name_code.is_empty() {
            return Err(anyhow!(
                "The following instruments are not distributed:\n{}",
                inst_name_code.join("\n"),
            ));
        }

        self.instrument_group_vec = instrument_group_vec;

        Ok(())
    }

    /// spawn threads to create engine and calculate
    pub fn calculate(&mut self) -> Result<()> {
        let shared_results = Arc::new(Mutex::new(HashMap::<String, CalculationResult>::new()));
        let dt = self.evaluation_date.get_date_clone();
        let calc_res: Result<()> = self.instrument_group_vec.par_iter().enumerate().map(
            |(group_id, instrument_group)| {
                let engine = Engine::builder(
                    group_id,
                    self.calculation_configuration.clone(),
                    dt,
                    self.match_parameter.clone(),
                );
        
                let engine = match engine.with_instruments(instrument_group.clone()) {
                    Ok(engine) => engine,
                    Err(e) => return Err(e),
                };
        
                let mut engine = match engine.with_parameter_data(
                    self.fx_data.clone(),
                    self.stock_data.clone(),
                    self.curve_data.clone(),
                    self.dividend_data.clone(),
                    self.equity_constant_volatility_data.clone(),
                    self.equity_volatility_surface_data.clone(),
                    self.fx_constant_volatility_data.clone(),
                    self.quanto_correlation_data.clone(),
                    self.past_daily_value_data.clone(),
                ){
                    Ok(engine) => engine,
                    Err(e) => return Err(e),
                };
                
                engine.initialize_pricers()?;
                engine.calculate()?;
        
                let result = engine.get_calculation_result();
                let mut mut_res = shared_results.lock().unwrap();
        
                for (key, value) in result.iter() {
                    mut_res.insert(key.clone(), value.borrow().clone());
                }
        
                Ok(())
            }
        ).collect();

        //self.calculation_results = shared_results.lock().unwrap().clone();
        self.calculation_results.clone_from(&shared_results.lock().unwrap());

        match calc_res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_calculation_results(&self) -> &HashMap<String, CalculationResult> {
        &self.calculation_results
    }
}