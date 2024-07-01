use crate::types::precision::PrecisionTrait;
//
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookQuantity<T: PrecisionTrait> {
    pub iovalue: u64,
    _precision: PhantomData<T>
}

impl<T: PrecisionTrait> std::default::Default for BookQuantity<T> {
    fn default() -> Self {
        BookQuantity {
            iovalue: 0,
            _precision: PhantomData,
        }
    }
}

impl<T: PrecisionTrait> BookQuantity<T> {
    pub fn new(val: f64) -> Result<Self> {
        Ok(
            BookQuantity {
                iovalue: T::quantity_f64_to_u64(val)?,
                _precision: PhantomData,
            }
        )
    }

    pub fn zero() -> Self {
        BookQuantity {
            iovalue: 0,
            _precision: PhantomData,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.iovalue == 0
    }

    pub fn is_positive(&self) -> bool {
        self.iovalue > 0
    }

    pub fn from_iovalue(iovalue: u64) -> Self {
        BookQuantity {
            iovalue,
            _precision: PhantomData,
        }
    }

    pub fn as_f64(&self) -> f64 {
        T::quantity_u64_to_f64(self.iovalue)
    }

    pub fn precision(&self) -> u8 {
        T::precision()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::precision::{
        Prec0,
        Prec1,
        Prec2,
        Prec3,
        Prec4,
        Prec5,
        Prec6,
        Prec7,
        Prec8,
        Prec9,
    };

    #[test]
    fn test_book_quantity() {
        let q0 = BookQuantity::<Prec0>::new(0.0).unwrap();
        assert_eq!(q0.is_zero(), true);
        assert_eq!(q0.is_positive(), false);
        assert_eq!(q0.as_f64(), 0.0);

        let q1 = BookQuantity::<Prec1>::new(0.1).unwrap();
        assert_eq!(q1.is_zero(), false);
        assert_eq!(q1.is_positive(), true);
        assert_eq!(q1.as_f64(), 0.1);

        let q2 = BookQuantity::<Prec2>::new(0.01).unwrap();
        assert_eq!(q2.is_zero(), false);
        assert_eq!(q2.is_positive(), true);
        assert_eq!(q2.as_f64(), 0.01);

        let q3 = BookQuantity::<Prec3>::new(0.001).unwrap();
        assert_eq!(q3.is_zero(), false);
        assert_eq!(q3.is_positive(), true);
        assert_eq!(q3.as_f64(), 0.001);

        let q4 = BookQuantity::<Prec4>::new(0.0001).unwrap();
        assert_eq!(q4.is_zero(), false);
        assert_eq!(q4.is_positive(), true);
        assert_eq!(q4.as_f64(), 0.0001);

        let q5 = BookQuantity::<Prec5>::new(0.00001).unwrap();
        assert_eq!(q5.is_zero(), false);
        assert_eq!(q5.is_positive(), true);
        assert_eq!(q5.as_f64(), 0.00001);

        let q6 = BookQuantity::<Prec6>::new(0.000001).unwrap();
        assert_eq!(q6.is_zero(), false);
        assert_eq!(q6.is_positive(), true);
        assert_eq!(q6.as_f64(), 0.000001);

        let q7 = BookQuantity::<Prec7>::new(0.0000001).unwrap();
        assert_eq!(q7.is_zero(), false);
        assert_eq!(q7.is_positive(), true);

        let q8 = BookQuantity::<Prec8>::new(0.00000001).unwrap();
        assert_eq!(q8.is_zero(), false);
        assert_eq!(q8.is_positive(), true);

        let q9 = BookQuantity::<Prec9>::new(0.000000001).unwrap();
        assert_eq!(q9.is_zero(), false);
        assert_eq!(q9.is_positive(), true);

    }


}