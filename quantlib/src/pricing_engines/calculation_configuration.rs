use crate::definitions::{Real, Integer};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StickynessType {
    StickyToMoneyness,
    StickyToStrike,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalVolatilityInterplator {
    AndreasenHuge,
    Dupire,
}

/// CalculationConfiguration is a struct that holds the configuration of the calculation.
/// stickyness_type: StickynessType
/// StickynessType is an enum that represents the stickyness of the calculation.
/// If the stickyness_type is StickyToMoneyness, the delta will be calculated with respect to moneyness:
/// In other words, delta = dV/dS + dvol/dS * dV/dvols
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculationConfiguration {
    npv: bool,
    delta: bool,
    gamma: bool,
    vega: bool,
    vega_strucure: bool,
    theta: bool,
    rho: bool,
    rho_structure: bool,
    fx_exposure: bool,
    //
    stickyness_type: StickynessType,
    //
    delta_bump_ratio: Real,
    gamma_bump_ratio: Real,
    vega_bump_value: Real,
    rho_bump_value: Real,
    theta_day: Integer,
}

impl Default for CalculationConfiguration {
    fn default() -> CalculationConfiguration {
        CalculationConfiguration {
            npv: true,
            delta: false,
            gamma: false,
            vega: false,
            vega_strucure: false,
            theta: false,
            rho: false,
            rho_structure: false,
            fx_exposure: true,
            stickyness_type: StickynessType::StickyToMoneyness,
            delta_bump_ratio: 0.01,
            gamma_bump_ratio: 0.01,
            vega_bump_value: 0.01,
            rho_bump_value: 0.01,
            theta_day: 1,
        }
    }
}

impl CalculationConfiguration {
    pub fn new(
        npv: bool,
        delta: bool,
        gamma: bool,
        vega: bool,
        vega_strucure: bool,
        theta: bool,
        rho: bool,
        rho_structure: bool,
        fx_exposure: bool,
        theta_day: Integer,
        stickyness_type: StickynessType,
        delta_bump_ratio: Real,
        gamma_bump_ratio: Real,
        vega_bump_value: Real,
        rho_bump_value: Real,
    ) -> CalculationConfiguration {
        CalculationConfiguration {
            npv,
            delta,
            gamma,
            vega,
            vega_strucure,
            theta,
            rho,
            rho_structure,
            fx_exposure,
            stickyness_type,
            delta_bump_ratio,
            gamma_bump_ratio,
            vega_bump_value,
            rho_bump_value,
            theta_day,
        }
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

    pub fn with_vega_structure_calculation(mut self, vega_structure: bool) -> CalculationConfiguration {
        self.vega_strucure = vega_structure;
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

    pub fn with_rho_structure_calculation(mut self, rho_structure: bool) -> CalculationConfiguration {
        self.rho_structure = rho_structure;
        self
    }

    pub fn with_stickyness_type(mut self, stickyness_type: StickynessType) -> CalculationConfiguration {
        self.stickyness_type = stickyness_type;
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

    pub fn with_rho_bump_value(mut self, rho_bump_value: Real) -> CalculationConfiguration {
        self.rho_bump_value = rho_bump_value;
        self
    }

    pub fn with_theta_day(mut self, theta_day: Integer) -> CalculationConfiguration {
        self.theta_day = theta_day;
        self
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

    pub fn get_theta_day(&self) -> Integer {
        self.theta_day
    }

    pub fn get_stickyness_type(&self) -> StickynessType {
        self.stickyness_type.clone()
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