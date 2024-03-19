use crate::definitions::{Real, Time};
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::time::{
    calendar_trait::CalendarTrait,
    calendars::nullcalendar::NullCalendar,
};
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use super::observable::Observable;

pub struct SurfaceData {
    value: Vec<Vec<Real>>,
    date_strike: Option<Vec<Vec<(OffsetDateTime, Real)>>>,
    time_strike: Vec<Vec<(Time, Real)>>,
    market_datetime: OffsetDateTime,
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
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

    pub fn get_value(&self) -> &Vec<Vec<Real>> {
        &self.value
    }

    pub fn get_date_strike(&self) -> Option<&Vec<Vec<(OffsetDateTime, Real)>> > {
        self.date_strike.as_ref()
    }

    pub fn get_time_strike(&self) -> &Vec<Vec<(Time, Real)>> {
        &self.time_strike
    }

    pub fn get_market_datetime(&self) -> &OffsetDateTime {
        &self.market_datetime
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

}

impl Observable for SurfaceData {
    fn notify_observers(&mut self) {
        let observers = self.observers.clone();
        for observer in observers {
            observer.borrow_mut().update(self)
                .expect("SurfaceData::notify_observers => failed to update observer")
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>) {
        self.observers.push(observer);
    }

    fn as_any(&self) -> &dyn Any {
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

