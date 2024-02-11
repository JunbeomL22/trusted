use crate::definitions::Real;
use std::ops::{Add, Sub, Mul, Div};
pub trait Parameter {
    fn update(&mut self);
    fn value(&self) -> Real;
    fn clone(&self) -> Box<dyn Parameter>;
}
/// Data struct is an observable parameter to publish to parameter observers
/// such as rate curves, volatility surfaces, etc.
pub struct Data {
    value: Vec<Real>,
    observers: Vec<Box<dyn Parameter>>,
}

impl Data {
    pub fn new(value: Vec<Real>) -> Data {
        Data {
            value,
            observers: vec![],
        }
    }

    fn notify_observers(&mut self) {
        for observer in &mut self.observers {
            observer.update();
        }
    }
}

impl Add<Real> for Data {
    type Output = Self;

    fn add(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value += rhs;
        }
        self.notify_observers();
        self
    }
}

impl Sub<Real> for Data {
    type Output = Self;

    fn sub(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value -= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Mul<Real> for Data {
    type Output = Self;

    fn mul(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value *= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Div<Real> for Data {
    type Output = Self;

    fn div(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value /= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Add<Vec<Real>> for Data {
    type Output = Self;

    fn add(mut self, rhs: Vec<Real>) -> Self::Output {
        assert_eq!(self.value.len(), rhs.len(), "Vectors must be the same length");
        for (value, rhs_value) in self.value.iter_mut().zip(rhs) {
            *value += rhs_value;
        }
        self.notify_observers();
        self
    }
}

impl Sub<Vec<Real>> for Data {
    type Output = Self;

    fn sub(mut self, rhs: Vec<Real>) -> Self::Output {
        assert_eq!(self.value.len(), rhs.len(), "Vectors must be the same length");
        for (value, rhs_value) in self.value.iter_mut().zip(rhs) {
            *value -= rhs_value;
        }
        self.notify_observers();
        self
    }
}

impl Mul<Vec<Real>> for Data {
    type Output = Self;

    fn mul(mut self, rhs: Vec<Real>) -> Self::Output {
        assert_eq!(self.value.len(), rhs.len(), "Vectors must be the same length");
        for (value, rhs_value) in self.value.iter_mut().zip(rhs) {
            *value *= rhs_value;
        }
        self.notify_observers();
        self
    }
}

impl Div<Vec<Real>> for Data {
    type Output = Self;

    fn div(mut self, rhs: Vec<Real>) -> Self::Output {
        assert_eq!(self.value.len(), rhs.len(), "Vectors must be the same length");
        for (value, rhs_value) in self.value.iter_mut().zip(rhs) {
            *value /= rhs_value;
        }
        self.notify_observers();
        self
    }
}