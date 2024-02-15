use crate::definitions::{Real, Time};
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::time::calendar::{NullCalendar, Calendar};

pub struct SurfaceData {
    value: Vec<Vec<Real>>,
    date_strike: Option<Vec<Vec<(OffsetDateTime, Real)>>>,
    time_strike: Vec<Vec<(Time, Real)>>,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
    name: String,
}

impl SurfaceData {
    pub fn new(value: Vec<Vec<Real>>, 
            date_strike: Option<Vec<Vec<(OffsetDateTime, Real)>>>, 
            time_strike: Option<Vec<Vec<(Time, Real)>>>, 
            market_datetime: OffsetDateTime, 
            name: String) -> SurfaceData {
        let res = match date_strike {
                Some(_date_strike) => {
                    if value.len() != _date_strike.len() {
                        let mut message = String::from("SurfaceData::new => unmatched size\n");
                        message.push_str(&format!("value: {:?}\n", value));
                        message.push_str(&format!("date_strike: {:?}", _date_strike));
                        panic!("{}", message);
                    } 
                    for (row, date_strike_row) in value.iter().zip(&_date_strike) {
                        if row.len() != date_strike_row.len() {
                            let mut message = String::from("SurfaceData::new => unmatched size\n");
                            message.push_str(&format!("value: {:?}\n", value));
                            message.push_str(&format!("date_strike: {:?}", _date_strike));
                            panic!("{}", message);
                        }
                    }
                    let calendar = NullCalendar::default();
                    let time_strike = _date_strike.iter().map(|row| row.iter().map(|(date, strike)| (calendar.get_time_difference(&market_datetime, date), *strike)).collect()).collect();
                    SurfaceData {
                        value,
                        date_strike: Some(_date_strike),
                        time_strike: time_strike,
                        market_datetime: market_datetime,
                        observers: vec![],
                        name: name,
                    }
                },
                None => {
                    match time_strike {
                        Some(time_strike) => {
                            if value.len() != time_strike.len() {
                                let mut message = String::from("SurfaceData::new => unmatched size\n");
                                message.push_str(&format!("value: {:?}\n", value));
                                message.push_str(&format!("time_strike: {:?}", time_strike));
                                panic!("{}", message);
                            }

                            for (row, time_strike_row) in value.iter().zip(&time_strike) {
                                if row.len() != time_strike_row.len() {
                                    let mut message = String::from("SurfaceData::new => unmatched size\n");
                                    message.push_str(&format!("value: {:?}\n", value));
                                    message.push_str(&format!("time_strike: {:?}", time_strike_row));
                                    panic!("{}", message);
                                }
                            }
                            SurfaceData {
                                value,
                                date_strike: None,
                                time_strike: time_strike,
                                market_datetime: market_datetime,
                                observers: vec![],
                                name: name,
                            }
                        },
                        None => {
                            panic!("Either date_strike or time_strike must be provided")
                        },
                    }
                },
        };
        res
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

