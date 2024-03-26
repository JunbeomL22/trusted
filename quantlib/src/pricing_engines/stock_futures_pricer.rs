use crate::evaluation_date::EvaluationDate;
use crate::assets::{
    stock::Stock,
    currency::Currency,
};
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::InstrumentTrait;
//
use time::OffsetDateTime;
use anyhow::{anyhow, Context, Result};
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

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
        datetime: &OffsetDateTime
    ) -> Result<Real> {
        let stock_price = self.stock.borrow().get_last_price();
        let collateral_discount = self.collateral_curve
            .borrow()
            .get_discount_factor_at_date(datetime)
            .context("(StockFuturesPricer::fair_forward) failed to get collateral discount factor at date")?;
        let borrowing_discount = self.borrowing_curve
            .borrow()
            .get_discount_factor_at_date(datetime)
            .context("(StockFuturesPricer::fair_forward) failed to get borrowing discount factor at date")?;
        let dividend_deduction_ratio = self.stock
            .borrow()
            .get_dividend_deduction_ratio(datetime)
            .context("(StockFuturesPricer::fair_forward) failed to get dividend deduction ratio at date")?;

        let fwd: Real = stock_price * borrowing_discount / collateral_discount * dividend_deduction_ratio;
        Ok(fwd)
    }

}

impl PricerTrait for StockFuturesPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let res = match instrument {
            Instrument::StockFutures(stock_futures) => {
                let maturity = stock_futures.get_maturity().unwrap();
                let res = NpvResult::new_from_npv(self.fair_forward(&maturity)?);
                Ok(res)
            }
            _ => Err(anyhow!(
                "StockFuturesPricer::npv: not supported instrument type: {}", 
                instrument.get_type_name().to_string()))
        };
        res
    }

    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let res = match instrument {
            Instrument::StockFutures(stock_futures) => {
                let maturity = stock_futures.get_maturity().unwrap();
                self.fair_forward(&maturity)
            }
            _ => Err(anyhow!(
                "StockFuturesPricer::npv: not supported instrument type: {}", 
                instrument.get_type_name().to_string()))
        };
        res
    }

    fn fx_exposure(&self, instrument: &Instrument, _npv: Real) -> Result<HashMap<Currency, Real>> {
        match instrument {
            Instrument::StockFutures(stock_futures) => {
                let npv = self.npv(instrument)
                    .expect("StockFuturesPricer::fx_exposure: failed to calculate npv.");
                        
                let average_trade_price = stock_futures.get_average_trade_price();
                let unit_notional = stock_futures.get_unit_notional();
                let exposure = (npv - average_trade_price) * unit_notional;
                let currency = stock_futures.get_currency();
                let res = HashMap::from_iter(vec![(currency.clone(), exposure)]);
                Ok(res)
            },
            _ => Err(anyhow!(
                "StockFuturesPricer::fx_exposure: not supported instrument type: {}", 
                instrument.get_type_name().to_string()
            ))
        }
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::observable::Observable;
    use crate::instrument::InstrumentTrait;
    use crate::{assets::currency::Currency, instruments::stock_futures::StockFutures, parameters::discrete_ratio_dividend::DiscreteRatioDividend};
    use time::macros::datetime;
    use crate::data::vector_data::VectorData;
    use ndarray::Array1;
    use anyhow::Result;

    #[test]
    fn test_stock_futures_engine() -> Result<()> {
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

        let instrument = Instrument::StockFutures(futures.clone());
        let res = pricer.npv(&instrument).expect("failed to calculate npv");
        let fx_exposure = pricer.fx_exposure(&instrument, res)?
            .get(&Currency::KRW)
            .unwrap()
            / futures.get_unit_notional();


        println!();
        println!("stock futures: \n{}", serde_json::to_string(&futures).unwrap());
        println!("spot: {}", spot);
        println!("ksd compound: {:?}", spot*(1.0/ksd_curve.borrow().get_discount_factor_at_date(futures.get_maturity().unwrap())?-1.0));
        println!("dividend deduction: {:?}", spot*(1.0-(stock.borrow().get_dividend_deduction_ratio(futures.get_maturity().unwrap()))?));
        println!("npv: {}", res);
        println!("average trade price: {}", average_trade_price);
        println!("fx exposure: {}", fx_exposure);

        // Replace these with the expected values
        let expected_npv = 346.38675;
        let expected_fx_exposure = 26.38675;
        //let expected_coupons = HashMap::<OffsetDateTime, Real>::new(); // Assuming coupons is a HashMap

        assert!(
            (res - expected_npv).abs() < 1.0e-5, 
            "Unexpected npv (expected: {}, actual: {})",
            expected_npv,
            res
        );

        assert!(
            (fx_exposure - expected_fx_exposure).abs() < 1.0e-5, 
            "Unexpected fx exposure (expected: {}, actual: {})",
            expected_fx_exposure,
            fx_exposure
        );
        Ok(())
    }
}




