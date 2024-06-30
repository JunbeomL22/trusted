use crate::utils::string_arithmetic::{add_period, sub_period};
//use crate::data::observable::Observable;
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend,
    market_price::MarketPrice,
};
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Date};
use std::{
    ops::{AddAssign, SubAssign, Add, Sub},
    cmp::Ordering,
};


#[derive(Clone, Serialize, Deserialize)]
pub struct EvaluationDate {
    date: OffsetDateTime,
    #[serde(skip)]
    marketprice_observers: Vec<Rc<RefCell<MarketPrice>>>,
    #[serde(skip)]
    dividend_observers: Vec<Rc<RefCell<DiscreteRatioDividend>>>,
}

impl PartialEq<OffsetDateTime> for EvaluationDate {
    fn eq(&self, other: &OffsetDateTime) -> bool {
        self.date == *other
    }
}

impl PartialOrd<OffsetDateTime> for EvaluationDate {
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        self.date.partial_cmp(other)
    }
}

impl PartialEq<Date> for EvaluationDate {
    fn eq(&self, other: &Date) -> bool {
        self.date() == *other
    }
}

impl PartialOrd<Date> for EvaluationDate {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        self.date().partial_cmp(other)
    }
}

impl Debug for EvaluationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvaluationDate")
            .field("date", &self.date)
            .finish()
    }
}

impl Default for EvaluationDate {
    fn default() -> EvaluationDate {
        EvaluationDate {
            date: OffsetDateTime::now_utc(),
            marketprice_observers: vec![],
            dividend_observers: vec![],
        }
    }
}

impl EvaluationDate {
    pub fn new(date: OffsetDateTime) -> EvaluationDate {
        EvaluationDate {
            date,
            marketprice_observers: vec![],
            dividend_observers: vec![],
        }
    }

    pub fn date(&self) -> Date {
        self.date.date()
    }

    pub fn get_date_clone(&self) -> OffsetDateTime {
        self.date
    }

    pub fn get_date(&self) -> &OffsetDateTime {
        &self.date
    }

    pub fn set_date(&mut self, date: OffsetDateTime) {
        self.date = date;
        self.notify_observers();
    }

    pub fn add_dividend_observer(&mut self, observer: Rc<RefCell<DiscreteRatioDividend>>) {
        self.dividend_observers.push(observer);
    }
    
    pub fn add_marketprice_observer(&mut self, observer: Rc<RefCell<MarketPrice>>) {
        self.marketprice_observers.push(observer);
    }

    fn notify_observers(&mut self) {
        for marketprice_observer in self.marketprice_observers.iter() {
            {
                marketprice_observer.borrow_mut()
                .update_evaluation_date(self)
                .expect("Failed to update market price observer");
            }
        }

        for dividend_observer in self.dividend_observers.iter() {
            {
                dividend_observer.borrow_mut()
                .update_evaluation_date(self)
                .expect("Failed to update dividend observer");
            }
        }
    }

    pub fn display_observers(&self) {
        println!("Market Price Observers:");
        for observer in self.marketprice_observers.iter() {
            println!("{:?}", observer.borrow().get_name());
        }

        println!("Dividend Observers:");
        for observer in self.dividend_observers.iter() {
            println!("{:?}", observer.borrow().get_name());
        }
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