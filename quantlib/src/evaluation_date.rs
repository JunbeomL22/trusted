use time::OffsetDateTime;
use std::ops::{AddAssign, SubAssign};
use crate::utils::string_arithmetic::{add_period, sub_period};
use crate::data::observable::Observable;
use crate::parameter::Parameter;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

pub struct EvaluationDate {
    date: OffsetDateTime,
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
        for observer in &mut self.observers {
            observer.borrow_mut().update();
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>) {
        self.observers.push(observer);
    }
}

impl EvaluationDate {
    pub fn new(date: OffsetDateTime) -> EvaluationDate {
        EvaluationDate {
            date,
            observers: vec![],
        }
    }

    pub fn get_date(&self) -> OffsetDateTime {
        self.date.clone()
    }
}

impl AddAssign<&str> for EvaluationDate {
    fn add_assign(&mut self, rhs: &str) {
        self.date = add_period(self.date, rhs);
        self.notify_observers();
    }
}

impl SubAssign<&str> for EvaluationDate {
    fn sub_assign(&mut self, rhs: &str) {
        self.date = sub_period(self.date, rhs);
        self.notify_observers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter::Parameter;
    use time::macros::datetime;
    use std::rc::Rc;
    use std::cell::RefCell;

    struct TestParameter {
        pub value: i32,
    }

    impl Parameter for TestParameter {
        fn update(&mut self) {
            self.value += 1;
        }
    }

    #[test]
    fn test_add_assign() {
        let date = datetime!(2020-01-01 00:00:00 UTC);
        let mut eval_date = EvaluationDate::new(date);
        let test_param = Rc::new(RefCell::new(TestParameter { value: 0 }));
        eval_date.add_observer(test_param.clone());
        eval_date += "1D";
        assert_eq!(eval_date.get_date(), datetime!(2020-01-02 00:00:00 UTC));
        assert_eq!(test_param.borrow().value, 1);
    }

    #[test]
    fn test_sub_assign() {
        let date = datetime!(2020-01-01 00:00:00 UTC);
        let mut eval_date = EvaluationDate::new(date);
        let test_param = Rc::new(RefCell::new(TestParameter { value: 0 }));
        eval_date.add_observer(test_param.clone());
        eval_date -= "1D";
        assert_eq!(eval_date.get_date(), datetime!(2019-12-31 00:00:00 UTC));
        assert_eq!(test_param.borrow().value, 1);
    }
}