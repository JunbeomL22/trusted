use crate::pricing_engines::calculation_result::CalculationResult;
use std::collections::HashMap;
use crate::definitions::Real;
/// Engine typically handles a bunch of instruments and calculate the pricing of the instruments.
/// Therefore, the result of calculations is a hashmap with the key being the code of the instrument
pub trait Engine {
    fn npv(&self) -> HashMap<String, Real>; // Code -> NPV
    fn set_npv(&mut self); 
    fn fx_exposure(&self) -> HashMap<String, Real>; // Code -> FX Exposure
    fn set_fx_exposure(&mut self);
    fn delta(&self) -> HashMap<String, HashMap<String, Real>>; // Code -> (Underlying Code -> Delta)
    fn set_delta(&mut self) {} 
    fn set_gamma(&mut self) {}
    fn set_vega(&mut self) {}
    fn set_vega_structure(&mut self) {}
    fn set_theta(&mut self) {}
    fn set_rho(&mut self) {}
    fn set_rho_structure(&mut self) {}
    fn get_calculation_result(&self) -> &HashMap<String, CalculationResult>;
    fn calculate(&mut self);
}