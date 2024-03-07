use crate::assets::currency::Currency;
use crate::definitions::{Real, Time};
use crate::time::calendar::{Calendar, NullCalendar};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::data::observable::Observable;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use crate::utils::myerror::{MyError, VectorDisplay};
use anyhow::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct VectorData {
    value: Array1<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Array1<Time>,
    market_datetime: OffsetDateTime,
    #[serde(skip)]
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
    currency: Currency,
    name: String,
}

impl fmt::Debug for VectorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VectorData")
            .field("value", &self.value)
            .field("dates", &self.dates)
            .field("times", &self.times)
            .field("market_datetime", &self.market_datetime)
            .field("currency", &self.currency)
            .field("name", &self.name)
            .field("observers", &self.observers.iter().map(|observer| {
                let observer = observer.borrow();
                format!("Address: {}, Name: {}, TypeName: {}", observer.get_address(), observer.get_name(), observer.get_type_name())
            }).collect::<Vec<_>>())
            .finish()
    }
}

impl Observable for VectorData {
    fn notify_observers(&mut self) {
        let observers = self.observers.clone();
        for observer in observers {
            observer.borrow_mut().update(self)
                .expect("VectorData::notify_observers failed to update observer")
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>) {
        self.observers.push(observer);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl VectorData {
    /// value: Array1<Real>,
    /// dates: Option<Vec<OffsetDateTime>>,
    /// times: Option<Array1<Time>>,
    /// market_datetime: OffsetDateTime,
    /// name: String
    pub fn new(
        value: Array1<Real>, 
        dates: Option<Vec<OffsetDateTime>>, 
        times: Option<Array1<Time>>,
        market_datetime: OffsetDateTime, 
        currency: Currency,
        name: String
    ) -> Result<VectorData, MyError> {
        // sanity check first
        if dates == None && times == None {
            return Err(MyError::NoneError {
                file: file!().to_string(),
                line: line!(),
                other_info: "dates and times are both None".to_string()
            });
        }

        if let Some(dates) = &dates {
            // change the following assertion to return Err
            if value.len() != dates.len() {
                return Err(MyError::MismatchedLengthError { 
                    file: file!().to_string(),
                    line: line!(),
                    left: VectorDisplay::REAL(value.to_vec()),
                    right: VectorDisplay::DATETIME(dates.clone()),
                    other_info: "The length of value and dates must be the same".to_string()
                });
            }
            
            let times: Array1<Time> = dates
            .iter()
            .map(|date| NullCalendar::default().get_time_difference(&market_datetime, date))
            .collect();
            
            let res = VectorData {
                value,
                dates: Some(dates.to_vec()),
                times: times,
                market_datetime: market_datetime,
                observers: Vec::new(),
                currency: currency,
                name: name,
            };
            Ok(res)
        } else {
            if let Some(times) = times {
                if value.len() != times.len() {
                    return Err(MyError::MismatchedLengthError { 
                        file: file!().to_string(),
                        line: line!(),
                        left: VectorDisplay::REAL(value.to_vec()),
                        right: VectorDisplay::TIME(times.to_vec()),
                        other_info: "The length of value and times must be the same".to_string()
                    });
                } else {
                    let res = VectorData {
                        value,
                        dates,
                        times: times,
                        market_datetime,
                        observers: Vec::new(),
                        currency: currency,
                        name,
                    };
                    Ok(res)
                }
            } else {
                return Err(MyError::NoneError {
                    file: file!().to_string(),
                    line: line!(),
                    other_info: "dates and times are both None".to_string()
                });
            }
        }
    }

    pub fn get_name_clone(&self) -> String {
        self.name.clone()
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
    ) -> Result<(), MyError> {
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

        if self.value.shape()[0] != self.times.shape()[0] {
            return Err(MyError::MismatchedLengthError { 
                file: file!().to_string(),
                line: line!(),
                left: VectorDisplay::REAL(self.value.to_vec()),
                right: VectorDisplay::TIME(self.times.to_vec()),
                other_info: "The length of value and times must be the same".to_string()
            });
        }

        self.notify_observers();
        Ok(())
    }

    /// add bimp_value to self.value wehere self.times in [t1, t2)
    fn bump_time_interval(
        &mut self, 
        from_t: Time, 
        before_t: Time, 
        bump_value: Real
    ) -> Result<(), MyError> {
        if from_t >= before_t {
            return Err(MyError::MisorderedTimeError {
                file: file!().to_string(),
                line: line!(),
                t1: from_t,
                t2: before_t,
                other_info: "VectorData::bump_time_interval".to_string()
            });
        }

        let mut i = 0;
        let time_length = self.times.shape()[0];
        let eps = 1e-8;

        while i < time_length {
            if self.times[i] >= from_t - eps && self.times[i] < before_t - eps {
                self.value[i] += bump_value;
            }
            i += 1;
        }
        self.notify_observers();
        Ok(())
    }

    fn bump_date_interval(&mut self, from_date: OffsetDateTime, before_date: OffsetDateTime, bump_value: Real) -> Result<(), MyError> {
        if from_date >= before_date {
            return Err(MyError::MisorderedOffsetDateTimeError {
                file: file!().to_string(),
                line: line!(),
                d1: from_date,
                d2: before_date,
                other_info: "VectorData::bump_date_interval".to_string()
            });
        }

        let mut i = 0;
        let time_length = self.dates.as_ref().unwrap().len();
        let eps = time::Duration::seconds(1);

        while i < time_length {
            if self.dates.as_ref().unwrap()[i] >= from_date - eps && self.dates.as_ref().unwrap()[i] <= before_date + eps {
                self.value[i] += bump_value;
            }
            i += 1;
        }
        self.notify_observers();
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use time::macros::datetime;
    use ndarray::array;

    #[test]
    fn test_vector_data_serialization() {
        let vector_data = VectorData::new(
            array![1.0, 2.0, 3.0, 4.0, 5.0], 
            None, 
            Some(array![0.0, 1.0, 2.0, 3.0, 4.0]), 
            datetime!(2020-01-01 00:00:00 UTC), 
            &Currency::KRW,
            "test_vector_data_serialization".to_string()
        );

        let serialized = serde_json::to_string(&vector_data).unwrap();
        println!("VectorData serialized = {}", serialized);
        let desrialized: VectorData = serde_json::from_str(&serialized).unwrap();
        println!("VectorData deserialized = {:?}", desrialized);
        
        // value check
        assert_eq!(vector_data.get_value_clone(), desrialized.get_value_clone());
        // times check
        assert_eq!(vector_data.get_times_clone(), desrialized.get_times_clone());
    }
} 
