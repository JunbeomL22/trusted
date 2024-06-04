use crate::definitions::Real;
use time::OffsetDateTime;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use crate::currency::Currency;
use anyhow::Result;

/// value: Real, market_datetime: OffsetDateTime, name: String
/// The examples are flat volatility, constant continuous dividend yield
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueData {
    value: Real,
    market_datetime: Option<OffsetDateTime>,
    currency: Currency,
    name: String,
    code: String,
}

impl Debug for ValueData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueData")
            .field("value", &self.value)
            .field("market_datetime", &self.market_datetime)
            .field("name", &self.name)
            .field("code", &self.code)
            .finish()
    }
}

impl ValueData {
    pub fn new(
        value: Real, 
        market_datetime: Option<OffsetDateTime>, 
        currency: Currency,
        name: String,
        code: String,
    ) -> Result<ValueData> {
        Ok(ValueData {
            value,
            market_datetime,
            currency,
            name,
            code,
        })
    }

    /*
    fn reset_data(&mut self, value: Real, market_datetime: Option<OffsetDateTime>) {
        self.value = value;
        self.market_datetime = market_datetime;
        self.notify_observers();
    }
     */

    pub fn get_value(&self) -> Real {
        self.value
    }

    pub fn get_market_datetime(&self) -> &Option<OffsetDateTime> {
        &self.market_datetime
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
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
    use crate::currency::Currency;
    use anyhow::Result;

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
        fn update(&mut self, _data: &dyn Observable) -> Result<()> {
            self.value += 1.0;
            Ok(())
        }
    }

    #[test]
    fn test_creation() -> Result<()> {
        let mut value_data = ValueData::new(
            1.0, 
            None,//OffsetDateTime::now_utc(),
            Currency::NIL,
            "test".to_string(),
            "test".to_string(),
        ).expect("Failed to create ValueData");

        Ok(())
    }
}



