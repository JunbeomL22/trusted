use crate::definitions::Real;
pub trait Engine {
    fn npv(&self) -> Real; // exclude cash-flow on the date of the evaluation
}