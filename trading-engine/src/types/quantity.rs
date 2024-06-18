use crate::types::precision::Precision;
//
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quantity<T: Precision> {
    pub iovalue: u64,
    _precision: PhantomData<T>
}

impl<T: Precision> std::default::Default for Quantity<T> {
    fn default() -> Self {
        Quantity {
            iovalue: 0,
            _precision: PhantomData,
        }
    }
}

impl<T: Precision> Quantity<T> {
    pub fn new(val: f64) -> Result<Self> {
        Ok(
            Quantity {
                iovalue: T::quantity_f64_to_u64(val)?,
                _precision: PhantomData,
            }
        )
    }

    pub fn zero() -> Self {
        Quantity {
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
        Quantity {
            iovalue: iovalue,
            _precision: PhantomData,
        }
    }

    pub fn as_f64(&self) -> f64 {
        T::quantity_i64_to_f64(self.iovalue)
    }
}