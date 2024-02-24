use crate::definitions::Real;
use crate::utils::find_index_ndarray::{binary_search_index_ndarray, vectorized_search_index_for_sorted_ndarray};
use crate::math::interpolator::Interpolator1D;
use ndarray::Array1;
use crate::util::is_ndarray_sorted;
use num_traits::Num;

#[derive(Clone, Debug)]
pub struct StepwiseInterpolator1D<T>
where T: Num + PartialOrd + Copy
{
    domain: Array1<T>,
    value: Array1<Real>,
    allow_extrapolation: bool,
    left_extrapolation_value: Option<Real>,
    right_extrapolation_value: Option<Real>,
}

impl<T> StepwiseInterpolator1D<T>
where T: Num + PartialOrd + Copy
{
    pub fn new(
        domain: Array1<T>, 
        value: Array1<Real>, 
        allow_extrapolation: bool,
        left_extrapolation_value: Option<Real>,
        right_extrapolation_value: Option<Real>,
    ) -> StepwiseInterpolator1D<T> {
        let n = domain.len();
        assert_eq!(n, value.len());

        assert!(
            n!=0, 
            "(StepwiseInterpolator1D::new) domain and value must not be empty"
        );

        // the domain must be sorted
        assert!(
            is_ndarray_sorted(&domain),
            "(StepwiseInterpolator1D::new) domain must be sorted:"
        );

        StepwiseInterpolator1D {
            domain,
            value,
            allow_extrapolation,
            left_extrapolation_value,
            right_extrapolation_value,
        }
    }
}

impl<T> Interpolator1D<T> for StepwiseInterpolator1D<T>
where T: Num + PartialOrd + Copy
{
    fn interpolate(&self, x: T) -> Real
    {
        let n = self.domain.len();
        if x < self.domain[0] {
            if self.allow_extrapolation {
                if self.left_extrapolation_value.is_none() {
                    return self.value[0] 
                } else {
                    return self.left_extrapolation_value.unwrap();
                }
                
            } else {
                panic!("(occured at StepwiseInterpolator1D::interpolate) x is out of range");
            }
        }
        if x > self.domain[n-1] {
            if self.allow_extrapolation {
                if self.right_extrapolation_value.is_none() {
                    return self.value[n-1];
                } else {
                    return self.right_extrapolation_value.unwrap();
                }
            } else {
                panic!("(occured at StepwiseInterpolator1D::interpolate) x is out of range");
            }
        }
        let index = binary_search_index_ndarray(&self.domain, x);
        self.value[index]
    }

    fn vectorized_interpolate_for_sorted_ndarray(&self, x: &Array1<T>) -> Array1<Real>
    {
        let length = x.len();
        let mut result = Array1::zeros(length);
        if length <= 2 {
            for i in 0..length {
                result[i] = self.interpolate(x[i]);
            }
        } else {
            let index = vectorized_search_index_for_sorted_ndarray(&self.domain, x);
            let left_bound = self.domain[0];
            let right_bound = self.domain[self.domain.len()-1];
            for i in 0..length {
                if !self.allow_extrapolation && (x[i] < left_bound || x[i] > right_bound) {
                    panic!("(occured at StepwiseInterpolator1D::vectorized_interpolate_for_sorted_ndarray)\nValue out of domain range and extrapolation is not allowed");
                }
                result[i] = self.value[index[i]];
            }
        }
        result
    }
}

/// ConstantInterpolator1D is a type of StepwiseInterpolator1D that gives only one value for any input.
#[derive(Debug, Clone)]
pub struct ConstantInterpolator1D
{
    value: Real,
}

impl ConstantInterpolator1D
{
    pub fn new(value: Real) -> ConstantInterpolator1D {
        ConstantInterpolator1D {
            value,
        }
    }
}

impl<T> Interpolator1D<T> for ConstantInterpolator1D
where T: Num + PartialOrd + Copy
{
    fn interpolate(&self, _x: T) -> Real
    {
        self.value
    }

    fn vectorized_interpolate_for_sorted_ndarray(&self, x: &Array1<T>) -> Array1<Real>
    {
        let result = Array1::from_elem(x.len(), self.value);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_interpolate() {
        let domain = array![1,   3,   6,   8,   11];
        let value  = array![1.0, 3.0, 6.0, 9.0, 11.0];
        let interpolator = StepwiseInterpolator1D::new(
            domain, 
            value, 
            true,
            Some(1.0),
            Some(11.0),
        );
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
        let domain = array![1,   3,   6,   8,   11];
        let value  = array![1.0, 3.0, 6.0, 9.0, 11.0];

        let interpolator = StepwiseInterpolator1D::new(
                                domain, 
                                value, 
                                true,
                                None,
                                None,
                            );
        let x = array![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let result = interpolator.vectorized_interpolate_for_sorted_ndarray(&x);
        assert_eq!(
            result, 
            array![1.0, 1.0, 1.0, 3.0, 3.0, 3.0, 6.0, 6.0, 9.0, 9.0, 9.0, 11.0, 11.0]
        );
    }
}