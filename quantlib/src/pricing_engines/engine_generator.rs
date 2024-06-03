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
    sync::Arc,
    collections::HashMap,
    thread,
};
use serde::{Deserialize, Serialize};
use anyhow::{
    Result,
    anyhow,
    Context,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentCategory {
    pub type_names: Option<Vec<String>>,
    pub currency: Option<Vec<Currency>>,
    pub underlying_codes: Option<Vec<String>>,
}

impl Default for InstrumentCategory {
    fn default() -> InstrumentCategory {
        InstrumentCategory {
            type_names: None,
            currency: None,
            underlying_codes: None,
        }
    }
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
            if underlying_codes.iter().collect::<Vec<&String>>() != underlying_codes_inp {
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
    calculation_configuration: CalculationConfiguration,
    match_parameter: MatchParameter,
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
    // engines
    engines: Vec<Engine>,
}

impl Default for EngineGenerator {
    fn default() -> Self {
        EngineGenerator {
            instruments: Instruments::default(),
            instrument_group_vec: vec![],
            instrument_categories: vec![],
            calculation_configuration: CalculationConfiguration::default(),
            match_parameter: MatchParameter::default(),
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
            engines: vec![],
        }   
    }
}

impl EngineGenerator {
    pub fn initialize() -> EngineGenerator {
        EngineGenerator::default()
    }

    pub fn with_configuration(
        &mut self,
        calculation_configuration: CalculationConfiguration,
        match_parameter: MatchParameter,
    ) -> Result<&mut Self> {
        self.calculation_configuration = calculation_configuration;
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

    pub fn with_data(
        &mut self,
        fx_data: Arc<HashMap<FxCode, ValueData>>,
        stock_data: Arc<HashMap<String, ValueData>>,
        curve_data: Arc<HashMap<String, VectorData>>,
        dividend_data: Arc<HashMap<String, VectorData>>,
        equity_constant_volatility_data: Arc<HashMap<String, ValueData>>,
        equity_volatility_surface_data: Arc<HashMap<String, SurfaceData>>,
        fx_constant_volatility_data: Arc<HashMap<FxCode, ValueData>>,
        quanto_correlation_data: Arc<HashMap<(String, FxCode), ValueData>>,
        past_daily_value_data: Arc<HashMap<String, DailyValueData>>,
    ) -> Result<&mut Self> {
        self.fx_data = fx_data;
        self.stock_data = stock_data;
        self.curve_data = curve_data;
        self.dividend_data = dividend_data;
        self.equity_constant_volatility_data = equity_constant_volatility_data;
        self.equity_volatility_surface_data = equity_volatility_surface_data;
        self.fx_constant_volatility_data = fx_constant_volatility_data;
        self.quanto_correlation_data = quanto_correlation_data;
        self.past_daily_value_data = past_daily_value_data;
        Ok(self)
    }

    pub fn distribute_instruments(&mut self) -> Result<()> {
        let mut distribution_checker: Vec<bool> = vec![false; self.instruments.len()];

        let mut instrument_group_vec: Vec<Vec<Instrument>> = vec![];
        for instrument_category in &self.instrument_categories {
            let mut instrument_group: Vec<Instrument> = vec![];
            for (inst_id, instrument) in self.instruments.iter().enumerate() {
                if instrument_category.contains(instrument)? {
                    instrument_group.push(instrument.as_ref().clone());
                    if distribution_checker[inst_id] {
                        return Err(anyhow!(
                            "The instrument {} ({})is already distributed",
                            instrument.get_name(),
                            instrument.get_code(),
                        ));
                    }
                    distribution_checker[inst_id] = true;
                }
            }
            instrument_group_vec.push(instrument_group);
        }
        
        let mut inst_name_code: Vec<String> = vec![];
        for (inst_id, is_distributed) in distribution_checker.iter().enumerate() {
            if !is_distributed {
                let msg = format!(
                    "{} ({})"
                    self.instruments[inst_id].get_name(),
                    self.instruments[inst_id].get_code(),
                );

                inst_name_code.push(msg);
            }
        }

        if !inst_name_code.is_empty() {
            return Err(anyhow!(
                "The following instruments are not distributed: {}",
                inst_name_code.join("\n"),
            ));
        }

        self.instrument_group_vec = instrument_group_vec;

        Ok(())
    }

    pub fn distribute_engines(&mut self) -> Result<()> {
        let mut engines: Vec<Engine> = vec![];
        for (id, instrument_group) in self.instrument_group_vec.iter().enumerate() {
            let engine = Engine::builder(
                id,
                self.calculation_configuration.clone(),
                self.evaluation_date.clone(),
                self.match_parameter.clone(),
            ).with_instruments(instrument_group.clone())?
            .with_parameter_data(
                self.fx_data.clone(),
                self.stock_data.clone(),
                self.curve_data.clone(),
                self.dividend_data.clone(),
                self.equity_constant_volatility_data.clone(),
                self.equity_volatility_surface_data.clone(),
                self.fx_constant_volatility_data.clone(),
                self.quanto_correlation_data.clone(),
                self.past_daily_value_data.clone(),
            )?;
            engines.push(engine);
        }
        self.engines = engines;
        Ok(())
    }
}