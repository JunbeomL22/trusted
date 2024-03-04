use crate::definitions::Real;
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::data::observable::Observable;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

/// value: Real, market_datetime: OffsetDateTime, name: String
/// The examples are flat volatility, constant continuous dividend yield
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueData {
    value: Real,
    market_datetime: OffsetDateTime,
    #[serde(skip)]
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
    name: String,
}

impl Debug for ValueData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueData")
            .field("value", &self.value)
            .field("market_datetime", &self.market_datetime)
            .field("name", &self.name)
            .field("observers", &self.observers.iter().map(|observer| {
                let observer = observer.borrow();
                format!("Address: {:p}, Name: {}, TypeName: {}", observer, observer.get_name(), observer.get_typename())
            }).collect::<Vec<_>>())
            .finish()
    }
}

impl Observable for ValueData {
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

impl ValueData {
    pub fn new(value: Real, market_datetime: OffsetDateTime, name: String) -> ValueData {
        ValueData {
            value,
            market_datetime,
            observers: vec![],
            name,
        }
    }

    fn reset_data(&mut self, value: Real, market_datetime: OffsetDateTime) {
        self.value = value;
        self.market_datetime = market_datetime;
        self.notify_observers();
    }

    pub fn get_value(&self) -> Real {
        self.value
    }

    pub fn get_market_datetime(&self) -> &OffsetDateTime {
        &self.market_datetime
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl Add<Real> for ValueData {
    type Output = Self;
    fn add(mut self, rhs: Real) -> Self::Output {
        self.value += rhs;
        self.notify_observers();
        self
    }
}

impl Sub<Real> for ValueData {
    type Output = Self;

    fn sub(mut self, rhs: Real) -> Self::Output {
        self.value -= rhs;
        self.notify_observers();
        self
    }
}

impl Mul<Real> for ValueData {
    type Output = Self;

    fn mul(mut self, rhs: Real) -> Self::Output {
        self.value *= rhs;
        self.notify_observers();
        self
    }
}

impl Div<Real> for ValueData {
    type Output = Self;

    fn div(mut self, rhs: Real) -> Self::Output {
        self.value /= rhs;
        self.notify_observers();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::data::observable::Observable;
    use crate::data::value_data::ValueData;
    use crate::parameter::Parameter;
    use time::OffsetDateTime;
    use crate::definitions::Real;
    use std::rc::Rc;
    use std::cell::RefCell; 
    
    struct MockParameter {
        value: Real,
    }

    impl MockParameter {
        fn get_value(&self) -> Real {
            self.value
        }
    }
    impl Parameter for MockParameter {
        fn update(&mut self, data: &dyn Observable) {
            self.value += 1.0;
        }
    }

    #[test]
    fn test_add() {
        let mut value_data = ValueData::new(1.0, OffsetDateTime::now_utc(),"test".to_string());
        let mock_parameter = MockParameter { value: 1.0 };
        let mock_parameter_rc = Rc::new(RefCell::new(mock_parameter));

        value_data.add_observer(mock_parameter_rc.clone());
        value_data = value_data + 1.0;
        assert_eq!(value_data.value, 2.0);
        assert_eq!(mock_parameter_rc.borrow().get_value(), 2.0);
    }

    #[test]
    fn test_reset_data() {
        let mut value_data = ValueData::new(1.0, OffsetDateTime::now_utc(),"test".to_string());
        let mock_parameter = Rc::new(RefCell::new(MockParameter { value: 1.0 }));
        value_data.add_observer(mock_parameter.clone());
        value_data.reset_data(2.0, OffsetDateTime::now_utc());
        assert_eq!(value_data.value, 2.0);
        assert_eq!(mock_parameter.borrow().get_value(), 2.0);
    }
}



