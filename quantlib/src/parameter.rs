use crate::definitions::{Real, Time};
use std::ops::{Add, Sub, Mul, Div};
use time::{OffsetDateTime, Duration};
pub trait Parameter {
    fn update(&mut self);
    fn value(&self) -> Real;
    fn clone(&self) -> Box<dyn Parameter>;
}
/// Data struct is an observable parameter to publish to parameter observers
/// such as rate curves, volatility surfaces, etc.
pub struct Data {
    value: Real,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
}

pub struct CurveData {
    value: Vec<Real>,
    tenor: Vec<OffsetDateTime>,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
}

pub struct SurfaceData {
    value: Vec<Vec<Real>>,
    observers: Vec<Box<dyn Parameter>>,
}

impl CurveData {
    pub fn new(value: Vec<Real>) -> CurveData {
        CurveData {
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

impl Add<Real> for CurveData {
    type Output = Self;

    fn add(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value += rhs;
        }
        self.notify_observers();
        self
    }
}

impl Sub<Real> for CurveData {
    type Output = Self;

    fn sub(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value -= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Mul<Real> for CurveData {
    type Output = Self;

    fn mul(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value *= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Div<Real> for CurveData {
    type Output = Self;

    fn div(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value /= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Add<Vec<Real>> for CurveData {
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

impl Sub<Vec<Real>> for CurveData {
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

impl Mul<Vec<Real>> for CurveData {
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

impl Div<Vec<Real>> for CurveData {
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

impl Data {
    pub fn new(value: Real) -> Data {
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


impl SurfaceData {
    pub fn new(value: Vec<Vec<Real>>) -> SurfaceData {
        SurfaceData {
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
        self.value += rhs;
        self.notify_observers();
        self
    }
}

impl Sub<Real> for Data {
    type Output = Self;

    fn sub(mut self, rhs: Real) -> Self::Output {
        self.value -= rhs;
        self.notify_observers();
        self
    }
}

impl Mul<Real> for Data {
    type Output = Self;

    fn mul(mut self, rhs: Real) -> Self::Output {
        self.value *= rhs;
        self.notify_observers();
        self
    }
}

impl Div<Real> for Data {
    type Output = Self;

    fn div(mut self, rhs: Real) -> Self::Output {
        self.value /= rhs;
        self.notify_observers();
        self
    }
}

impl Add<Real> for SurfaceData {
    type Output = Self;

    fn add(mut self, rhs: Real) -> Self::Output {
        for row in &mut self.value {
            for value in row {
                *value += rhs;
            }
        }
        self.notify_observers();
        self
    }
}

impl Add<Vec<Vec<Real>>> for SurfaceData {
    type Output = Self;

    fn add(mut self, rhs: Vec<Vec<Real>>) -> Self::Output {
        assert_eq!(self.value.len(), rhs.len(), "Vectors must be the same length");
        for (row, rhs_row) in self.value.iter_mut().zip(rhs) {
            assert_eq!(row.len(), rhs_row.len(), "Inner vectors must be the same length");
            for (value, rhs_value) in row.iter_mut().zip(rhs_row) {
                *value += rhs_value;
            }
        }
        self.notify_observers();
        self
    }
}

impl Sub<Real> for SurfaceData {
    type Output = Self;

    fn sub(mut self, rhs: Real) -> Self::Output {
        for row in &mut self.value {
            for value in row {
                *value -= rhs;
            }
        }
        self.notify_observers();
        self
    }
}

impl Sub<Vec<Vec<Real>>> for SurfaceData {
    type Output = Self;

    fn sub(mut self, rhs: Vec<Vec<Real>>) -> Self::Output {
        assert_eq!(self.value.len(), rhs.len(), "Vectors must be the same length");
        for (row, rhs_row) in self.value.iter_mut().zip(rhs) {
            assert_eq!(row.len(), rhs_row.len(), "Inner vectors must be the same length");
            for (value, rhs_value) in row.iter_mut().zip(rhs_row) {
                *value -= rhs_value;
            }
        }
        self.notify_observers();
        self
    }
}

impl Mul<Real> for SurfaceData {
    type Output = Self;

    fn mul(mut self, rhs: Real) -> Self::Output {
        for row in &mut self.value {
            for value in row {
                *value *= rhs;
            }
        }
        self.notify_observers();
        self
    }
}

impl Div<Real> for SurfaceData {
    type Output = Self;

    fn div(mut self, rhs: Real) -> Self::Output {
        for row in &mut self.value {
            for value in row {
                *value /= rhs;
            }
        }
        self.notify_observers();
        self
    }
}

pub struct EvaluationDate {
    date: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
}

impl EvaluationDate {
    pub fn new(date: OffsetDateTime) -> EvaluationDate {
        EvaluationDate {
            date,
            observers: vec![],
        }
    }

    pub fn set_date(&mut self, new_date: OffsetDateTime) {
        self.date = new_date;
        self.notify_observers();
    }

    pub fn add_duration(&mut self, duration: Duration) {
        self.date += duration;
        self.notify_observers();
    }

    pub fn sub_duration(&mut self, duration: Duration) {
        self.date -= duration;
        self.notify_observers();
    }

    fn notify_observers(&mut self) {
        for observer in &mut self.observers {
            observer.update();
        }
    }
}

