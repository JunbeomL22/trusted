use std::collections::HashMap;
use crate::instrument::Instrument;
use crate::definitions::Real;

pub trait Pricer {
    // Code -> NPV
    fn npv(&self, instrument: &Vec<Instrument>) -> HashMap<String, Real>; 
    // Code -> (Risk Factor -> Delta)
    fn delta(&self, instrument: &Vec<Instrument>) -> HashMap<String, HashMap<String, Real>>; 
}