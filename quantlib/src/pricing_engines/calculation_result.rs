use crate::currency::Currency;
use crate::definitions::{Integer, Real};
use crate::instruments::inst_info::InstInfo;
use crate::pricing_engines::npv_result::NpvResult;
use crate::utils::number_format::{formatted_number, write_number_with_commas};
use anyhow::{anyhow, Result};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use static_id::StaticId;

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
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CalculationResult {
    instrument_info: Option<InstInfo>,
    evaluation_date: Option<OffsetDateTime>,
    npv_result: Option<NpvResult>,
    value: Option<Real>,
    fx_exposure: Option<HashMap<Currency, Real>>,
    delta: Option<HashMap<StaticId, Real>>,
    gamma: Option<HashMap<StaticId, Real>>,
    vega: Option<HashMap<StaticId, Real>>,
    vega_strucure: Option<HashMap<StaticId, Vec<Real>>>, // underlying code -> Vec::<Real> on vega_tenor in CalculationConfiguration
    vega_matrix: Option<HashMap<StaticId, Array2<Real>>>, // underlying code -> Vec<Vec<Real>> vega_matrix
    theta: Option<Real>,
    div_delta: Option<HashMap<StaticId, Real>>,
    div_structure: Option<HashMap<StaticId, Vec<Real>>>, // underlying code -> Vec::<Real> on div_tenor in CalculationConfiguration
    rho: Option<HashMap<StaticId, Real>>,                // Curve Code -> rho
    rho_structure: Option<HashMap<StaticId, Vec<Real>>>, // curve code -> Vec::<Real> on rho_tenor in CalculationConfig
    theta_day: Option<Integer>,
    #[serde(skip)]
    cashflows: Option<HashMap<OffsetDateTime, Real>>, //expected cashflow inbetween
    representation_currency: Option<Currency>,
}

impl std::fmt::Debug for CalculationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        if let Some(ref info) = self.instrument_info {
            writeln!(f, " * instrument  {:?}", info)?;
        }
        if let Some(ref date) = self.evaluation_date {
            writeln!(f, " * evaluation_date: {:?}\n", date.date())?;
        }
        if let Some(ref result) = self.npv_result {
            writeln!(f, " * npv_result: {:?}", result)?;
        }
        if let Some(value) = self.value {
            write!(f, " * value: ")?;
            write_number_with_commas(f, value)?;
            writeln!(f, "\n")?;
        }
        if let Some(ref exposure) = self.fx_exposure {
            writeln!(f, " * fx_exposure: ")?;
            for (currency, value) in exposure {
                write!(f, "        {}: ", currency)?;
                write_number_with_commas(f, *value)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        if let Some(ref delta) = self.delta {
            writeln!(f, " * delta: ")?;
            for (key, value) in delta {
                write!(f, "        {}: ", key)?;
                write_number_with_commas(f, *value)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        // Similar formatting for gamma, vega, vega_structure, theta, div_delta, div_structure, rho, rho_structure
        if let Some(ref gamma) = self.gamma {
            writeln!(f, " * gamma: ")?;
            for (key, value) in gamma {
                write!(f, "        {}: ", key)?;
                write_number_with_commas(f, *value)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(ref theta) = self.theta {
            write!(f, " * theta: ")?;
            write_number_with_commas(f, *theta)?;
            writeln!(f)?;
        }
        writeln!(f)?;

        if let Some(ref vega) = self.vega {
            writeln!(f, " * vega: ")?;
            for (key, value) in vega {
                write!(f, "        {}: ", key)?;
                write_number_with_commas(f, *value)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(ref vega_structure) = self.vega_strucure {
            writeln!(f, " * vega_structure: ")?;
            for (key, value) in vega_structure {
                let vector_sum = value.iter().sum::<Real>();
                write!(f, "        {} (sum = ", key)?;
                write_number_with_commas(f, vector_sum)?;
                write!(f, "): ")?;

                for v in value {
                    write_number_with_commas(f, *v)?;
                    write!(f, " | ")?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(ref rho) = self.rho {
            writeln!(f, " * rho: ")?;
            for (key, value) in rho {
                write!(f, "        {}: ", key)?;
                write_number_with_commas(f, *value)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(ref rho_structure) = self.rho_structure {
            writeln!(f, " * rho_structure: ")?;
            for (key, value) in rho_structure {
                let vector_sum = value.iter().sum::<Real>();
                write!(f, "        {} (sum = ", key)?;
                write_number_with_commas(f, vector_sum)?;
                write!(f, "): ")?;

                for v in value {
                    write_number_with_commas(f, *v)?;
                    write!(f, " | ")?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(div_delta) = self.div_delta.as_ref() {
            writeln!(f, " * div_delta: ")?;
            for (key, value) in div_delta {
                write!(f, "        {}: ", key)?;
                write_number_with_commas(f, *value)?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(div_structure) = self.div_structure.as_ref() {
            writeln!(f, " * div_structure: ")?;
            for (key, value) in div_structure {
                let vector_sum = value.iter().sum::<Real>();
                write!(f, "        {} (sum = ", key)?;
                write_number_with_commas(f, vector_sum)?;
                write!(f, "): ")?;

                for v in value {
                    write_number_with_commas(f, *v)?;
                    write!(f, " | ")?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if let Some(vega_matrix) = self.vega_matrix.as_ref() {
            writeln!(f, " * vega_matrix: ")?;
            for (key, value) in vega_matrix {
                let matrix_sum: Real = value.iter().fold(0.0, |acc, &x| acc + x);
                write!(f, "        {} (sum = ", key)?;
                write_number_with_commas(f, matrix_sum)?;
                writeln!(f, "): ")?;

                let under_line = "-".repeat((9 + 3) * 17);
                writeln!(f, "{}", under_line)?;
                for row in value.rows() {
                    for element in row {
                        let formatted_number = format!("{:9}", formatted_number(*element));
                        write!(f, "{} | ", formatted_number)?;
                    }
                    writeln!(f)?;
                    writeln!(f, "{}", under_line)?;
                }
            }
            writeln!(f)?;
        }
        if let Some(ref currency) = self.representation_currency {
            writeln!(f, " * representation_currency: {:?}", currency)?;
        }
        writeln!(
            f,
            "==========================================================="
        )
    }
}

impl CalculationResult {
    pub fn new(
        instrument_info: InstrumentInfo,
        evaluation_date: OffsetDateTime,
    ) -> CalculationResult {
        let representation_currency = instrument_info.get_currency();
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
            vega_matrix: None,
            theta: None,
            div_delta: None,
            div_structure: None,
            rho: None,
            rho_structure: None,
            theta_day: None,
            cashflows: None,
            representation_currency: Some(representation_currency),
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
                let unit = self
                    .instrument_info
                    .as_ref()
                    .ok_or_else(|| anyhow!("instrument info is not set"))?
                    .get_unit_notional();
                self.value = Some(npv.get_npv() * unit);
                Ok(())
            }
        }
    }

    pub fn get_value(&self) -> Option<Real> {
        self.value
    }

    pub fn set_fx_exposure(&mut self, fx_exposure: HashMap<Currency, Real>) {
        self.fx_exposure = Some(fx_exposure);
    }

    /// insert delta to self.delta as und_code as its key
    /// if the key is already in the map, it will be updated
    pub fn set_single_delta(&mut self, und_code: &str, v: Real) {
        match &mut self.delta {
            None => {
                let mut delta = HashMap::new();
                delta.insert(und_code.to_owned(), v);
                self.delta = Some(delta);
            }
            Some(delta) => {
                delta.insert(und_code.to_owned(), v);
            }
        }
    }

    pub fn set_single_gamma(&mut self, und_code: &str, v: Real) {
        match &mut self.gamma {
            None => {
                let mut gamma = HashMap::new();
                gamma.insert(und_code.to_owned(), v);
                self.gamma = Some(gamma);
            }
            Some(gamma) => {
                gamma.insert(und_code.to_owned(), v);
            }
        }
    }

    pub fn set_single_vega_structure(&mut self, und_code: &str, vega_structure: Vec<Real>) {
        match &mut self.vega_strucure {
            None => {
                let mut vega_structure_map = HashMap::new();
                vega_structure_map.insert(und_code.to_owned(), vega_structure);
                self.vega_strucure = Some(vega_structure_map);
            }
            Some(vega_structure_map) => {
                vega_structure_map.insert(und_code.to_owned(), vega_structure);
            }
        }
    }

    pub fn set_single_vega(&mut self, und_code: &str, v: Real) {
        match &mut self.vega {
            None => {
                let mut vega = HashMap::new();
                vega.insert(und_code.to_owned(), v);
                self.vega = Some(vega);
            }
            Some(vega) => {
                vega.insert(und_code.to_owned(), v);
            }
        }
    }

    pub fn set_single_rho(&mut self, curve_code: &str, v: Real) {
        match &mut self.rho {
            None => {
                let mut rho = HashMap::new();
                rho.insert(curve_code.to_owned(), v);
                self.rho = Some(rho);
            }
            Some(rho) => {
                rho.insert(curve_code.to_owned(), v);
            }
        }
    }

    pub fn set_single_rho_structure(&mut self, curve_code: &str, rho_structure: Vec<Real>) {
        match &mut self.rho_structure {
            None => {
                let mut rho_structure_map = HashMap::new();
                rho_structure_map.insert(curve_code.to_owned(), rho_structure);
                self.rho_structure = Some(rho_structure_map);
            }
            Some(rho_structure_map) => {
                rho_structure_map.insert(curve_code.to_owned(), rho_structure);
            }
        }
    }

    pub fn set_single_div_delta(&mut self, und_code: &str, v: Real) {
        match &mut self.div_delta {
            None => {
                let mut div_delta = HashMap::new();
                div_delta.insert(und_code.to_owned(), v);
                self.div_delta = Some(div_delta);
            }
            Some(div_delta) => {
                div_delta.insert(und_code.to_owned(), v);
            }
        }
    }

    pub fn set_single_div_structure(&mut self, und_code: &str, div_structure: Vec<Real>) {
        match &mut self.div_structure {
            None => {
                let mut div_structure_map = HashMap::new();
                div_structure_map.insert(und_code.to_owned(), div_structure);
                self.div_structure = Some(div_structure_map);
            }
            Some(div_structure_map) => {
                div_structure_map.insert(und_code.to_owned(), div_structure);
            }
        }
    }

    pub fn set_theta_day(&mut self, theta_day: Integer) {
        self.theta_day = Some(theta_day);
    }

    pub fn set_theta(&mut self, theta: Real) {
        self.theta = Some(theta);
    }

    pub fn set_cashflows(&mut self, cashflows: HashMap<OffsetDateTime, Real>) {
        self.cashflows = Some(cashflows);
    }

    pub fn get_instrument_info(&self) -> Option<&InstrumentInfo> {
        self.instrument_info.as_ref()
    }

    pub fn get_evaluation_date(&self) -> Option<&OffsetDateTime> {
        self.evaluation_date.as_ref()
    }

    pub fn get_fx_exposure(&self) -> Option<&HashMap<Currency, Real>> {
        self.fx_exposure.as_ref()
    }

    pub fn get_delta(&self) -> Option<&HashMap<StaticId, Real>> {
        self.delta.as_ref()
    }

    pub fn get_gamma(&self) -> Option<&HashMap<StaticId, Real>> {
        self.gamma.as_ref()
    }

    pub fn get_vega(&self) -> Option<&HashMap<StaticId, Real>> {
        self.vega.as_ref()
    }

    pub fn get_vega_structure(&self) -> Option<&HashMap<StaticId, Vec<Real>>> {
        self.vega_strucure.as_ref()
    }

    pub fn get_vega_matrix(&self) -> Option<&HashMap<StaticId, Array2<Real>>> {
        self.vega_matrix.as_ref()
    }

    pub fn get_theta(&self) -> Option<Real> {
        self.theta
    }

    pub fn get_rho(&self) -> Option<&HashMap<StaticId, Real>> {
        self.rho.as_ref()
    }

    pub fn get_rho_structure(&self) -> Option<&HashMap<StaticId, Vec<Real>>> {
        self.rho_structure.as_ref()
    }

    pub fn get_cashflows(&self) -> Option<&HashMap<OffsetDateTime, Real>> {
        self.cashflows.as_ref()
    }

    pub fn get_div_delta(&self) -> Option<&HashMap<StaticId, Real>> {
        self.div_delta.as_ref()
    }

    pub fn get_div_structure(&self) -> Option<&HashMap<StaticId, Vec<Real>>> {
        self.div_structure.as_ref()
    }

    pub fn set_representation_currency(&mut self, currency: Currency) {
        self.representation_currency = Some(currency);
    }

    pub fn set_single_vega_matrix(&mut self, und_code: &str, vega_matrix: Array2<Real>) {
        match &mut self.vega_matrix {
            None => {
                let mut vega_matrix_map = HashMap::new();
                vega_matrix_map.insert(und_code.to_owned(), vega_matrix);
                self.vega_matrix = Some(vega_matrix_map);
            }
            Some(vega_matrix_map) => {
                vega_matrix_map.insert(und_code.to_owned(), vega_matrix);
            }
        }
    }

    pub fn representation_currency_conversion(
        &self,
        currency: Currency,
        fx_rate: Real,
    ) -> Result<CalculationResult> {
        if currency == *self.representation_currency.as_ref().unwrap() {
            return Ok(self.clone());
        }

        let instrument_info = self.instrument_info.clone();
        let evaluation_date = self.evaluation_date;
        let npv_result = self.npv_result.clone();
        let value = self.value.map(|x| x * fx_rate);
        let fx_exposure: Option<HashMap<Currency, Real>> = match &self.fx_exposure {
            Some(exposure) => {
                let mut new_exposure = HashMap::new();
                for (c, v) in exposure {
                    new_exposure.insert(*c, v * fx_rate);
                }
                Some(new_exposure)
            }
            None => None,
        };

        let delta: Option<HashMap<StaticId, f32>> = match &self.delta {
            Some(delta) => {
                let mut new_delta = HashMap::new();
                for (und_code, v) in delta {
                    new_delta.insert(und_code.clone(), v * fx_rate);
                }
                Some(new_delta)
            }
            None => None,
        };

        let gamma: Option<HashMap<StaticId, f32>> = match &self.gamma {
            Some(gamma) => {
                let mut new_gamma = HashMap::new();
                for (und_code, v) in gamma {
                    new_gamma.insert(und_code.clone(), v * fx_rate);
                }
                Some(new_gamma)
            }
            None => None,
        };

        let vega: Option<HashMap<StaticId, Real>> = match &self.vega {
            Some(vega) => {
                let mut new_vega = HashMap::new();
                for (und_code, v) in vega {
                    new_vega.insert(und_code.clone(), v * fx_rate);
                }
                Some(new_vega)
            }
            None => None,
        };
        let vega_strucure: Option<HashMap<StaticId, Vec<Real>>> = match &self.vega_strucure {
            Some(vega_structure) => {
                let mut new_vega_structure = HashMap::new();
                for (und_code, v) in vega_structure {
                    let new_v = v.iter().map(|x| x * fx_rate).collect();
                    new_vega_structure.insert(und_code.clone(), new_v);
                }
                Some(new_vega_structure)
            }
            None => None,
        };
        let vega_matrix: Option<HashMap<StaticId, Array2<Real>>> = match &self.vega_matrix {
            Some(vega_matrix) => {
                let mut new_vega_matrix = HashMap::new();
                for (und_code, v) in vega_matrix {
                    let new_v = v.mapv(|x| x * fx_rate);
                    new_vega_matrix.insert(und_code.clone(), new_v);
                }
                Some(new_vega_matrix)
            }
            None => None,
        };

        let theta: Option<Real> = self.theta.map(|x| x * fx_rate);
        let div_delta: Option<HashMap<StaticId, Real>> = match &self.div_delta {
            Some(div_delta) => {
                let mut new_div_delta = HashMap::new();
                for (und_code, v) in div_delta {
                    new_div_delta.insert(und_code.clone(), v * fx_rate);
                }
                Some(new_div_delta)
            }
            None => None,
        };
        let div_structure: Option<HashMap<StaticId, Vec<Real>>> = match &self.div_structure {
            Some(div_structure) => {
                let mut new_div_structure: HashMap<StaticId, Vec<f32>> = HashMap::new();
                for (und_code, v) in div_structure {
                    let new_v = v.iter().map(|x| x * fx_rate).collect();
                    new_div_structure.insert(und_code.clone(), new_v);
                }
                Some(new_div_structure)
            }
            None => None,
        };
        let rho: Option<HashMap<StaticId, Real>> = match &self.rho {
            Some(rho) => {
                let mut new_rho = HashMap::new();
                for (curve_code, v) in rho {
                    new_rho.insert(curve_code.clone(), v * fx_rate);
                }
                Some(new_rho)
            }
            None => None,
        };
        let rho_structure: Option<HashMap<StaticId, Vec<Real>>> = match &self.rho_structure {
            Some(rho_structure) => {
                let mut new_rho_structure = HashMap::new();
                for (curve_code, v) in rho_structure {
                    let new_v = v.iter().map(|x| x * fx_rate).collect();
                    new_rho_structure.insert(curve_code.clone(), new_v);
                }
                Some(new_rho_structure)
            }
            None => None,
        };
        let theta_day: Option<Integer> = self.theta_day;
        let cashflows: Option<HashMap<OffsetDateTime, Real>> = self.cashflows.clone();
        let representation_currency: Option<Currency> = Some(currency);

        let result = CalculationResult {
            instrument_info,
            evaluation_date,
            npv_result,
            value,
            fx_exposure,
            delta,
            gamma,
            vega,
            vega_strucure,
            vega_matrix,
            theta,
            div_delta,
            div_structure,
            rho,
            rho_structure,
            theta_day,
            cashflows,
            representation_currency,
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::InstrumentTrait;
    use crate::{
        currency::Currency,
        instrument::Instrument,
        instruments::{futures::Futures, inst_info::InstInfo},
    };
    use time::macros::datetime;

    #[test]
    fn test_calculation_result() {
        let instrument = InstrumentInfo::default();
        let evaluation_date = datetime!(2021-01-01 00:00:00 +00:00);
        let mut result = CalculationResult::new(instrument, evaluation_date);
        result.set_npv(NpvResult::new_from_npv(100.0));

        assert_eq!(result.get_npv_result().unwrap().get_npv(), 100.0);
    }

    #[test] // test serialization
    fn test_calculation_result_serialization() {
        let equity_futures = Futures::new(
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

        let inst = Instrument::Futures(equity_futures);

        let fut_trait = inst;
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
        result.set_npv(NpvResult::new_from_npv(100.0));

        result.set_single_delta(&"KOSPI200".to_string(), 0.1);

        let serialized = serde_json::to_string_pretty(&result).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: CalculationResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, result);
    }
}
