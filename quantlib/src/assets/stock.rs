use crate::assets::currency::Currency;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::evaluation_date::EvaluationDate;
use crate::parameter::Parameter;
use time::OffsetDateTime;
use crate::definitions::Real;
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::myerror::MyError;

/// an observer of evaluation_date 
/// when ever calculating theta the Stock price mut be deducted by the dividend
#[derive(Debug, Clone)]
pub struct Stock {
    last_price: Real,
    market_datetime: OffsetDateTime,
    dividend: Option<Rc<RefCell<DiscreteRatioDividend>>>,
    currency: Currency,
    name: String,
    code: String,
}

impl Stock {
    /// new(
    /// last_price: Real, 
    /// market_datetime: OffsetDateTime,
    /// dividend: Option<DiscreteRatioDividend>,
    /// currency: Currency,
    /// name: String,
    /// code: String,
    /// )
    pub fn new(
        last_price: Real, 
        market_datetime: OffsetDateTime,
        dividend: Option<Rc<RefCell<DiscreteRatioDividend>>>,
        currency: Currency,
        name: String,
        code: String,
    ) -> Stock {
        Stock {
            last_price,
            market_datetime,
            dividend,
            currency,
            name,
            code,
        }
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_last_price(&self) -> Real {
        self.last_price
    }

    pub fn get_market_datetime(&self) -> &OffsetDateTime {
        &self.market_datetime
    }

    pub fn get_dividend(&self) -> &Option<Rc<RefCell<DiscreteRatioDividend>>> {
        &self.dividend
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    /// If the dividend is None, this returns 1.0
    pub fn get_dividend_deduction_ratio(&self, datetime: &OffsetDateTime) -> Real {
        if let Some(dividend) = &self.dividend {
            dividend.borrow().get_deduction_ratio(datetime)
        } else {
            1.0
        }
    }
}

/// implments arithmetic for Real
/// This operates only on the last_price
impl AddAssign<Real> for Stock {
    fn add_assign(&mut self, rhs: Real) {
        self.last_price += rhs;
    }
}

impl SubAssign<Real> for Stock {
    fn sub_assign(&mut self, rhs: Real) {
        self.last_price -= rhs;
    }
}

impl MulAssign<Real> for Stock {
    fn mul_assign(&mut self, rhs: Real) {
        self.last_price *= rhs;
    }
}

impl DivAssign<Real> for Stock {
    fn div_assign(&mut self, rhs: Real) {
        self.last_price /= rhs;
    }
}


impl Parameter for Stock {
    /// the stock price must be deducted by the dividend
    /// the amount is the sum of the dividend amount 
    /// between the market_datetime and the EvaluationDate
    fn update_evaluation_date(&mut self, data: &EvaluationDate) -> Result<(), MyError> {
        if let Some(dividend) = &self.dividend {
            let eval_dt = data.get_date_clone();
            if self.market_datetime < eval_dt {   
                for (date, div) in dividend.borrow().get_dividend().iter() {
                    if (*date > self.market_datetime) && (*date <= eval_dt) {
                        self.last_price -= div;
                    }
                }
                self.market_datetime = eval_dt;   
            } else {
                for (date, div) in dividend.borrow().get_dividend().iter() {
                    if (*date > eval_dt) && (*date <= self.market_datetime) {
                        self.last_price += div;
                    }
                }
            }
            
        }        
        Ok(())
    }

    fn get_type_name(&self) -> &'static str {
        "Stock"
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
    use time::OffsetDateTime;
    use time;
    use crate::evaluation_date::EvaluationDate;
    use crate::definitions::{CLOSING_TIME, SEOUL_OFFSET};
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::data::vector_data::VectorData;
    use ndarray::Array1;
    use crate::data::observable::Observable;

    #[test]
    fn test_stock_update_evaluation_date() {
        let (h, m, s) = SEOUL_OFFSET;
        let offset = time::UtcOffset::from_hms(h, m, s).unwrap();
        let eval_dt = OffsetDateTime::new_in_offset(
            time::macros::date!(2021-01-01),
            CLOSING_TIME,
            offset,
        );

        let evaluation_date = Rc::new(RefCell::new(
            EvaluationDate::new(
                eval_dt.clone()
            )
        ));

        let div_dates = vec![
            eval_dt + time::Duration::days(1),
            eval_dt + time::Duration::days(2),
            eval_dt + time::Duration::days(3),
        ];

        let spot = 100.0;
        let div_amounts = vec![1.0, 1.0, 1.0];
        let data = VectorData::new(
            Array1::from_vec(div_amounts.clone()),
            Some(div_dates.clone()),
            None,
            eval_dt.clone(),
            "dividend vecto data".to_string(),
        );

        let dividend = DiscreteRatioDividend::new(
            evaluation_date.clone(),
            &data,
            offset,
            spot,
            "MockStock".to_string(),
        );

        let stock = Rc::new(RefCell::new(
            Stock::new(
                spot,
                eval_dt.clone(),
                Some(dividend),
                Currency::KRW,
                "MockStock".to_string(),
                "MockCode".to_string(),
            )
        ));

        evaluation_date.borrow_mut().add_observer(stock.clone());

        for i in 1..div_dates.len() {
            *evaluation_date.borrow_mut() += "1D";
            let price = stock.borrow().get_last_price();
            assert!(
                (price - (spot - i as Real)).abs() < 1.0e-10,
                "stock: {}, (spot - i): {}",
                price,
                spot - i as Real
            );  
        }

        // get back the evaluation_date to the original
        *evaluation_date.borrow_mut() -= "3D";
        assert!(
            (stock.borrow().get_last_price() - spot).abs() < 1.0e-10,
            "stock: {}",
            stock.borrow().get_last_price()
        );     
    }
}


