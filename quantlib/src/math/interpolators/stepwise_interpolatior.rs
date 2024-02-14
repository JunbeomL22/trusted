use crate::definitions::Real;
use crate::utils::find_index::{binary_search_index, vectorized_search_index_for_sorted_input};
use crate::math::interpolator::Interpolator1D;
use num_traits::Num;

pub struct StepWiseInterpolator1D<T>
where T: Num + PartialOrd + Copy
{
    domain: Vec<T>,
    value: Vec<Real>,
    allow_extrapolation: bool,
}

impl<T> StepWiseInterpolator1D<T>
where T: Num + PartialOrd + Copy
{
    pub fn new(domain: Vec<T>, value: Vec<Real>, allow_extrapolation: bool) -> StepWiseInterpolator1D<T> {
        let n = domain.len();
        assert_eq!(n, value.len());
        // the domain must be sorted
        assert!(domain.windows(2).all(|w| w[0] <= w[1]));
        StepWiseInterpolator1D {
            domain,
            value,
            allow_extrapolation,
        }
    }
}

impl<T> Interpolator1D<T> for StepWiseInterpolator1D<T>
where T: Num + PartialOrd + Copy
{
    fn interpolate(&self, x: T) -> Real
    {
        let n = self.domain.len();
        if x < self.domain[0] {
            if self.allow_extrapolation {
                return self.value[0];
            } else {
                panic!("x is out of range");
            }
        }
        if x > self.domain[n-1] {
            if self.allow_extrapolation {
                return self.value[n-1];
            } else {
                panic!("x is out of range");
            }
        }
        let index = binary_search_index(&self.domain, x);
        self.value[index]
    }

    fn vectorized_interpolate_for_sorted_input(&self, x: &Vec<T>) -> Vec<Real>
    {
        let length = x.len();
        let mut result = vec![0.0; length];
        if length <= 2 {
            for i in 0..length {
                result[i] = self.interpolate(x[i]);
            }
        } else {
            let index = vectorized_search_index_for_sorted_input(&self.domain, x);
            let left_bound = self.domain[0];
            let right_bound = self.domain[self.domain.len()-1];
            for i in 0..length {
                if !self.allow_extrapolation && (x[i] < left_bound || x[i] > right_bound) {
                    panic!("Value out of domain range and extrapolation is not allowed");
                }
                result[i] = self.value[index[i]];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate() {
        let domain = vec![1,   3,   6,   8,   11];
        let value  = vec![1.0, 3.0, 6.0, 9.0, 11.0];
        let interpolator = StepWiseInterpolator1D::new(domain, value, true);
        assert_eq!(interpolator.interpolate(0), 1.0);
        assert_eq!(interpolator.interpolate(1), 1.0);
        assert_eq!(interpolator.interpolate(2), 1.0);
        assert_eq!(interpolator.interpolate(3), 3.0);
        assert_eq!(interpolator.interpolate(4), 3.0);
        assert_eq!(interpolator.interpolate(5), 3.0);
        assert_eq!(interpolator.interpolate(6), 6.0);
        assert_eq!(interpolator.interpolate(7), 6.0);
        assert_eq!(interpolator.interpolate(8), 9.0);
        assert_eq!(interpolator.interpolate(9), 9.0);
        assert_eq!(interpolator.interpolate(10), 9.0);
        assert_eq!(interpolator.interpolate(11), 11.0);
        assert_eq!(interpolator.interpolate(12), 11.0);
    }

    #[test]
    fn test_vectorized_interpolate_sorted_input() {
        let domain = vec![1,   3,   6,   8,   11];
        let value  = vec![1.0, 3.0, 6.0, 9.0, 11.0];

        let interpolator = StepWiseInterpolator1D::new(domain, value, true);
        let x = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let result = interpolator.vectorized_interpolate_for_sorted_input(&x);
        assert_eq!(result, vec![1.0, 1.0, 1.0, 3.0, 3.0, 3.0, 6.0, 6.0, 9.0, 9.0, 9.0, 11.0, 11.0]);
    }
}