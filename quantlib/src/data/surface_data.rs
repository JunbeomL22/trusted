use crate::definitions::{Real, Time};
use std::ops::{Add, Sub, Mul, Div};
use time::{OffsetDateTime, Duration};
use crate::parameter::Parameter;

pub struct SurfaceData {
    value: Vec<Vec<Real>>,
    date_strike: Option<Vec<Vec<(OffsetDateTime, Real)>>>,
    time_strike: Vec<Vec<(Time, Real)>>,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
    name: String,
}

impl SurfaceData {
    pub fn new(value: Vec<Vec<Real>>, market_datetime: OffsetDateTime, name: String) -> SurfaceData {
        SurfaceData {
            value,
            date_strike: None,
            time_strike: vec![],
            market_datetime,
            observers: vec![],
            name: name,
        }
    }

    pub fn from_date_strike(value: Vec<Vec<Real>>, date_strike: Vec<Vec<(OffsetDateTime, Real)>>, market_datetime: OffsetDateTime, name: String) -> SurfaceData {
        SurfaceData {
            value,
            date_strike: Some(date_strike),
            time_strike: date_strike.iter().map(|row| row.iter().map(|(date, _)| NullCalendar::default().year_fraction(market_datetime, *date)).collect()).collect(),
            market_datetime,
            observers: vec![],
            name: name,
        }
    }

    pub fn from_time_strike(value: Vec<Vec<Real>>, time_strike: Vec<Vec<(Time, Real)>>, market_datetime: OffsetDateTime, name: String) -> SurfaceData {
        SurfaceData {
            value,
            date_strike: None,
            time_strike: time_strike,
            market_datetime: market_datetime,
            observers: vec![],
            name: name,
        }
    }

    fn notify_observers(&mut self) {
        for observer in &mut self.observers {
            observer.update();
        }
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

