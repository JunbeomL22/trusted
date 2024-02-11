use crate::definitions::{Real, Integer};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ExtraPolationType
{
    None = 0,
    Flat = 1,
    Linear = 2,
}

impl Display for ExtraPolationType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ExtraPolationType::None => write!(f, "None"),
            ExtraPolationType::Flat => write!(f, "Flat"),
            ExtraPolationType::Linear => write!(f, "Linear"),
        }
    }
}
pub trait RealInterpolator1D
{
    fn interpolate(&self, x: Real) -> Real;
    /// Interpolate for a vector of x. This function does not check if x is sorted.
    fn vectorized_interpolate_sorted_input(&self, x: &Vec<Real>) -> Vec<Real>;
}

/// I have chosen the domain to be Integer type to avoid floating point comparison error.
pub trait IntegerInterpolator1D
{
    fn interpolate(&self, x: Integer) -> Real;
    fn vectorized_interpolate_sorted_input(&self, x: &Vec<Integer>) -> Vec<Real>;
}