use crate::instruments::instrument_info::InstrumentInfo;
use crate::definitions::{Real, Integer};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::pricing_engines::npv_result::NpvResult;
/// CalculationResult is a struct that holds the result of the calculation.
/// It is used to store the result of the calculation of the pricing engine.
/// instrument: InstrumentInfo
/// evaluation_date: OffsetDateTime
///
/// npv: Option<Real>: Net Present Value:
/// exclude cashflow at evaluation date, not considering unit_notional
/// value: Option<Real>: 
/// Value of the instrument considering unit_notional excluding cashflow at evaluation date
/// mostly pnl -> value_2 - value_1 +cashflow_inbetween
/// all greeks are calculated based on value not the npv in other words, considering unit_notional
/// fx_exposure: Option<Real>:
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculationResult {
    instrument_info: Option<InstrumentInfo>,
    evaluation_date: Option<OffsetDateTime>,
    npv_result: Option<NpvResult>, 
    value: Option<Real>,
    fx_exposure: Option<Real>,
    delta: Option<HashMap<String, Real>>,
    gamma: Option<HashMap<String, Real>>,
    vega: Option<HashMap<String, Real>>,
    vega_strucure: Option<HashMap<String, Vec<Real>>>, // underlying code -> Vec::<Real> on vega_tenor in CalculationConfiguration
    theta: Option<Real>,
    div_delta: Option<HashMap<String, Real>>,
    div_structure: Option<HashMap<String, Vec<Real>>>, // underlying code -> Vec::<Real> on div_tenor in CalculationConfiguration
    rho: Option<HashMap<String, Real>>, // Curve Code -> rho
    rho_structure: Option<HashMap<String, Vec<Real>>>, // curve code -> Vec::<Real> on rho_tenor in CalculationConfig
    theta_day: Option<Integer>,
    cashflow_inbetween: Option<HashMap<OffsetDateTime, Real>>, //expected cashflow inbetween
}

impl Default for CalculationResult {
    fn default() -> CalculationResult {
        CalculationResult {
            instrument_info: None,
            evaluation_date: None,
            npv_result: None,
            value: None,
            fx_exposure: None,
            delta: None,
            gamma: None,
            vega: None,
            vega_strucure: None,
            theta: None,
            div_delta: None,
            div_structure: None,
            rho: None,
            rho_structure: None,
            theta_day: None,
            cashflow_inbetween: None,
        }
    }
}


impl CalculationResult {
    pub fn new(instrument_info: InstrumentInfo, evaluation_date: OffsetDateTime) -> CalculationResult {
        CalculationResult {
            instrument_info: Some(instrument_info),
            evaluation_date: Some(evaluation_date),
            npv_result: None,
            value: None,
            fx_exposure: None,
            delta: None,
            gamma: None,
            vega: None,
            vega_strucure: None,
            theta: None,
            div_delta: None,
            div_structure: None,
            rho: None,
            rho_structure: None,
            theta_day: None,
            cashflow_inbetween: None,
        }
    }

    pub fn set_npv(&mut self, npv_result: NpvResult) {
        self.npv_result = Some(npv_result);
    }

    pub fn get_npv_result(&self) -> Option<&NpvResult> {
        self.npv_result.as_ref()
    }

    pub fn set_value(&mut self) -> Result<()> {
        match self.npv_result.as_ref() {
            None => Err(anyhow!("npv result is not set")),
            Some(npv) => {
                let unit = self.instrument_info.as_ref()
                        .ok_or_else(|| anyhow!("instrument info is not set"))?
                        .get_unit_notional();
                self.value = Some(npv.get_npv() * unit);
                Ok(())
            },
        }
    }

    pub fn get_value(&self) -> Option<Real> {
        self.value
    }

    pub fn set_fx_exposure(&mut self, fx_exposure: Real) {
        self.fx_exposure = Some(fx_exposure);
    }

    /// insert delta to self.delta as und_code as its key
    /// if the key is already in the map, it will be updated
    pub fn set_single_delta(&mut self, und_code: &String, v: Real) {
        match &mut self.delta {
            None => {
                let mut delta = HashMap::new();
                delta.insert(und_code.clone(), v);
                self.delta = Some(delta);
            },
            Some(delta) => {
                delta.insert(und_code.clone(), v);
            },
        }
    }

    pub fn set_single_gamma(&mut self, und_code: &String, v: Real) {
        match &mut self.gamma {
            None => {
                let mut gamma = HashMap::new();
                gamma.insert(und_code.clone(), v);
                self.gamma = Some(gamma);
            },
            Some(gamma) => {
                gamma.insert(und_code.clone(), v);
            },
        }
    }

    pub fn set_single_rho(&mut self, curve_code: &String, v: Real) {
        match &mut self.rho {
            None => {
                let mut rho = HashMap::new();
                rho.insert(curve_code.clone(), v);
                self.rho = Some(rho);
            },
            Some(rho) => {
                rho.insert(curve_code.clone(), v);
            },
        }
    }

    pub fn set_single_rho_structure(
        &mut self, 
        curve_code: &String, 
        rho_structure: Vec<Real>,
    ) {
        match &mut self.rho_structure {
            None => {
                let mut rho_structure_map = HashMap::new();
                rho_structure_map.insert(curve_code.clone(), rho_structure);
                self.rho_structure = Some(rho_structure_map);
            },
            Some(rho_structure_map) => {
                rho_structure_map.insert(curve_code.clone(), rho_structure);
            },
        }
    }

    pub fn set_single_div_delta(&mut self, und_code: &String, v: Real) {
        match &mut self.div_delta {
            None => {
                let mut div_delta = HashMap::new();
                div_delta.insert(und_code.clone(), v);
                self.div_delta = Some(div_delta);
            },
            Some(div_delta) => {
                div_delta.insert(und_code.clone(), v);
            },
        }
    }

    pub fn set_single_div_structure(
        &mut self, 
        und_code: &String, 
        div_structure: Vec<Real>,
    ) {
        match &mut self.div_structure {
            None => {
                let mut div_structure_map = HashMap::new();
                div_structure_map.insert(und_code.clone(), div_structure);
                self.div_structure = Some(div_structure_map);
            },
            Some(div_structure_map) => {
                div_structure_map.insert(und_code.clone(), div_structure);
            },
        }
    }

    pub fn set_theta_day(&mut self, theta_day: Integer) {
        self.theta_day = Some(theta_day);
    }

    pub fn set_theta(&mut self, theta: Real) {
        self.theta = Some(theta);
    }

    pub fn set_cashflow_inbetween(&mut self, cashflow_inbetween: HashMap<OffsetDateTime, Real>) {
        self.cashflow_inbetween = Some(cashflow_inbetween);
    }
    
    pub fn get_instrument_info(&self) -> Option<&InstrumentInfo> {
        self.instrument_info.as_ref()
    }

    pub fn get_evaluation_date(&self) -> Option<&OffsetDateTime> {
        self.evaluation_date.as_ref()
    }

    pub fn get_fx_exposure(&self) -> Option<Real> {
        self.fx_exposure
    }

    pub fn get_delta(&self) -> Option<&HashMap<String, Real>> {
        self.delta.as_ref()
    }

    pub fn get_gamma(&self) -> Option<&HashMap<String, Real>> {
        self.gamma.as_ref()
    }

    pub fn get_vega_structure(&self) -> Option<&HashMap<String, Vec<Real>>> {
        self.vega_strucure.as_ref()
    }

    pub fn get_theta(&self) -> Option<Real> {
        self.theta
    }

    pub fn get_rho(&self) -> Option<&HashMap<String, Real>> {
        self.rho.as_ref()
    }

    pub fn get_rho_structure(&self) -> Option<&HashMap<String, Vec<Real>>> {
        self.rho_structure.as_ref()
    }

    pub fn get_cashflow_inbetween(&self) -> Option<&HashMap<OffsetDateTime, Real>> {
        self.cashflow_inbetween.as_ref()
    }

    pub fn get_div_delta(&self) -> Option<&HashMap<String, Real>> {
        self.div_delta.as_ref()
    }

    pub fn get_div_structure(&self) -> Option<&HashMap<String, Vec<Real>>> {
        self.div_structure.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assets::currency::Currency, 
        instrument::Instrument,
        instruments::{
            instrument_info::InstrumentInfo, 
            stock_futures::StockFutures
        },
    };
    use time::macros::datetime;

    #[test]
    fn test_calculation_result() {
        let instrument = InstrumentInfo::default();
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(NpvResult::new(100.0));

        assert_eq!(result.get_npv_result().unwrap().get_npv(), 100.0);
        
    }

    #[test] // test serialization
    fn test_calculation_result_serialization() {
        let stock_futures = StockFutures::new(
            300.0,
            datetime!(2021-01-01 09:00:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI200".to_string(),
            "KOSPI200".to_string(),
            "KOSPI200".to_string(),
        );

        let inst = Instrument::StockFutures(Box::new(stock_futures));

        let fut_trait = inst.as_trait();
        let name = fut_trait.get_name();
        let code = fut_trait.get_code();
        let type_name = fut_trait.get_type_name();
        let maturity = fut_trait.get_maturity();
        
        let instrument = InstrumentInfo::new(
            name.clone(),
            code.clone(),
            type_name,
            *fut_trait.get_currency(),
            fut_trait.get_unit_notional(),
            maturity,
        );
        
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(NpvResult::new(100.0));
        
        result.set_single_delta(&"KOSPI200".to_string(), 0.1);


        let serialized = serde_json::to_string_pretty(&result).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: CalculationResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, result);
    }
}