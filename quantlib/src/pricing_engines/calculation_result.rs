use crate::instruments::instrument_info::InstrumentInfo;
use crate::definitions::Real;
use crate::evaluation_date::EvaluationDate;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    instrument: InstrumentInfo,
    evaluation_date: OffsetDateTime,
    npv: Option(Real), 
    delta: Option(HashMap<String, Real>),
    gamma: Option(HashMap<String, Real>),
    vega: Option(HashMap<String, Real>),
    vega_strucure: Option(HashMap<String, HashMap<Duration, Real>>),
    theta: Option(HashMap<String, Real>),
    rho: Option(HashMap<String, Real>),
    rho_structure: Option(HashMap<String, HashMap<Dutation, Real>>),
}