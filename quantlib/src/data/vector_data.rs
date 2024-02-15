use crate::definitions::{Real, Time};
use crate::time::calendar::{Calendar, NullCalendar};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::data::observable::Observable;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct VectorData {
    value: Vec<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Vec<Time>,
    market_datetime: OffsetDateTime,
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
    name: String,
}

impl Observable for VectorData {
    fn notify_observers(&mut self) {
        for observer in &mut self.observers {
            observer.borrow_mut().update();
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>) {
        self.observers.push(observer);
    }
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
    pub fn new(value: Vec<Real>, dates: Option<Vec<OffsetDateTime>>, times: Option<Vec<Time>>, market_datetime: OffsetDateTime, name: String) -> VectorData {
        // sanity check first
        if dates == None && times == None {
            panic!("VectorData::new => Both dates and times cannot be None (occured at {})", name);
        }

        if let Some(dates) = &dates {
            assert_eq!(
                value.len(), 
                dates.len(),
                "VectorData::new => The length of value and dates must be the same (occured at {}),\n value: {:?},\n dates: {:?}\n",
                name, value, dates);
            

            let times: Vec<Time> = dates.iter().map(|date| NullCalendar::default().get_time_difference(&market_datetime, date)).collect();
            
            VectorData {
                value,
                dates: Some(dates.to_vec()),
                times: times,
                market_datetime: market_datetime,
                observers: Vec::new(),
                name: name,
            }
        } else {
            assert_eq!(
                value.len(), 
                times.as_ref().unwrap().len(),
                "VectorData::new => The length of value and times must be the same (occured at {}),\n value: {:?},\n times: {:?}",
                name, value, times.as_ref().unwrap()
            );

            VectorData {
                value,
                dates,
                times: times.unwrap(),
                market_datetime,
                observers: Vec::new(),
                name,
            }
        }
    }

    pub fn get_value(&self) -> Vec<Real> {
        self.value.clone()
    }

    pub fn get_times(&self) -> Vec<Time> {
        self.times.clone()
    }

    pub fn get_dates(&self) -> Option<Vec<OffsetDateTime>> {
        self.dates.clone()
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

}
impl AddAssign<Real> for VectorData {
    fn add_assign(&mut self, rhs: Real) {
        for value in &mut self.value {
            *value += rhs;
        }
        self.notify_observers();
    }
}

impl SubAssign<Real> for VectorData {
    fn sub_assign(&mut self, rhs: Real) {
        for value in &mut self.value {
            *value -= rhs;
        }
        self.notify_observers();
    }
}

impl MulAssign<Real> for VectorData {
    fn mul_assign(&mut self, rhs: Real) {
        for value in &mut self.value {
            *value *= rhs;
        }
        self.notify_observers();
    }
}

impl DivAssign<Real> for VectorData {
    fn div_assign(&mut self, rhs: Real) {
        for value in &mut self.value {
            *value /= rhs;
        }
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
        assert_eq!(
            self.value.len(), 
            rhs.len(), 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, &rhs
        );
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
        assert_eq!(
            self.value.len(), 
            rhs.len(), 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, &rhs
        );
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
        assert_eq!(
            self.value.len(), 
            rhs.len(), 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, &rhs
        );
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
        assert_eq!(
            self.value.len(), 
            rhs.len(), 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, &rhs
        );
        for (value, rhs_value) in self.value.iter_mut().zip(rhs) {
            *value /= rhs_value;
        }
        self.notify_observers();
        self
    }
}
