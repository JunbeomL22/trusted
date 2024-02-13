use crate::definitions::Real;
use std::ops::{Add, Sub, Mul, Div};
use time::OffsetDateTime;
use crate::parameter::Parameter;
use crate::data::data_types::MarketDataType;


pub struct ValueData {
    value: Real,
    market_datetime: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
    data_type: MarketDataType,
    name: String,
}

impl ValueData {
    pub fn new(value: Real, market_datetime: OffsetDateTime, data_type: MarketDataType, name: String) -> ValueData {
        ValueData {
            value,
            market_datetime,
            observers: vec![],
            data_type,
            name,
        }
    }

    fn notify_observers(&mut self) {
        for observer in &mut self.observers {
            observer.update();
        }
    }

    fn reset_data(&mut self, value: Real, market_datetime: OffsetDateTime) {
        self.value = value;
        self.market_datetime = market_datetime;
        self.notify_observers();
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
    use crate::data::value_data::ValueData;
    use crate::parameter::Parameter;
    use crate::data::data_types::MarketDataType;
    use time::OffsetDateTime;
    use rstest::rstest;
    use crate::definitions::Real;

    struct MockParameter {
        value: Real,
    }

    impl Parameter for MockParameter {
        fn update(&mut self) {
            self.value += 1.0;
        }
    }

    #[test]
    fn test_add() {
        let mut value_data = ValueData::new(1.0, OffsetDateTime::now_utc(), MarketDataType::Spot, "test".to_string());
        let mock_parameter = MockParameter { value: 1.0 };
        value_data.observers.push(Box::new(&mock_parameter));
        value_data = value_data + 1.0;
        assert_eq!(value_data.value, 2.0);
        assert_eq!(mock_parameter.value, 2.0);
    }

}



