use crate::types::precision::Precision;
//
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Price<T: Precision> {
    pub iovalue: i64,
    _precision: PhantomData<T>
}

impl<T: Precision> std::default::Default for Price<T> {
    fn default() -> Self {
        Price {
            iovalue: 0,
            _precision: PhantomData,
        }
    }
}

impl<T: Precision> Price<T> {
    pub fn new(val: f64) -> Result<Self> {
        Ok(
            Price {
                iovalue: T::price_f64_to_i64(val)?,
                _precision: PhantomData,
            }
        )
    }

    pub fn zero() -> Self {
        Price {
            iovalue: 0,
            _precision: PhantomData,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.iovalue == 0
    }

    pub fn from_iovalue(iovalue: i64) -> Self {
        Price {
            iovalue: iovalue,
            _precision: PhantomData,
        }
    }

    pub fn as_f64(&self) -> f64 {
        T::price_i64_to_f64(self.iovalue)
    }
}