use crate::currency::Currency;
use crate::definitions::Real;
use crate::evaluation_date::EvaluationDate;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use anyhow::Result;
use std::cell::RefCell;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::rc::Rc;
use time::OffsetDateTime;
use tracing::debug;

/// an observer of evaluation_date
/// when ever calculating theta the MarketPrice price mut be deducted by the dividend
#[derive(Debug, Clone)]
pub struct MarketPrice {
    value: Real,
    market_datetime: OffsetDateTime,
    dividend: Option<Rc<RefCell<DiscreteRatioDividend>>>,
    currency: Currency,
    name: String,
    code: String,
}

impl MarketPrice {
    /// new(
    /// last_price: Real,
    /// market_datetime: OffsetDateTime,
    /// dividend: Option<DiscreteRatioDividend>,
    /// currency: Currency,
    /// name: String,
    /// code: String,
    /// )
    pub fn new(
        value: Real,
        market_datetime: OffsetDateTime,
        dividend: Option<Rc<RefCell<DiscreteRatioDividend>>>,
        currency: Currency,
        name: String,
        code: String,
    ) -> MarketPrice {
        MarketPrice {
            value,
            market_datetime,
            dividend,
            currency,
            name,
            code,
        }
    }

    pub fn set_price(&mut self, price: Real) {
        self.value = price;
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_value(&self) -> Real {
        self.value
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
    pub fn get_dividend_deduction_ratio(&self, datetime: &OffsetDateTime) -> Result<Real> {
        if let Some(dividend) = &self.dividend {
            dividend.borrow().get_deduction_ratio(datetime)
        } else {
            Ok(1.0)
        }
    }

    pub fn update_evaluation_date(&mut self, date: &EvaluationDate) -> Result<()> {
        if let Some(dividend) = &self.dividend {
            let eval_dt = date.get_date_clone();
            if self.market_datetime < eval_dt {
                for (date, div) in dividend.borrow().get_dividend_ratio().iter() {
                    if (*date > self.market_datetime) && (*date <= eval_dt) {
                        self.value *= 1.0 - div;
                        debug!(
                            "\n{} ({}) is DEDUCTED from dividens by {} on {}\n\
                            evaluation_date: {:?}, value: {}\n",
                            self.name, self.code, div, &date, &eval_dt, &self.value
                        );
                    }
                }
                self.market_datetime = eval_dt;
            } else {
                for (date, div) in dividend.borrow().get_dividend_ratio().iter() {
                    if (*date > eval_dt) && (*date <= self.market_datetime) {
                        self.value /= 1.0 - div;
                        debug!(
                            "\n{} ({}) div deduction is ROLLED back by {} on {}\n\
                            evluation_date: {:?}, value: {}\n",
                            self.name, self.code, div, &date, &eval_dt, &self.value
                        );
                    }
                }
                self.market_datetime = eval_dt;
            }
        }
        Ok(())
    }
}

/// implments arithmetic for Real
/// This operates only on the last_price
impl AddAssign<Real> for MarketPrice {
    fn add_assign(&mut self, rhs: Real) {
        self.value += rhs;
    }
}

impl SubAssign<Real> for MarketPrice {
    fn sub_assign(&mut self, rhs: Real) {
        self.value -= rhs;
    }
}

impl MulAssign<Real> for MarketPrice {
    fn mul_assign(&mut self, rhs: Real) {
        self.value *= rhs;
    }
}

impl DivAssign<Real> for MarketPrice {
    fn div_assign(&mut self, rhs: Real) {
        self.value /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::data::vector_data::VectorData;
    use crate::definitions::{DEFAULT_CLOSING_TIME, SEOUL_OFFSET};
    use crate::evaluation_date::EvaluationDate;
    use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
    use ndarray::Array1;
    use std::cell::RefCell;
    use std::rc::Rc;
    use time;
    use time::OffsetDateTime;

    #[test]
    fn test_equity_update_evaluation_date() {
        let (h, m, s) = SEOUL_OFFSET;
        let offset = time::UtcOffset::from_hms(h, m, s).unwrap();
        let eval_dt = OffsetDateTime::new_in_offset(
            time::macros::date!(2021 - 01 - 01),
            DEFAULT_CLOSING_TIME,
            offset,
        );

        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(eval_dt.clone())));

        let div_dates = vec![
            eval_dt + time::Duration::days(1),
            eval_dt + time::Duration::days(2),
            eval_dt + time::Duration::days(3),
        ];

        let spot = 100.0;
        let div_amounts = vec![1.0, 1.0, 1.0];
        let div_yields = div_amounts.iter().map(|x| x / spot).collect::<Vec<Real>>();
        let data = VectorData::new(
            Array1::from_vec(div_amounts.clone()),
            Some(div_dates.clone()),
            None,
            Some(eval_dt),
            Currency::NIL,
            "dividend vecto data".to_string(),
            "dividend vector data".to_string(),
        )
        .expect("failed to create VectorData");

        let dividend = DiscreteRatioDividend::new(
            evaluation_date.clone(),
            &data,
            spot,
            "MockMarketPrice".to_string(),
            "MockMarketPrice".to_string(),
        )
        .expect("failed to create DiscreteRatioDividend");

        let stock = Rc::new(RefCell::new(MarketPrice::new(
            spot,
            eval_dt.clone(),
            Some(Rc::new(RefCell::new(dividend))),
            Currency::KRW,
            "MockMarketPrice".to_string(),
            "MockCode".to_string(),
        )));

        evaluation_date
            .borrow_mut()
            .add_marketprice_observer(stock.clone());

        let mut test_spot = spot;
        for i in 1..div_yields.len() {
            *evaluation_date.borrow_mut() += "1D";
            let price = stock.borrow().get_value();
            test_spot *= 1.0 - div_yields[i];
            assert!(
                (price - (test_spot as Real)).abs() < 1.0e-10,
                "stock: {}, test_spot at i: {}",
                price,
                test_spot as Real
            );
        }

        // get back the evaluation_date to the original
        *evaluation_date.borrow_mut() -= "3D";
        assert!(
            (stock.borrow().get_value() - spot).abs() < 1.0e-10,
            "stock: {}",
            stock.borrow().get_value()
        );
    }
}
