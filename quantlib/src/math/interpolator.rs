use crate::definitions::Real;
use std::fmt::Display;
use ndarray::Array1;
use num_traits::Num;

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
pub trait InterpolatorReal1D
{
    fn interpolate(&self, x: Real) -> Real; 
    /// Interpolate for a vector of x. This function does not check if x is sorted.
    fn vectorized_interpolate_for_sorted_ndarray(&self, x: &Array1<Real>) -> Array1<Real>;
}

/// I have chosen the domain to be Integer type to avoid floating point comparison error.
pub trait Interpolator1D<T>
where T: Num + Copy + PartialOrd
{
    fn interpolate(&self, x: T) -> Real;
    fn vectorized_interpolate_for_sorted_ndarray(&self, x: &Array1<T>) -> Array1<Real>;
}