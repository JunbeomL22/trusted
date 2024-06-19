use crate::types::precision::Precision;
//
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookQuantity<T: Precision> {
    pub iovalue: u64,
    _precision: PhantomData<T>
}

impl<T: Precision> std::default::Default for BookQuantity<T> {
    fn default() -> Self {
        BookQuantity {
            iovalue: 0,
            _precision: PhantomData,
        }
    }
}

impl<T: Precision> BookQuantity<T> {
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
            iovalue: iovalue,
            _precision: PhantomData,
        }
    }

    pub fn as_f64(&self) -> f64 {
        T::quantity_u64_to_f64(self.iovalue)
    }
}