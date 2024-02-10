use crate::definitions::Real;
pub trait InterpolatorReal1D
{
    fn interpolate(&self, domain: Real) -> Real;
}