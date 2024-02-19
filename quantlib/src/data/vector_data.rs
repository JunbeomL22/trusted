use crate::definitions::{Real, Time};
use crate::time::calendar::{Calendar, NullCalendar};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::data::observable::Observable;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use ndarray::Array1;

pub struct VectorData {
    value: Array1<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Array1<Time>,
    market_datetime: OffsetDateTime,
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
    name: String,
}

impl Observable for VectorData {
    fn notify_observers(&mut self) {
        let observers = self.observers.clone();
        for observer in observers {
            observer.borrow_mut().update(self);
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>) {
        self.observers.push(observer);
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    pub fn new(
        value: Array1<Real>, 
        dates: Option<Vec<OffsetDateTime>>, 
        times: Option<Array1<Time>>,
        market_datetime: OffsetDateTime, 
        name: String
    ) -> VectorData {
        // sanity check first
        if dates == None && times == None {
            panic!(
                "VectorData::new => Both dates and times cannot be None (occured at {})", 
                name
            );
        }

        if let Some(dates) = &dates {
            assert_eq!(
                value.len(), 
                dates.len(),
                "VectorData::new => The length of value and dates must be the same (occured at {}),\n value: {:?},\n dates: {:?}\n",
                &name, 
                &value, 
                &dates);
            

            let times: Array1<Time> = dates
            .iter()
            .map(|date| NullCalendar::default().get_time_difference(&market_datetime, date))
            .collect();
            
            VectorData {
                value,
                dates: Some(dates.to_vec()),
                times: times,
                market_datetime: market_datetime,
                observers: Vec::new(),
                name: name,
            }
        } else {
            let times = times.unwrap();
            assert_eq!(
                value.shape()[0],
                times.shape()[0],
                "VectorData::new => The length of value and times must be the same (occured at {}),\n value: {:?},\n times: {:?}",
                name, 
                value, 
                times,
            );

            VectorData {
                value,
                dates,
                times: times,
                market_datetime,
                observers: Vec::new(),
                name,
            }
        }
    }

    pub fn get_value_clone(&self) -> Array1<Real> {
        self.value.clone()
    }

    pub fn get_times_clone(&self) -> Array1<Time> {
        self.times.clone()
    }

    pub fn get_dates_clone(&self) -> Option<Vec<OffsetDateTime>> {
        self.dates.clone()
    }
    /// This resets data.
    /// recieve dates and times as optional arguments.
    /// If times is not None, it will be saved as the input not calculated from dates vector
    /// If datetime is not None and times is None, the times will be calculated from the dates vector.
    /// Otherwise, the times and dates will be used as is.
    pub fn reset_data(
        &mut self, 
        value: Array1<Real>, 
        dates: Option<Vec<OffsetDateTime>>,
        times: Option<Array1<Time>>,
        market_datetime: Option<OffsetDateTime>
    ) {
        self.value = value;
        if let Some(market_datetime) = market_datetime {
            self.market_datetime = market_datetime;
        }

        if let Some(times) = times {
            self.times = times;
        } else if let Some(dates) = dates {
            self.times = (&dates)
            .iter()
            .map(|date| NullCalendar::default().get_time_difference(&self.market_datetime, &date))
            .collect();
        }

        assert!(
            self.value.shape()[0] == self.times.shape()[0], 
            "The length of value and times must be the same"
        );

        self.notify_observers();
    }

    /// add bimp_value to self.value wehere self.times in (t1, t2]
    fn add_bump_value(&mut self, t1: Time, t2: Time, bump_value: Real) {
        assert!(
            t1 < t2, 
            "(occured at) add_bump_value(t1: {}, t2: {}, bump_value: {})",
            t1,
            t2,
            bump_value
        );

        let mut i = 0;
        let time_length = self.times.shape()[0];
        let eps = 1e-8;

        while i < time_length {
            if self.times[i] > t1 + eps && self.times[i] <= t2 + eps {
                self.value[i] += bump_value;
            }
            i += 1;
        }
        self.notify_observers();
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

impl AddAssign<Array1<Real>> for VectorData {
    fn add_assign(&mut self, rhs: Array1<Real>) {
        assert_eq!(
            self.value.shape()[0],
            rhs.shape()[0], 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, 
            &rhs
        );

        self.value += &rhs;

        self.notify_observers();
    }
}

impl SubAssign<Array1<Real>> for VectorData {
    fn sub_assign(&mut self, rhs: Array1<Real>) {
        assert_eq!(
            self.value.shape()[0],
            rhs.shape()[0], 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, 
            &rhs
        );

        self.value -= &rhs;

        self.notify_observers();
    }
}

impl MulAssign<Array1<Real>> for VectorData {
    fn mul_assign(&mut self, rhs: Array1<Real>) {
        assert_eq!(
            self.value.shape()[0],
            rhs.shape()[0], 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, 
            &rhs
        );

        self.value = &self.value * rhs;

        self.notify_observers();
    }
}

impl DivAssign<Array1<Real>> for VectorData {
    fn div_assign(&mut self, rhs: Array1<Real>) {
        assert_eq!(
            self.value.shape()[0],
            rhs.shape()[0], 
            "unmatched size => self: {:?}, rhs: {:?}", 
            &self.value, 
            &rhs
        );

        // rhs must not have zero
        assert!(
            rhs.iter().all(|&x| x != 0.0),
            "rhs must not have zero"
        );

        self.value = &self.value / rhs;

        self.notify_observers();
    }
}

