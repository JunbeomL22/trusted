use time::OffsetDateTime;
use std::ops::{AddAssign, SubAssign, Add, Sub};
use crate::utils::string_arithmetic::{add_period, sub_period};
use crate::data::observable::Observable;
use crate::parameter::Parameter;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EvaluationDate {
    date: OffsetDateTime,
    #[serde(skip)]
    observers: Vec<Rc<RefCell<dyn Parameter>>>,
}

impl Debug for EvaluationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvaluationDate")
            .field("date", &self.date)
            .finish()
    }
}

impl Observable for EvaluationDate {
    fn notify_observers(&mut self) {
        let observers = self.observers.clone();
        for observer in observers {
            observer.borrow_mut().update_evaluation_date(self).expect("Failed to update evaluation date");
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>) {
        self.observers.push(observer);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl EvaluationDate {
    pub fn new(date: OffsetDateTime) -> EvaluationDate {
        EvaluationDate {
            date,
            observers: vec![],
        }
    }

    pub fn get_date_clone(&self) -> OffsetDateTime {
        self.date.clone()
    }

    pub fn set_date(&mut self, date: OffsetDateTime) {
        self.date = date;
        self.notify_observers();
    }
}

impl AddAssign<&str> for EvaluationDate {
    fn add_assign(&mut self, rhs: &str) {
        self.date = add_period(&self.date, rhs);
        self.notify_observers();
    }
}

impl SubAssign<&str> for EvaluationDate {
    fn sub_assign(&mut self, rhs: &str) {
        self.date = sub_period(&self.date, rhs);
        self.notify_observers();
    }
}

impl Add<&str> for EvaluationDate {
    type Output = OffsetDateTime;

    fn add(self, rhs: &str) -> OffsetDateTime {
        add_period(&self.date, rhs)
    }
}

impl Sub<&str> for EvaluationDate {
    type Output = OffsetDateTime;

    fn sub(self, rhs: &str) -> OffsetDateTime {
        sub_period(&self.date, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter::Parameter;
    use time::macros::datetime;
    use std::rc::Rc;
    use std::cell::RefCell;
    use anyhow::Result;

    struct TestParameter {
        pub value: i32,
        name: String,
    }

    impl Parameter for TestParameter {
        fn update_evaluation_date(&mut self, _data: &EvaluationDate) -> Result<()> {
            self.value += 1;
            Ok(())
        }

        fn get_name(&self) -> &String {
            &self.name
        }

        fn get_type_name(&self) -> &'static str {
            "TestParameter"
        }
    }

    #[test]
    fn test_add_assign() {
        let date = datetime!(2020-01-01 00:00:00 UTC);
        let mut eval_date = EvaluationDate::new(date);
        
        let test_param = Rc::new(RefCell::new(TestParameter { value: 0, name: "TestParameter".to_string()}));
        eval_date.add_observer(test_param.clone());

        eval_date += "1D";
        assert_eq!(eval_date.get_date_clone(), datetime!(2020-01-02 00:00:00 UTC));
        assert_eq!(test_param.borrow().value, 1);
    }

    #[test]
    fn test_sub_assign() {
        let date = datetime!(2020-01-01 00:00:00 UTC);
        let mut eval_date = EvaluationDate::new(date);
        let test_param = Rc::new(RefCell::new(TestParameter { value: 0, name: "TestParameter".to_string()}));
        eval_date.add_observer(test_param.clone());
        eval_date -= "1D";
        assert_eq!(eval_date.get_date_clone(), datetime!(2019-12-31 00:00:00 UTC));
        assert_eq!(test_param.borrow().value, 1);
    }
}