use crate::utils::myerror::MyError;
use time::OffsetDateTime;
use crate::evaluation_date::EvaluationDate;
use crate::assets::stock::Stock;
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use std::collections::HashMap;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::npv_result::NpvResult;
//
use std::rc::Rc;
use std::cell::RefCell;

pub struct StockFuturesPricer {
    stock: Rc<RefCell<Stock>>,
    collateral_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    borrowing_curve: Rc<RefCell<ZeroCurve>>, // or repo
    evaluation_date: Rc<RefCell<EvaluationDate>>,
}

impl StockFuturesPricer {
    pub fn new(
        stock: Rc<RefCell<Stock>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        ) -> StockFuturesPricer {
        StockFuturesPricer {
            stock,
            collateral_curve,
            borrowing_curve,
            evaluation_date,
        }
    }

    pub fn fair_forward(
        &self, 
        datetime: &OffsetDateTime) -> Real 
    {
        let stock_price = self.stock.borrow().get_last_price();
        let collateral_discount = self.collateral_curve.borrow().get_discount_factor_at_date(datetime);
        let borrowing_discount = self.borrowing_curve.borrow().get_discount_factor_at_date(datetime);
        let dividend_deduction_ratio = self.stock.borrow().get_dividend_deduction_ratio(datetime);

        let fwd: Real = stock_price * borrowing_discount / collateral_discount * dividend_deduction_ratio;
        fwd
    }

}

impl PricerTrait for StockFuturesPricer {
    fn npv_result(&self, instruments: &Instrument) -> Result<NpvResult, MyError> {
        let res = match instruments {
            Instrument::StockFutures(stock_futures) => {
                let maturity = stock_futures.get_maturity().unwrap();
                let res = NpvResult::new(self.fair_forward(&maturity));
                Ok(res)
            }
            _ => Err(MyError::BaseError {
                 file: file!().to_string(), 
                 line: line!(), 
                 contents: format!("StockFuturesPricer::npv: not supported instrument type: {}", instruments.as_trait().get_type_name().to_string())
                })
        };
        res
    }

    fn npv(&self, instruments: &Instrument) -> Result<Real, MyError> {
        let res = match instruments {
            Instrument::StockFutures(stock_futures) => {
                let maturity = stock_futures.get_maturity().unwrap();
                let res = self.fair_forward(&maturity);
                Ok(res)
            }
            _ => Err(MyError::BaseError {
                 file: file!().to_string(), 
                 line: line!(), 
                 contents: format!("StockFuturesPricer::npv: not supported instrument type: {}", instruments.as_trait().get_type_name().to_string())
                })
        };
        res
    }

    fn fx_exposure(&self, instruments: &Instrument) -> Result<Real, MyError> {
        match instruments {
            Instrument::StockFutures(stock_futures) => {
                let npv = self.npv(instruments)
                    .expect("StockFuturesPricer::fx_exposure: failed to calculate npv.");
                        
                let average_trade_price = stock_futures.get_average_trade_price();
                let unit_notional = stock_futures.get_unit_notional();
                Ok((npv - average_trade_price) * unit_notional)
            },
            _ => Err(
                MyError::BaseError {
                    file: file!().to_string(), 
                    line: line!(), 
                    contents: format!(
                        "StockFuturesPricer::fx_exposure: not supported instrument type: {}", 
                        instruments.as_trait().get_type_name().to_string(),
                    )
                }
            )
        }
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::observable::Observable;
    use crate::definitions::COUPON_PAYMENT_TIME;
    use crate::instrument::InstrumentTriat;
    use crate::{assets::currency::Currency, instruments::stock_futures::StockFutures, parameters::discrete_ratio_dividend::DiscreteRatioDividend};
    use time::macros::datetime;
    use crate::parameters::zero_curve_code::ZeroCurveCode;
    use crate::data::vector_data::VectorData;
    use ndarray::Array1;
    use time::Duration;

    #[test]
    fn test_stock_futures_engine() {
        let market_datetime = datetime!(2024-01-02 00:00:00 +09:00);
        let evaluation_date = Rc::new(
            RefCell::new(EvaluationDate::new(market_datetime.clone()))
        );

        let spot: Real = 350.0;
        let name = "KOSPI2";

        // make a vector data for dividend ratio
        let dividend_data = VectorData::new(
            Array1::from(vec![3.0, 3.0]),
            Some(vec![datetime!(2024-01-15 00:00:00 +09:00), datetime!(2024-02-15 00:00:00 +09:00)]),
            None,
            market_datetime.clone(),
            Currency::KRW,
            "KOSPI2".to_string(),
        ).expect("failed to make a vector data for dividend ratio");

        let dividend = DiscreteRatioDividend::new(
            evaluation_date.clone(),
            &dividend_data,      
            spot,
            name.to_string(),
        ).expect("failed to make a discrete ratio dividend");

        // make a stock
        let stock = Rc::new(
            RefCell::new(
                Stock::new(
                    spot,
                    market_datetime.clone(),
                    Some(Rc::new(RefCell::new(dividend))),
                    Currency::KRW,
                    name.to_string(),
                    name.to_string(),
                )
            )
        );

        // make a zero curve which represents KSD curve which is equivelantly KRWGOV - 5bp
        let mut ksd_data = VectorData::new(
            Array1::from(vec![0.0345, 0.0345]),
            Some(vec![datetime!(2021-01-02 16:00:00 +09:00), datetime!(2022-01-01 00:00:00 +09:00)]),
            None,
            market_datetime.clone(),
            Currency::KRW,
            "KSD".to_string(),
        ).expect("failed to make a vector data for KSD curve");

        let ksd_curve = Rc::new(
            RefCell::new(
                ZeroCurve::new(
                    evaluation_date.clone(),
                    &ksd_data,
                    String::from("KSD"),
                    String::from("KSD"),
                ).expect("failed to make a zero curve for KSD")
            )
        );

        ksd_data.add_observer(ksd_curve.clone());

        let dummy_curve = Rc::new(RefCell::new(
            ZeroCurve::dummy_curve().expect("failed to make a dummy curve")
        ));

        // make a stock futures with maturity 2024-03-14
        let average_trade_price = 320.0;
        let futures_maturity = datetime!(2024-03-14 13:40:00 +09:00);
        let futures = StockFutures::new(
            average_trade_price,
            datetime!(2023-01-15 09:00:00 +09:00),
            futures_maturity.clone(),
            futures_maturity.clone(),
            futures_maturity.clone(),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 Fut Mar24".to_string(),
            "165XXXX".to_string(),
        );

        let pricer = StockFuturesPricer::new(
            stock.clone(),
            ksd_curve.clone(),
            dummy_curve.clone(),
            evaluation_date.clone(),
        );

        let instrument = Instrument::StockFutures(Box::new(futures.clone()));
        let res = pricer.npv(&instrument).expect("failed to calculate npv");
        let fx_exposure = pricer.fx_exposure(&instrument)
            .expect("failed to calculate fx exposure")
            /futures.get_unit_notional();


        println!();
        println!("stock futures: \n{}", serde_json::to_string(&futures).unwrap());
        println!("spot: {}", spot);
        println!("ksd compound: {:?}", spot*(1.0/ksd_curve.borrow().get_discount_factor_at_date(futures.get_maturity().unwrap())-1.0));
        println!("dividend deduction: {:?}", spot*(1.0-(stock.borrow().get_dividend_deduction_ratio(futures.get_maturity().unwrap()))));
        println!("npv: {}", res);
        println!("average trade price: {}", average_trade_price);
        println!("fx exposure: {}", fx_exposure);

        // Replace these with the expected values
        let expected_npv = 346.38675;
        let expected_fx_exposure = 26.38675;
        let expected_coupons = HashMap::<OffsetDateTime, Real>::new(); // Assuming coupons is a HashMap

        assert!(
            (res - expected_npv).abs() < 1.0e-6, 
            "Unexpected npv (expected: {}, actual: {})",
            expected_npv,
            res
        );

        assert!(
            (fx_exposure - expected_fx_exposure).abs() < 1.0e-6, 
            "Unexpected fx exposure (expected: {}, actual: {})",
            expected_fx_exposure,
            fx_exposure
        );

    }
}




