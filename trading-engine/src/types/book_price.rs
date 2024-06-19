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
}