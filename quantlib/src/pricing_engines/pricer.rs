use std::collections::HashMap;
use crate::instrument::Instrument;
use crate::definitions::Real;

pub trait Pricer {
    fn npv(&self, instrument: &Vec<Instrument>) -> HashMap<String, Real>; // Code -> NPV
}