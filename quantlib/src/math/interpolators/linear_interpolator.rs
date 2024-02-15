use crate::definitions::Real;
use crate::utils::find_index::{binary_search_index, vectorized_search_index_for_sorted_input};
use crate::math::interpolator::{InterpolatorReal1D, ExtraPolationType};

#[derive(Debug, Clone)]
pub struct LinearInterpolator1D
{
    domain: Vec<Real>,
    value: Vec<Real>,
    derivatives: Vec<Real>,
    extrapolation_type: ExtraPolationType,
    allow_extrapolation: bool,
}

impl LinearInterpolator1D
{
    pub fn new(domain: Vec<Real>, 
            value: Vec<Real>, 
            extrapolation_type: ExtraPolationType,
            allow_extrapolation: bool) -> LinearInterpolator1D {
        let n = domain.len();
        if n != value.len() {
            let mut message = format!("domain and value must have the same length");
            message.push_str(&format!("\ndomain: {:?}", domain));
            message.push_str(&format!("\nvalue: {:?}", value));
            panic!("{}", message);
        }
        // the domain must be sorted
        assert!(domain.windows(2).all(|w| w[0] <= w[1]));
        let mut derivatives = vec![0.0; n-1];
        for i in 0..n-1 {
            derivatives[i] = (value[i+1] - value[i]) / ((domain[i+1] - domain[i]));
        }
        LinearInterpolator1D {
            domain,
            value,
            derivatives,
            extrapolation_type,
            allow_extrapolation,
        }
    }
}

impl InterpolatorReal1D for LinearInterpolator1D
{
    fn interpolate(&self, x: Real) -> Real
    {
        let n = self.domain.len();
        if x < self.domain[0] {
            if self.allow_extrapolation {
                if self.extrapolation_type == ExtraPolationType::Flat {
                    return self.value[0];
                }
                else if self.extrapolation_type == ExtraPolationType::Linear {
                    return self.value[0] + self.derivatives[0] * (x - self.domain[0]);    
                }
                else {
                    panic!("{}: extrapolation has not been implemented yet", self.extrapolation_type)
                }
            } else {
                panic!("x is out of range");
            }
        }
        if x > self.domain[n-1] {
            if self.allow_extrapolation {
                if self.extrapolation_type == ExtraPolationType::Flat {
                    return self.value[n-1];
                }
                else if self.extrapolation_type == ExtraPolationType::Linear {
                    return self.value[n-1] + self.derivatives[n-2] * (x - self.domain[n-1]);
                }
                else {
                    panic!("{}: extrapolation has not been implemented yet", self.extrapolation_type)
                }
            } 
            else {
                panic!("x is out of range");
            }
        }
        
        let index = binary_search_index(&self.domain, x);
        let res = self.value[index] + self.derivatives[index] * (x - self.domain[index]);
        return res;
    }

    /// Interpolate for a vector of x. This function does not check if x is sorted.
    fn vectorized_interpolate_for_sorted_input(&self, x: &Vec<Real>) -> Vec<Real> {
        let x_n = x.len();
        let domain_n = self.domain.len();
        let indices = vectorized_search_index_for_sorted_input(&self.domain, &x);
        let mut result = vec![0.0; x_n];

        for i in 0..x_n {
            if x[i] < self.domain[0] {
                if self.allow_extrapolation {
                    if self.extrapolation_type == ExtraPolationType::Flat {
                        result[i] = self.value[0];
                    }
                    else if self.extrapolation_type == ExtraPolationType::Linear {
                        result[i] = self.value[0] + self.derivatives[0] * (x[i] - self.domain[0]);
                    }
                    else {
                        panic!("{}: extrapolation has not been implemented yet", self.extrapolation_type)
                    }
                } else {
                    panic!("x is out of range");
                }
            }
            else if x[i] > self.domain[domain_n - 1] {
                if self.allow_extrapolation {
                    if self.extrapolation_type == ExtraPolationType::Flat {
                        result[i] = self.value[domain_n-1];
                    }
                    else if self.extrapolation_type == ExtraPolationType::Linear {
                        result[i] = self.value[domain_n-1] + self.derivatives[domain_n-2] * (x[i] - self.domain[domain_n-1]);
                    }
                    else {
                        panic!("{}: extrapolation has not been implemented yet", self.extrapolation_type)
                    }
                } 
                else {
                    panic!("x is out of range");
                }
            }
            else {
                result[i] = self.value[indices[i]] + self.derivatives[indices[i]] * (x[i] - self.domain[indices[i]]);
            }
        }
        result
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_linear_interpolator_real_1d_flat_extrapolation() {
        let domain = vec![1.0, 2.0, 3.0];
        let value = vec![1.0, 3.0, 2.0];
        let extrapolation_type = ExtraPolationType::Flat;
        let allow_extrapolation = true;
        let input = vec![0.5, 1.5, 2.5, 3.5];
        let expected = vec![1.0, 2.0, 2.5, 2.0];

        let interpolator = LinearInterpolator1D::new(domain, value, extrapolation_type, allow_extrapolation);
        let res = interpolator.vectorized_interpolate_for_sorted_input(&input);
        for i in 0..res.len() {
            assert_approx_eq!(res[i], expected[i]);
        }
    }

    #[test]
    fn test_linear_interpolator_real_1d_linear_extrapolation() {
        let domain = vec![1.0, 2.0, 3.0];
        let value = vec![1.0, 3.0, 2.0];
        let extrapolation_type = ExtraPolationType::Linear;
        let allow_extrapolation = true;
        let input = vec![0.5, 1.5, 2.5, 3.5];
        let expected = vec![0.0, 2.0, 2.5, 1.5];

        let interpolator = LinearInterpolator1D::new(domain, value, extrapolation_type, allow_extrapolation);
        let res = interpolator.vectorized_interpolate_for_sorted_input(&input);
        for i in 0..res.len() {
            assert_approx_eq!(res[i], expected[i]);
        }
    }
}