use definitions::{Real, Integer, Bool};
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

pub struct CalculationConfiguration {
    npv: Bool,
    delta: Bool,
    gamma: Bool,
    vega: Bool,
    vega_strucure: Bool,
    theta: Bool,
    rho: Bool,
    rho_structure: Bool,
    fx_exposure: Bool,
    theta_day: Integer,
    //
    stickyness_type: StickynessType,
}