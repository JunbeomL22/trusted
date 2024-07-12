use crate::definitions::{Integer, Real};
use crate::enums::{StickynessType, VanillaOptionCalculationMethod};
use crate::parameters::volatilities::volatiltiy_interpolator::VolatilityInterplator;
use anyhow::{anyhow, Result};
use ndarray::Array1;
use serde::{Deserialize, Serialize};
/// CalculationConfiguration is a struct that holds the configuration of the calculation.
/// stickyness_type: StickynessType
/// StickynessType is an enum that represents the stickyness of the calculation.
/// If the stickyness_type is StickyToMoneyness, the delta will be calculated with respect to moneyness:
/// In other words, delta = dV/dS + dvol/dS * dV/dvols
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculationConfiguration {
    npv: bool,
    fx_exposure: bool,
    delta: bool,
    gamma: bool,
    vega: bool,
    rho: bool,
    div_delta: bool,
    theta: bool,
    vega_strucure: bool,
    rho_structure: bool,
    div_structure: bool,
    vega_matrix: bool,
    //
    stickyness_type: StickynessType,
    lv_interpolator: VolatilityInterplator,
    //
    delta_bump_ratio: Real,
    gamma_bump_ratio: Real,
    vega_bump_value: Real,
    vega_structure_bump_value: Real,
    vega_matrix_bump_value: Real,
    rho_bump_value: Real,
    div_bump_value: Real,
    theta_day: Integer,
    //
    rho_structure_tenors: Vec<String>,
    vega_structure_tenors: Vec<String>,
    div_structure_tenors: Vec<String>,
    vega_matrix_spot_moneyness: Array1<Real>,
    //
    vanilla_option_calculation_method: VanillaOptionCalculationMethod,
    //
}

impl Default for CalculationConfiguration {
    fn default() -> CalculationConfiguration {
        let rho_tenors = vec![
            "1M".to_string(),
            "2M".to_string(),
            "3M".to_string(),
            "6M".to_string(),
            "9M".to_string(),
            "1Y".to_string(),
            "1Y6M".to_string(),
            "2Y".to_string(),
            "3Y".to_string(),
            "5Y".to_string(),
            "7Y".to_string(),
            "10Y".to_string(),
            "15Y".to_string(),
            "20Y".to_string(),
            "30Y".to_string(),
        ];
        let div_tenors = vec![
            "1M".to_string(),
            "2M".to_string(),
            "3M".to_string(),
            "6M".to_string(),
            "9M".to_string(),
            "1Y".to_string(),
            "1Y6M".to_string(),
            "2Y".to_string(),
            "3Y".to_string(),
        ];
        let vega_tenors = vec![
            "1M".to_string(),
            "2M".to_string(),
            "3M".to_string(),
            "6M".to_string(),
            "9M".to_string(),
            "1Y".to_string(),
            "1Y6M".to_string(),
            "2Y".to_string(),
            "3Y".to_string(),
        ];
        let vega_matrix_spot_moneyness = Array1::linspace(0.6, 1.4, 17);
        CalculationConfiguration {
            npv: true,
            fx_exposure: true,
            delta: false,
            gamma: false,
            vega: false,
            rho: false,
            div_delta: false,
            theta: false,
            vega_strucure: false,
            rho_structure: false,
            div_structure: false,
            vega_matrix: false,
            stickyness_type: StickynessType::StickyToMoneyness,
            lv_interpolator: VolatilityInterplator::default(),
            delta_bump_ratio: 0.01,
            gamma_bump_ratio: 0.01,
            vega_bump_value: 0.01,
            vega_structure_bump_value: 0.01,
            vega_matrix_bump_value: 0.001,
            rho_bump_value: 0.0001,
            div_bump_value: 0.0001,
            theta_day: 1,
            rho_structure_tenors: rho_tenors,
            vega_structure_tenors: vega_tenors,
            div_structure_tenors: div_tenors,
            vega_matrix_spot_moneyness,
            vanilla_option_calculation_method: VanillaOptionCalculationMethod::Analytic,
        }
    }
}

impl CalculationConfiguration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        npv: bool,
        fx_exposure: bool,
        delta: bool,
        gamma: bool,
        vega: bool,
        rho: bool,
        div_delta: bool,
        theta: bool,
        vega_strucure: bool,
        rho_structure: bool,
        div_structure: bool,
        vega_matrix: bool,
        theta_day: Integer,
        stickyness_type: StickynessType,
        lv_interpolator: VolatilityInterplator,
        delta_bump_ratio: Real,
        gamma_bump_ratio: Real,
        vega_bump_value: Real,
        vega_structure_bump_value: Real,
        vega_matrix_bump_value: Real,
        rho_bump_value: Real,
        div_bump_value: Real,
        //
        rho_structure_tenors: Vec<String>,
        vega_structure_tenors: Vec<String>,
        div_structure_tenors: Vec<String>,
        vega_matrix_spot_moneyness: Array1<Real>,
        //
        vanilla_option_calculation_method: VanillaOptionCalculationMethod,
    ) -> Result<CalculationConfiguration> {
        if delta != gamma {
            return Err(anyhow!("delta and gamma must be both true or both false"));
        }
        if delta_bump_ratio <= 0.0 {
            return Err(anyhow!(
                "delta_bump_ratio must be > 0.0, got {}",
                delta_bump_ratio
            ));
        }
        if gamma_bump_ratio <= 0.0 {
            return Err(anyhow!(
                "gamma_bump_ratio must be > 0.0, got {}",
                gamma_bump_ratio
            ));
        }
        if rho_bump_value <= 0.0 {
            return Err(anyhow!("rho_bump must be > 0.0, got {}", rho_bump_value));
        }
        if vega_bump_value <= 0.0 {
            return Err(anyhow!("vega_bump must be > 0.0, got {}", vega_bump_value));
        }

        if vega_structure_bump_value <= 0.0 {
            return Err(anyhow!(
                "vega_structure_bump_value must be > 0.0, got {}",
                vega_structure_bump_value
            ));
        }

        if vega_matrix_bump_value <= 0.0 {
            return Err(anyhow!(
                "vega_matrix_bump_value must be > 0.0, got {}",
                vega_matrix_bump_value
            ));
        }

        if theta_day <= 0 {
            return Err(anyhow!("theta_day must be > 0, got {}", theta_day));
        }

        if div_bump_value <= 0.0 {
            return Err(anyhow!(
                "div_bump_value must be > 0.0, got {}",
                div_bump_value
            ));
        }

        Ok(CalculationConfiguration {
            npv,
            fx_exposure,
            delta,
            gamma,
            vega,
            rho,
            div_delta,
            theta,
            vega_strucure,
            div_structure,
            rho_structure,
            vega_matrix,
            //
            stickyness_type,
            lv_interpolator,
            //
            delta_bump_ratio,
            gamma_bump_ratio,
            vega_bump_value,
            vega_structure_bump_value,
            vega_matrix_bump_value,
            rho_bump_value,
            div_bump_value,
            theta_day,
            rho_structure_tenors,
            vega_structure_tenors,
            div_structure_tenors,
            vega_matrix_spot_moneyness,
            //
            vanilla_option_calculation_method,
        })
    }

    pub fn with_theta_day(mut self, theta_day: Integer) -> CalculationConfiguration {
        self.theta_day = theta_day;
        self
    }

    pub fn with_delta_calculation(mut self, delta: bool) -> CalculationConfiguration {
        self.delta = delta;
        self
    }

    pub fn with_gamma_calculation(mut self, gamma: bool) -> CalculationConfiguration {
        self.gamma = gamma;
        self
    }

    pub fn with_vega_calculation(mut self, vega: bool) -> CalculationConfiguration {
        self.vega = vega;
        self
    }

    pub fn with_vega_structure_calculation(
        mut self,
        vega_structure: bool,
    ) -> CalculationConfiguration {
        self.vega_strucure = vega_structure;
        self
    }

    pub fn with_vega_matrix_calculation(mut self, vega_matrix: bool) -> CalculationConfiguration {
        self.vega_matrix = vega_matrix;
        self
    }

    pub fn with_theta_calculation(mut self, theta: bool) -> CalculationConfiguration {
        self.theta = theta;
        self
    }

    pub fn with_rho_calculation(mut self, rho: bool) -> CalculationConfiguration {
        self.rho = rho;
        self
    }

    pub fn with_rho_structure_calculation(
        mut self,
        rho_structure: bool,
    ) -> CalculationConfiguration {
        self.rho_structure = rho_structure;
        self
    }

    pub fn with_stickyness_type(
        mut self,
        stickyness_type: StickynessType,
    ) -> CalculationConfiguration {
        self.stickyness_type = stickyness_type;
        self
    }

    pub fn with_div_delta_calculation(mut self, div_delta: bool) -> CalculationConfiguration {
        self.div_delta = div_delta;
        self
    }

    pub fn with_div_structure_calculation(
        mut self,
        div_structure: bool,
    ) -> CalculationConfiguration {
        self.div_structure = div_structure;
        self
    }

    pub fn with_delta_bump_ratio(mut self, delta_bump_ratio: Real) -> CalculationConfiguration {
        self.delta_bump_ratio = delta_bump_ratio;
        self
    }

    pub fn with_gamma_bump_ratio(mut self, gamma_bump_ratio: Real) -> CalculationConfiguration {
        self.gamma_bump_ratio = gamma_bump_ratio;
        self
    }

    pub fn with_vega_bump_value(mut self, vega_bump_value: Real) -> CalculationConfiguration {
        self.vega_bump_value = vega_bump_value;
        self
    }

    pub fn with_vega_structure_bump_value(
        mut self,
        vega_structure_bump_value: Real,
    ) -> CalculationConfiguration {
        self.vega_structure_bump_value = vega_structure_bump_value;
        self
    }

    pub fn with_vega_matrix_bump_value(
        mut self,
        vega_matrix_bump_value: Real,
    ) -> CalculationConfiguration {
        self.vega_matrix_bump_value = vega_matrix_bump_value;
        self
    }

    pub fn with_rho_bump_value(mut self, rho_bump_value: Real) -> CalculationConfiguration {
        self.rho_bump_value = rho_bump_value;
        self
    }

    pub fn with_rho_structure_tenors(
        mut self,
        rho_structure_tenors: Vec<String>,
    ) -> CalculationConfiguration {
        self.rho_structure_tenors = rho_structure_tenors;
        self
    }

    pub fn with_vega_structure_tenors(
        mut self,
        vega_structure_tenors: Vec<String>,
    ) -> CalculationConfiguration {
        self.vega_structure_tenors = vega_structure_tenors;
        self
    }

    pub fn with_div_structure_tenors(
        mut self,
        div_structure_tenors: Vec<String>,
    ) -> CalculationConfiguration {
        self.div_structure_tenors = div_structure_tenors;
        self
    }

    pub fn with_vega_matrix_spot_moneyness(
        mut self,
        vega_matrix_spot_moneyness: Array1<Real>,
    ) -> CalculationConfiguration {
        self.vega_matrix_spot_moneyness = vega_matrix_spot_moneyness;
        self
    }

    pub fn with_vanilla_option_calculation_method(
        mut self,
        vanilla_option_calculation_method: VanillaOptionCalculationMethod,
    ) -> CalculationConfiguration {
        self.vanilla_option_calculation_method = vanilla_option_calculation_method;
        self
    }

    pub fn with_lv_interpolator(
        mut self,
        lv_interpolator: VolatilityInterplator,
    ) -> CalculationConfiguration {
        self.lv_interpolator = lv_interpolator;
        self
    }

    pub fn get_vanilla_option_calculation_method(&self) -> VanillaOptionCalculationMethod {
        self.vanilla_option_calculation_method
    }

    pub fn get_div_structure_tenors(&self) -> &Vec<String> {
        &self.div_structure_tenors
    }

    pub fn get_rho_structure_tenors(&self) -> &Vec<String> {
        &self.rho_structure_tenors
    }

    pub fn get_vega_structure_tenors(&self) -> &Vec<String> {
        &self.vega_structure_tenors
    }

    pub fn get_vega_matrix_spot_moneyness(&self) -> &Array1<Real> {
        &self.vega_matrix_spot_moneyness
    }

    pub fn get_vega_matrix_calculation(&self) -> bool {
        self.vega_matrix
    }

    pub fn get_vega_structure_bump_value(&self) -> Real {
        self.vega_structure_bump_value
    }

    pub fn get_delta_bump_ratio(&self) -> Real {
        self.delta_bump_ratio
    }

    pub fn get_gamma_bump_ratio(&self) -> Real {
        self.gamma_bump_ratio
    }

    pub fn get_vega_bump_value(&self) -> Real {
        self.vega_bump_value
    }

    pub fn get_rho_bump_value(&self) -> Real {
        self.rho_bump_value
    }

    pub fn get_div_bump_value(&self) -> Real {
        self.div_bump_value
    }

    pub fn get_theta_day(&self) -> Integer {
        self.theta_day
    }

    pub fn get_stickyness_type(&self) -> StickynessType {
        self.stickyness_type
    }

    pub fn get_npv_calculation(&self) -> bool {
        self.npv
    }

    pub fn get_delta_calculation(&self) -> bool {
        self.delta
    }

    pub fn get_gamma_calculation(&self) -> bool {
        self.gamma
    }

    pub fn get_div_delta_calculation(&self) -> bool {
        self.div_delta
    }

    pub fn get_div_structure_calculation(&self) -> bool {
        self.div_structure
    }

    pub fn get_vega_calculation(&self) -> bool {
        self.vega
    }

    pub fn get_vega_structure_calculation(&self) -> bool {
        self.vega_strucure
    }

    pub fn get_theta_calculation(&self) -> bool {
        self.theta
    }

    pub fn get_rho_calculation(&self) -> bool {
        self.rho
    }

    pub fn get_rho_structure_calculation(&self) -> bool {
        self.rho_structure
    }

    pub fn get_fx_exposure_calculation(&self) -> bool {
        self.fx_exposure
    }

    pub fn get_lv_interpolator(&self) -> VolatilityInterplator {
        self.lv_interpolator.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_calculation_configuration_serde() {
        let config = CalculationConfiguration::default();
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: CalculationConfiguration = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        assert_eq!(config, deserialized);
    }
}
