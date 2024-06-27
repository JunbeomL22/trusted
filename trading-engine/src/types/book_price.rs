use crate::types::precision::Precision;
//
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookPrice<T: Precision + Clone> {
    pub iovalue: i64,
    _precision: PhantomData<T>
}

impl<T: Precision + Clone> std::default::Default for BookPrice<T> {
    fn default() -> Self {
        BookPrice {
            iovalue: 0,
            _precision: PhantomData,
        }
    }
}

impl<T: Precision + Clone> BookPrice<T> {
    pub fn new(val: f64) -> Result<Self> {
        Ok(
            BookPrice {
                iovalue: T::price_f64_to_i64(val)?,
                _precision: PhantomData,
            }
        )
    }

    pub fn zero() -> Self {
        BookPrice {
            iovalue: 0,
            _precision: PhantomData,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.iovalue == 0
    }

    pub fn from_iovalue(iovalue: i64) -> Self {
        BookPrice {
            iovalue: iovalue,
            _precision: PhantomData,
        }
    }

    pub fn as_f64(&self) -> f64 {
        T::price_i64_to_f64(self.iovalue)
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
    };

    #[test]
    fn test_book_price() {
        let bp = BookPrice::<Prec2>::new(1.23).unwrap();
        assert_eq!(bp.as_f64(), 1.23);
        assert_eq!(bp.iovalue, 1_230_000_000);
        assert_eq!(bp.is_zero(), false);

        let bp = BookPrice::<Prec2>::zero();
        assert_eq!(bp.as_f64(), 0.0);
        assert_eq!(bp.iovalue, 0);
        assert_eq!(bp.is_zero(), true);

        let bp = BookPrice::<Prec2>::from_iovalue(1_230_000_000);
        assert_eq!(bp.as_f64(), 1.23);
        assert_eq!(bp.iovalue, 1_230_000_000);

        let bp = BookPrice::<Prec0>::new(1.23).unwrap();
        assert_eq!(bp.as_f64(), 1.0);
        assert_eq!(bp.iovalue, 1_000_000_000);
        assert_eq!(bp.is_zero(), false);

        let bp = BookPrice::<Prec0>::zero();
        assert_eq!(bp.as_f64(), 0.0);
        assert_eq!(bp.iovalue, 0);
        assert_eq!(bp.is_zero(), true);

        let bp = BookPrice::<Prec1>::from_iovalue(100_000_000);
        assert_eq!(bp.as_f64(), 0.1);
        assert_eq!(bp.iovalue, 100_000_000);
    }

}