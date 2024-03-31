use crate::assets::currency::Currency;
use crate::definitions::{Real, Time};
use crate::time::{calendars::nullcalendar::NullCalendar, calendar_trait::CalendarTrait};
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
use anyhow::{anyhow, Result};

#[derive(Serialize, Deserialize, Clone)]
pub struct VectorData {
    value: Array1<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Array1<Time>,
    market_datetime: Option<OffsetDateTime>,
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
        market_datetime: Option<OffsetDateTime>, 
        currency: Currency,
        name: String
    ) -> Result<VectorData> {
        // sanity check first
        if dates == None && times == None {
            return Err(anyhow!(
                "dates and times are both None"
            ));
        }

        if let Some(dates) = &dates {
            // change the following assertion to return Err
            if value.len() != dates.len() {
                return Err(anyhow!(
                    "The length of value and dates must be the same\n\
                    value: {:?}, dates: {:?}",
                    value,
                    dates,
                ));
            }
            
            let market_datetime = match market_datetime {
                Some(market_datetime) => market_datetime,
                None => {
                    return Err(anyhow!(
                        "({}:{}) the dates in VectorData of {} is not None, but market_datetime is None\n\
                        Thus, it is vague to calculate the time difference between market_datetime and dates",
                        file!(), line!(), name
                    ));
                }
            };
            let time_calculator = NullCalendar::default();
            let times: Array1<Time> = dates
            .iter()
            .map(|date| time_calculator.get_time_difference(&market_datetime, date))
            .collect();
            
            let res = VectorData {
                value,
                dates: Some(dates.to_vec()),
                times: times,
                market_datetime: Some(market_datetime),
                observers: Vec::new(),
                currency: currency,
                name: name,
            };
            
            Ok(res)
        } else {
            if let Some(times) = times {
                if value.len() != times.len() {
                    return Err(anyhow!(
                        "The length of value and times must be the same\n\
                        value: {:?}, times: {:?}",
                        value,
                        times,
                    ));
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
                return Err(anyhow!("dates and times are both None"));
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
    ) -> Result<()> {
        self.value = value;
        
        self.market_datetime = market_datetime;
        

        if let Some(times) = times {
            self.times = times;
        } else if let Some(dates) = dates {
            let market_datetime = match market_datetime {
                Some(market_datetime) => market_datetime,
                None => {
                    return Err(anyhow!(
                        "({}:{}) the dates in VectorData of {} is not None, but market_datetime is None\n\
                        Thus, it is vague to calculate the time difference between market_datetime and dates",
                        file!(), line!(), self.name
                    ));
                }
            };

            let time_calculator = NullCalendar::default();
            self.times = (&dates)
            .iter()
            .map(|date| time_calculator.get_time_difference(&market_datetime, &date))
            .collect();
        }

        if self.value.shape()[0] != self.times.shape()[0] {
            return Err(anyhow!(
                "VectorData::reset_data value and times have different length\n\
                value: {:?}, times: {:?}",
                self.value,
                self.times
            ));
        }

        self.notify_observers();
        Ok(())
    }

    /// add bimp_value to self.value wehere self.times in [t1, t2)
    pub fn bump_time_interval(
        &mut self, 
        from_t: Time, 
        upto_t: Time, 
        bump_value: Real
    ) -> Result<()> {
        if from_t >= upto_t {
            return Err(anyhow!(
                "VectorData::bump_time_interval t1 = {}, t2 = {}", 
                from_t, upto_t
            ));
        }
            

        let mut i = 0;
        let time_length = self.times.shape()[0];
        
        while i < time_length {
            if self.times[i] >= from_t && self.times[i] <= upto_t {
                self.value[i] += bump_value;
            }
            i += 1;
        }
        self.notify_observers();
        Ok(())
    }

    pub fn bump_date_interval(
        &mut self, 
        from_date: &OffsetDateTime, 
        upto_date: &OffsetDateTime, 
        bump_value: Real
    ) -> Result<()> {
        if from_date >= upto_date {
            return Err(anyhow!(
                "VectorData::bump_date_interval from_date = {}, upto_date = {}", 
                from_date, upto_date,
            ));
        }

        let mut i = 0;
        let time_length = self.dates.as_ref().unwrap().len();

        while i < time_length {
            if self.dates.as_ref().unwrap()[i] >= *from_date && self.dates.as_ref().unwrap()[i] <= *upto_date {
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
            None,//datetime!(2020-01-01 00:00:00 UTC), 
            Currency::KRW,
            "test_vector_data_serialization".to_string()
        ).expect("failed to create VectorData");

        let serialized = serde_json::to_string(&vector_data).unwrap();
        println!("VectorData serialized = {}", serialized);
        let desrialized: VectorData = serde_json::from_str(&serialized).unwrap();
        println!("VectorData deserialized = {:?}", desrialized);
        
        // value check
        assert_eq!(vector_data.get_value_clone(), desrialized.get_value_clone());
        // times check
        assert_eq!(vector_data.get_times_clone(), desrialized.get_times_clone());
    }

    // test bump value time interval
    #[test]
    fn test_bump_value_time_interval() {
        let mut vector_data = VectorData::new(
            array![0.0, 1.0, 2.0, 3.0, 4.0],
            None, 
            Some(array![0.0, 1.0, 2.0, 3.0, 4.0]), 
            Some(datetime!(2020-01-01 00:00:00 UTC)), 
            Currency::KRW,
            "test_bump_value_time_interval".to_string()
        ).expect("failed to create VectorData");

        vector_data.bump_time_interval(-0.0, 0.5, 1.0).expect("failed to bump time interval");
        assert_eq!(vector_data.get_value_clone(), array![1.0, 1.0, 2.0, 3.0, 4.0]);
        vector_data.bump_time_interval(-0.0, 2.0, 1.0).expect("failed to bump time interval");
        assert_eq!(vector_data.get_value_clone(), array![2.0, 2.0, 3.0, 3.0, 4.0]);
        vector_data.bump_time_interval(-0.0, 4.0, 1.0).expect("failed to bump time interval");
        assert_eq!(vector_data.get_value_clone(), array![3.0, 3.0, 4.0, 4.0, 5.0]);
    }
} 
