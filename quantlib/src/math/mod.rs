use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
use crate::math::interpolators::stepwise_interpolatior::StepWiseInterpolator1D;

pub mod interpolator;
pub mod interpolators {
    pub mod linear_interpolator;
    pub mod stepwise_interpolatior;
}