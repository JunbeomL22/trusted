use crate::definitions::Real;
use crate::utils::find_index::binary_search_index;
use crate::math::interpolator::InterpolatorReal1D;
//use crate::definitions::Real;

#[derive(Debug, Clone)]
pub struct LinearInterpolatorReal1D
{
    domain: Vec<Real>,
    value: Vec<Real>,
    derivatives: Vec<Real>,
    extrapolation: bool,
}


impl LinearInterpolatorReal1D
{
    pub fn new(domain: Vec<Real>, value: Vec<Real>, extrapolation: bool) -> LinearInterpolatorReal1D {
        let n = domain.len();
        assert_eq!(n, value.len());
        let mut derivatives = vec![0.0; n-1];
        for i in 0..n-1 {
            derivatives[i] = (value[i+1] - value[i]) / (domain[i+1] - domain[i]);
        }
        LinearInterpolatorReal1D {
            domain,
            value,
            derivatives,
            extrapolation,
        }
    }
}

impl InterpolatorReal1D for LinearInterpolatorReal1D 
{
    fn interpolate(&self, x: Real) -> Real
    {
        let n = self.domain.len();
        if x < self.domain[0] {
            if self.extrapolation {
                return self.value[0] + self.derivatives[0] * (x - self.domain[0]);
            } else {
                panic!("x is out of range");
            }
        }
        if x > self.domain[n-1] {
            if self.extrapolation {
                return self.value[n-1] + self.derivatives[n-2] * (x - self.domain[n-1]);
            } else {
                panic!("x is out of range");
            }
        }
        
        let index = binary_search_index(&self.domain, x);
        let res = self.value[index] + self.derivatives[index] * (x - self.domain[index]);
        return res;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolator_real_1d() {
        let domain = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let value = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let interpolator = LinearInterpolatorReal1D::new(domain, value, true);
        assert_eq!(interpolator.interpolate(1.5), 1.5);
        assert_eq!(interpolator.interpolate(2.5), 2.5);
        assert_eq!(interpolator.interpolate(3.5), 3.5);
        assert_eq!(interpolator.interpolate(4.5), 4.5);
        assert_eq!(interpolator.interpolate(0.5), 0.5);
        assert_eq!(interpolator.interpolate(5.5), 5.5);
    }
}