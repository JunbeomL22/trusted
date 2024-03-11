use crate::definitions::Real;
use crate::utils::myerror::MyError;
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::data::observable::Observable;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use crate::assets::currency::Currency;

/// value: Real, market_datetime: OffsetDateTime, name: String
/// The examples are flat volatility, constant continuous dividend yield
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueData {
    value: Real,
    market_datetime: OffsetDateTime,
    #[serde(skip)]
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
    currency: Currency,
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
                format!("Address: {}, Name: {}, TypeName: {}", observer.get_address(), observer.get_name(), observer.get_type_name())
            }).collect::<Vec<_>>())
            .finish()
    }
}

impl Observable for ValueData {
    fn notify_observers(&mut self) {
        let observers = self.observers.clone();
        for observer in observers {
            observer.borrow_mut().update(self)
                .expect("ValueData::notify_observers => failed to update observer")
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
    pub fn new(
        value: Real, 
        market_datetime: OffsetDateTime, 
        currency: Currency,
        name: String
    ) -> Result<ValueData, MyError> {
        Ok(ValueData {
            value,
            market_datetime,
            observers: vec![],
            currency,
            name,
        })
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

    pub fn get_currency(&self) -> &Currency {
        &self.currency
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
    use crate::assets::currency::Currency;
    use crate::utils::myerror::MyError;
    struct MockParameter {
        value: Real,
        name: String,
    }

    impl MockParameter {
        fn get_value(&self) -> Real {
            self.value
        }
    }
    impl Parameter for MockParameter {
        fn update(&mut self, data: &dyn Observable) -> Result<(), MyError> {
            self.value += 1.0;
            Ok(())
        }

        fn get_type_name(&self) -> &'static str {
            "MockParameter"
        }

        fn get_name(&self) -> &String {
            &self.name
        }
    }

    #[test]
    fn test_add() {
        let mut value_data = ValueData::new(
            1.0, 
            OffsetDateTime::now_utc(),
            Currency::NIL,
            "test".to_string()).expect("Failed to create ValueData");
        let mock_parameter = MockParameter { value: 1.0, name: "test".to_string() };
        let mock_parameter_rc = Rc::new(RefCell::new(mock_parameter));

        value_data.add_observer(mock_parameter_rc.clone());
        value_data = value_data + 1.0;
        assert_eq!(value_data.get_value(), 2.0);
        assert_eq!(mock_parameter_rc.borrow().get_value(), 2.0);
    }

    #[test]
    fn test_reset_data() {
        let mut value_data = ValueData::new(
            1.0, 
            OffsetDateTime::now_utc(),
            Currency::NIL,
            "test".to_string()
        ).expect("Failed to create ValueData");
        
        let mock_parameter = Rc::new(RefCell::new(MockParameter { value: 1.0, name: "test".to_string()}));
        value_data.add_observer(mock_parameter.clone());
        value_data.reset_data(2.0, OffsetDateTime::now_utc());
        assert_eq!(value_data.get_value(), 2.0);
        assert_eq!(mock_parameter.borrow().get_value(), 2.0);
    }
}



