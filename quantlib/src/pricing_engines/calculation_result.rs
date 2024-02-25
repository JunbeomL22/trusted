use crate::instruments::instrument_info::InstrumentInfo;
use crate::definitions::{Real, Integer};
//use crate::evaluation_date::EvaluationDate;
//use time::Duration;
//use std::rc::Rc;
//use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};

/// CalculationResult is a struct that holds the result of the calculation.
/// It is used to store the result of the calculation of the pricing engine.
/// instrument: InstrumentInfo
/// evaluation_date: OffsetDateTime
/// npv: Option<Real>
/// delta: Option<HashMap<String, Real>>
/// gamma: Option<HashMap<String, Real>>
/// vega: Option<HashMap<String, Real>>
/// vega_strucure: Option<HashMap<String, HashMap<Duration, Real>>>
/// theta: Option<HashMap<String, Real>>
/// rho: Option<HashMap<String, Real>>
/// rho_structure: Option<HashMap<String, HashMap<Duration, Real>>
/// theta_day: Option<Integer>
/// cashflow_inbetween: Option<HashMap<OffsetDateTime, Real>>
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculationResult {
    instrument: InstrumentInfo,
    evaluation_date: OffsetDateTime,
    npv: Option<Real>, 
    delta: Option<HashMap<String, Real>>,
    gamma: Option<HashMap<String, Real>>,
    vega: Option<HashMap<String, Real>>,
    vega_strucure: Option<HashMap<String, HashMap<Duration, Real>>>,
    theta: Option<HashMap<String, Real>>,
    rho: Option<HashMap<String, Real>>,
    rho_structure: Option<HashMap<String, HashMap<Duration, Real>>>,
    theta_day: Option<Integer>,
    cashflow_inbetween: Option<HashMap<OffsetDateTime, Real>>,
}

impl Default for CalculationResult {
    fn default() -> CalculationResult {
        CalculationResult {
            instrument: InstrumentInfo::default(),
            evaluation_date: OffsetDateTime::now_utc(),
            npv: None,
            delta: None,
            gamma: None,
            vega: None,
            vega_strucure: None,
            theta: None,
            rho: None,
            rho_structure: None,
            theta_day: None,
            cashflow_inbetween: None,
        }
    }
}


impl CalculationResult {
    pub fn new(instrument: InstrumentInfo, evaluation_date: OffsetDateTime) -> CalculationResult {
        CalculationResult {
            instrument,
            evaluation_date,
            npv: None,
            delta: None,
            gamma: None,
            vega: None,
            vega_strucure: None,
            theta: None,
            rho: None,
            rho_structure: None,
            theta_day: None,
            cashflow_inbetween: None,
        }
    }
    pub fn set_npv(&mut self, npv: Real) {
        self.npv = Some(npv);
    }

    pub fn set_delta(&mut self, delta: HashMap<String, Real>) {
        self.delta = Some(delta);
    }

    pub fn set_gamma(&mut self, gamma: HashMap<String, Real>) {
        self.gamma = Some(gamma);
    }

    pub fn set_vega(&mut self, vega: HashMap<String, Real>) {
        self.vega = Some(vega);
    }

    pub fn set_vega_structure(&mut self, vega_structure: HashMap<String, HashMap<Duration, Real>>) {
        self.vega_strucure = Some(vega_structure);
    }

    pub fn set_theta(&mut self, theta: HashMap<String, Real>) {
        self.theta = Some(theta);
    }

    pub fn set_rho(&mut self, rho: HashMap<String, Real>) {
        self.rho = Some(rho);
    }

    pub fn set_rho_structure(&mut self, rho_structure: HashMap<String, HashMap<Duration, Real>>) {
        self.rho_structure = Some(rho_structure);
    }

    pub fn get_instrument(&self) -> &InstrumentInfo {
        &self.instrument
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assets::currency::Currency, instruments::{instrument_info::InstrumentInfo, stock_futures::StockFutures}};
    use time::macros::datetime;
    use std::collections::HashMap;

    #[test]
    fn test_calculation_result() {
        let instrument = InstrumentInfo::default();
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(100.0);

        let mut delta = HashMap::new();
        delta.insert("KRW".to_string(), 0.1);
        result.set_delta(delta);

        let mut gamma = HashMap::new();
        gamma.insert("KRW".to_string(), 0.2);

        result.set_gamma(gamma);
        let mut vega = HashMap::new();
        vega.insert("KRW".to_string(), 0.3);
        result.set_vega(vega);
        let mut vega_structure = HashMap::new();
        let mut vega_structure_inner = HashMap::new();
        vega_structure_inner.insert(Duration::days(1), 0.4);
        vega_structure.insert("KRW".to_string(), vega_structure_inner);
        result.set_vega_structure(vega_structure);
        let mut theta = HashMap::new();
        theta.insert("KRW".to_string(), 0.5);
        result.set_theta(theta);
        let mut rho = HashMap::new();
        rho.insert("KRW".to_string(), 0.6);
        result.set_rho(rho);
        let mut rho_structure = HashMap::new();
        let mut rho_structure_inner = HashMap::new();
        rho_structure_inner.insert(Duration::days(1), 0.7);
        rho_structure.insert("KRW".to_string(), rho_structure_inner);
        result.set_rho_structure(rho_structure);
        assert_eq!(result.npv.unwrap(), 100.0);
        assert_eq!(result.delta.unwrap().get("KRW").unwrap(), &0.1);
        assert_eq!(result.gamma.unwrap().get("KRW").unwrap(), &0.2);
        assert_eq!(result.vega.unwrap().get("KRW").unwrap(), &0.3);
        assert_eq!(result.vega_strucure.unwrap().get("KRW").unwrap().get(&Duration::days(1)).unwrap(), &0.4);
        assert_eq!(result.theta.unwrap().get("KRW").unwrap(), &0.5);
        assert_eq!(result.rho.unwrap().get("KRW").unwrap(), &0.6);
        assert_eq!(result.rho_structure.unwrap().get("KRW").unwrap().get(&Duration::days(1)).unwrap(), &0.7);
    }

    #[test] // test serialization
    fn test_calculation_result_serialization() {
        let stock_futures = StockFutures::new(
            datetime!(2021-01-01 09:00:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            datetime!(2022-01-01 15:40:00 +09:00),
            100.0,
            Currency::KRW,
            "KOSPI200".to_string(),
            "KOSPI200".to_string(),
            "KOSPI200".to_string(),
        );

        let instrument = InstrumentInfo::new(Box::new(stock_futures));
        
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(100.0);

        let mut delta = HashMap::new();
        delta.insert("KOSPI200".to_string(), 0.1);
        result.set_delta(delta);

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: CalculationResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, result);
    }
}