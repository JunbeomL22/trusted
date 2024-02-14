use crate::definitions::{Real, Time};
use crate::time::calendar::{Calendar, NullCalendar};
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use std::fmt;

pub struct VectorData {
    value: Vec<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Vec<Time>,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
    name: String,
}

impl fmt::Debug for VectorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VectorData")
            .field("value", &self.value)
            .field("dates", &self.dates)
            .field("times", &self.times)
            .field("market_datetime", &self.market_datetime)
            .field("name", &self.name)
            .finish()
    }
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
        let times = (&dates).iter().map(|date| NullCalendar::default().get_time_difference(&market_datetime, date)).collect();
        VectorData {
            value,
            dates: Some(dates),
            times: times,
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

    /// This resets data.
    /// recieve dates and times as optional arguments.
    /// If times is not None, it will be saved as the input not calculated from dates vector
    /// If datetime is not None and times is None, the times will be calculated from the dates vector.
    /// Otherwise, the times and dates will be used as is.
    pub fn reset_data(&mut self, value: Vec<Real>, 
                dates: Option<Vec<OffsetDateTime>>,
                times: Option<Vec<Time>>,
                market_datetime: Option<OffsetDateTime>) {
        self.value = value;
        if let Some(market_datetime) = market_datetime {
            self.market_datetime = market_datetime;
        }

        if let Some(times) = times {
            self.times = times;
        } else if let Some(dates) = dates {
            self.times = (&dates).iter().map(|date| NullCalendar::default().get_time_difference(&self.market_datetime, &date)).collect();
        }

        assert!(self.value.len() == self.times.len(), "The length of value and times must be the same");
    }

    pub fn get_value(&self) -> Vec<Real> {
        self.value.clone()
    }

    pub fn get_times(&self) -> Vec<Time> {
        self.times.clone()
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
