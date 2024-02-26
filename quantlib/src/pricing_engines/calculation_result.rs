use crate::instruments::instrument_info::InstrumentInfo;
use crate::definitions::{Real, Integer};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};
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
/// 
/// delta: Option<HashMap<String, Real>>: 1% PnL delta
/// gamma: Option<HashMap<String, Real>>: 1% PnL gamma
/// vega: Option<HashMap<String, Real>>: 1% PnL vega
/// vega_strucure: Option<HashMap<String, HashMap<Duration, Real>>>: 1% PnL vegas
/// theta: Option<HashMap<String, Real>>: 1 day PnL
/// rho: Option<HashMap<String, Real>>: 1bp PnL rho
/// rho_structure: Option<HashMap<String, HashMap<Duration, Real>>: 1bp PnL rhos
/// theta_day: Option<Integer>
/// cashflow_inbetween: Option<HashMap<OffsetDateTime, Real>>
/// cashflow in (evaluation_date, evaluation_date + theta)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculationResult {
    instrument: InstrumentInfo,
    evaluation_date: OffsetDateTime,
    npv: Option<Real>, 
    value: Option<Real>,
    fx_exposure: Option<Real>,
    delta: Option<HashMap<String, Real>>,
    gamma: Option<HashMap<String, Real>>,
    vega: Option<HashMap<String, Real>>,
    vega_strucure: Option<HashMap<String, HashMap<Duration, Real>>>, // underlying code -> duration -> vega
    theta: Option<Real>,
    rho: Option<HashMap<String, Real>>, // Curve Code -> rho
    rho_structure: Option<HashMap<String, HashMap<Duration, Real>>>, // curve code -> duration -> rho
    theta_day: Option<Integer>,
    cashflow_inbetween: Option<HashMap<OffsetDateTime, Real>>,
}

impl Default for CalculationResult {
    fn default() -> CalculationResult {
        CalculationResult {
            instrument: InstrumentInfo::default(),
            evaluation_date: OffsetDateTime::now_utc(),
            npv: None,
            value: None,
            fx_exposure: None,
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
            value: None,
            fx_exposure: None,
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

    pub fn get_npv(&self) -> Option<Real> {
        self.npv
    }

    pub fn set_value(&mut self) {
        self.value = Some(self.npv.expect("npv is not set") * self.instrument.get_unit_notional());
    }

    pub fn get_value(&self) -> Option<Real> {
        self.value
    }

    pub fn set_fx_exposure(&mut self, fx_exposure: Real) {
        self.fx_exposure = Some(fx_exposure);
    }

    pub fn set_delta(&mut self, delta: HashMap<String, Real>) {
        self.delta = Some(delta);
    }

    pub fn set_gamma(&mut self, gamma: HashMap<String, Real>) {
        self.gamma = Some(gamma);
    }

    pub fn set_theta_day(&mut self, theta_day: Integer) {
        self.theta_day = Some(theta_day);
    }

    pub fn set_vega(&mut self, vega: HashMap<String, Real>) {
        self.vega = Some(vega);
    }

    pub fn set_vega_structure(&mut self, vega_structure: HashMap<String, HashMap<Duration, Real>>) {
        self.vega_strucure = Some(vega_structure);
    }

    pub fn set_theta(&mut self, theta: Real) {
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

    pub fn get_evaluation_date(&self) -> &OffsetDateTime {
        &self.evaluation_date
    }

    pub fn get_fx_exposure(&self) -> Option<Real> {
        self.fx_exposure
    }

    pub fn get_delta(&self) -> &Option<HashMap<String, Real>> {
        &self.delta
    }

    pub fn get_gamma(&self) -> &Option<HashMap<String, Real>> {
        &self.gamma
    }

    pub fn get_vega_structure(&self) -> &Option<HashMap<String, HashMap<Duration, Real>>> {
        &self.vega_strucure
    }

    pub fn get_theta(&self) -> Option<Real> {
        self.theta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assets::currency::Currency, instrument::Instrument, instruments::{instrument_info::InstrumentInfo, stock_futures::StockFutures}};
    use time::macros::datetime;
    use std::collections::HashMap;

    #[test]
    fn test_calculation_result() {
        let instrument = InstrumentInfo::default();
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(100.0);

        assert_eq!(result.get_npv(), Some(100.0));
        
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
            "KOSPI200".to_string(),
            "KOSPI200".to_string(),
            "KOSPI200".to_string(),
        );

        let instrument = InstrumentInfo::new(
            stock_futures.get_name().clone(),
            stock_futures.get_code().clone(),
            stock_futures.get_currency().clone(),
            stock_futures.type_name().to_string(),
            stock_futures.get_unit_notional(),
            Some(stock_futures.get_maturity().clone()),
        );
        
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(100.0);

        let mut delta = HashMap::new();
        delta.insert("KOSPI200".to_string(), 0.1);
        result.set_delta(delta);

        let serialized = serde_json::to_string_pretty(&result).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: CalculationResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, result);
    }
}