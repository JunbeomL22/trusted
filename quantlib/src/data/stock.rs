use crate::data::observable::Observable;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::parameter::Parameter;
use time::OffsetDateTime;

/// an observer of evaluation_date 
/// when ever calculating theta the Stock price mut be deducted by the dividend
#[derive(Debug, Clone)]
pub struct Stock {
    last_price: Real,
    market_datetime: OffsetDateTime,
    cached_datetime: OffsetDateTime,
    dividend: Option<DiscreteRatioDividend>,
    name: String,
}

impl Stock {
    pub fn new(
        last_price: Real, 
        market_datetime: OffsetDateTime, 
        dividend: Option<DiscreteRatioDividend>,
        name: String
    ) -> Stock {
        Stock {
            last_price,
            market_datetime.clone(),
            cached_datetime: market_datetime,
            dividend,
            name,
        }
    }

    pub fn get_last_price(&self) -> Real {
        self.last_price
    }

    pub fn get_market_datetime(&self) -> &OffsetDateTime {
        &self.market_datetime
    }

    pub fn get_dividend(&self) -> &Option<DiscreteRatioDividend> {
        &self.dividend
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl Parameter for Stock {
    /// the stock price must be deducted by the dividend
    /// the amount is the sum of the dividend amount 
    /// between the market_datetime and the EvaluationDate
    fn update_evaluation_date(&mut self, data: &EvaluationDate) {
        if let Some(dividend) = &self.dividend {
            let eval_dt = data.get_date_clone();
            if self.market_datetime < eval_dt {   
                let mut deduction: Real = 0.0;
                for (&date, &div) in dividend.get_dividend().iter() {
                    if (date > self.market_datetime) && (date <= eval_dt) {
                        self.last_price -= div;
                    }
                }
                self.market_datetime = eval_dt;   
            } else {
                let mut addition: Real = 0.0;
                for (&date, &div) in dividend.get_dividend().iter() {
                    if (date > eval_dt) && (date <= self.market_datetime) {
                        self.last_price += div;
                    }
                }
            }
            
        }        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
    use time::OffsetDateTime;
    use time;
    use crate::evaluation_date::EvaluationDate;
    use crate::definitions::{CLOSING_TIME, SOUTH_KOREA_OFFSET};
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::data::vector_data::VectorData;

    #[test]
    fn test_stock_update_evaluation_date() {
        let (h, m, s) = SOUTH_KOREA_OFFSET;
        let offset = time::UtcOffset::from_hms(h, m, s);
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
        let mut dividend = DiscreteRatioDividend::new(HashMap::new());
        let data = VectorData::new(
            div_amounts.clone(),
            Some(div_dates.clone()),
            None,
            eval_dt.clone(),
            "dividend vecto data".to_string()
        );

        let dividend = DiscreteRatioDividend::new(
            evalaution_date.clone(),
            &data,
            spot,
            offset,
            "MockStock".to_string(),
        );

        let stock = Rc::new(RefCell::new(
            Stock::new(
                spot,
                eval_dt.clone(),
                Some(dividend),
                "MockStock".to_string()
            )
        ));

        evaluation_date.borrow_mut().add_observer(stock.clone());

        for i in 1..div_dates.len() {
            let period_str = format!("{}D", i);
            *evaluation_date.borrow_mut() += period_str;
            assert!(
                (stock.borrow().get_last_price() - (spot - i as Real)).abs() < 1.0e-10,
                "period_str: {}, stock: {}",
                period_str,
                stock.borrow().get_last_price()
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


