use crate::definitions::{Real, Time};
use crate::time::calendar::NullCalendar;
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;

pub struct VectorData {
    value: Vec<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Vec<Time>,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
    name: String,
}

impl VectorData {
    pub fn new(value: Vec<Real>, dates: Option<Vec<OffsetDateTime>>, times: Vec<Time>, market_datetime: OffsetDateTime, name: String) -> VectorData {
        VectorData {
            value,
            dates,
            times,
            market_datetime,
            observers: vec![],
            name: name
        }
    }

    pub fn from_offsetdatetime(value: Vec<Real>, dates: Vec<OffsetDateTime>, market_datetime: OffsetDateTime, name: String) -> VectorData {
        VectorData {
            value,
            dates: Some(dates),
            times: dates.iter().map(|date| NullCalendar::default().year_fraction(market_datetime, *date)).collect(),
            market_datetime,
            observers: vec![],
            name: name,
        }
    }

    pub fn from_time(value: Vec<Real>, times: Vec<Time>, market_datetime: OffsetDateTime, name: String) -> VectorData {
        VectorData {
            value,
            dates: None,
            times,
            market_datetime,
            observers: vec![],
            name: name,
        }
    }

    fn notify_observers(&mut self) {
        for observer in &mut self.observers {
            observer.update();
        }
    }

    pub fn reset_data(&mut self, value: Vec<Real>, datetime: Vec<OffsetDateTime>) {
        self.value = value;
        self.dates = Some(datetime);
        self.times = datetime.iter().map(|date| NullCalendar::default().year_fraction(self.market_datetime, *date)).collect();
        self.notify_observers();
    }

}

impl Add<Real> for VectorData {
    type Output = Self;

    fn add(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value += rhs;
        }
        self.notify_observers();
        self
    }
}

impl Sub<Real> for VectorData {
    type Output = Self;

    fn sub(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value -= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Mul<Real> for VectorData {
    type Output = Self;

    fn mul(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value *= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Div<Real> for VectorData {
    type Output = Self;

    fn div(mut self, rhs: Real) -> Self::Output {
        for value in &mut self.value {
            *value /= rhs;
        }
        self.notify_observers();
        self
    }
}

impl Add<Vec<Real>> for VectorData {
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

impl Sub<Vec<Real>> for VectorData {
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



impl Mul<Vec<Real>> for VectorData {
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

impl Div<Vec<Real>> for VectorData {
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
