use crate::definitions::Real;
use std::collections::HashMap;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::ops::{Add, Sub, Mul, Div};

/// NPV result
/// npv: Real
/// coupon_amounts: id -> (datetimes, amount)
/// coupon_paymeent_probability: id -> (datetime, probability)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NpvResult {
    npv: Real,
    cashflow_amounts: HashMap<usize, (OffsetDateTime, Real)>,
    cashflow_probabilities: HashMap<usize, (OffsetDateTime, Real)>,
}

impl NpvResult {
    pub fn new_from_npv(npv: Real) -> NpvResult {
        NpvResult {
            npv,
            cashflow_amounts: HashMap::new(),
            cashflow_probabilities: HashMap::new(),
        }
    }

    pub fn new(
        npv: Real,
        cashflow_amounts: HashMap<usize, (OffsetDateTime, Real)>,
        cashflow_probabilities: HashMap<usize, (OffsetDateTime, Real)>,
    ) -> NpvResult {
        NpvResult {
            npv,
            cashflow_amounts,
            cashflow_probabilities,
        }
    }

    pub fn get_npv(&self) -> Real {
        self.npv
    }

    pub fn get_expected_coupon_amount(&self) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        for (id, (datetime, amount)) in self.cashflow_amounts.iter() {
            let prob = self.cashflow_probabilities.get(id)
                .ok_or_else(||
                    anyhow::anyhow!("No probability found for coupon id {}", id)
            )?;
            res.insert(*datetime, *amount * prob.1);
        }
        Ok(res)
    }

    pub fn get_cashflow_amounts(&self) -> &HashMap<usize, (OffsetDateTime, Real)> {
        &self.cashflow_amounts
    }
}


impl Default for NpvResult {
    fn default() -> NpvResult {
        NpvResult {
            npv: 0.0,
            cashflow_amounts: HashMap::new(),
            cashflow_probabilities: HashMap::new(),
        }
    }
}

// Operations between two NpvResult instances
impl Add<&NpvResult> for NpvResult {
    type Output = Real;

    fn add(self, other: &NpvResult) -> Real {
        self.npv + other.get_npv()
    }
}

impl Sub<&NpvResult> for NpvResult {
    type Output = Real;

    fn sub(self, other: &Self) -> Real {
        self.npv - other.get_npv()
    }
}

impl Mul<&NpvResult> for NpvResult {
    type Output = Real;

    fn mul(self, other: &Self) -> Real {
        self.npv * other.get_npv()
    }
}

impl Div<&NpvResult> for NpvResult {
    type Output = Real;

    fn div(self, other: &Self) -> Real {
        self.npv / other.get_npv()
    }
}

// Operations between NpvResult and Real
impl Add<Real> for NpvResult {
    type Output = Real;

    fn add(self, rhs: Real) -> Real {
        self.npv + rhs
    }
}

impl Sub<Real> for NpvResult {
    type Output = Real;

    fn sub(self, rhs: Real) -> Real {
        self.npv - rhs
    }
}

impl Mul<Real> for NpvResult {
    type Output = Real;

    fn mul(self, rhs: Real) -> Real {
        self.npv * rhs
    }
}

impl Div<Real> for NpvResult {
    type Output = Real;

    fn div(self, rhs: Real) -> Real {
        self.npv / rhs
    }
}