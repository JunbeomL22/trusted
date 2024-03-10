use crate::utils::myerror::MyError;
use time::OffsetDateTime;
use crate::evaluation_date::EvaluationDate;
use crate::assets::stock::Stock;
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use std::collections::HashMap;
use crate::parameters::zero_curve::ZeroCurve;
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
    fn npv(&self, instruments: &Instrument) -> Result<Real, MyError> {
        let res = match instruments {
            Instrument::StockFutures(stock_futures) => {
                let maturity = stock_futures.get_maturity().unwrap();
                Ok(self.fair_forward(&maturity))
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

    fn coupons(
        &self, 
        _instruments: &Instrument,
        _start_date: &OffsetDateTime,
        _end_date: &OffsetDateTime,
    ) -> Result<HashMap<OffsetDateTime, Real>, MyError> {
        Ok(HashMap::new())
    }
}
    /* 
    /// 1% pnl delta
    fn delta(&self) -> HashMap<String, HashMap<String, Real>> {
        let mut all_deltas: HashMap<String, HashMap<String, Real>> = HashMap::new();

        for instrument in &self.instruments {
            let code = instrument.get_code();
            let underlying = instrument.get_underlying_asset();
            let mut instrument_delta: HashMap<String, Real> = HashMap::new();
            let delta_bump_ratio = self.configuration.get_delta_bump_ratio();
            for underlying_code in underlying {
                // bump self.stock by delta_bump_ratio
                *self.stock.borrow_mut() *= 1.0 + delta_bump_ratio;
                let up_price = self.fair_forward(instrument.get_maturity());
                // bump self.stock by -delta_bump_ratio
                *self.stock.borrow_mut() *= (1.0 - delta_bump_ratio) / (1.0 + delta_bump_ratio);
                let down_price = self.fair_forward(instrument.get_maturity());
                *self.stock.borrow_mut() /= 1.0 - delta_bump_ratio;
                let mut delta = (up_price - down_price) / 2.0 * DELTA_PNL_UNIT / delta_bump_ratio;
                delta *= instrument.get_unit_notional();
                instrument_delta.insert(underlying_code.clone(), delta);
            }
            all_deltas.insert(code.clone(), instrument_delta);
        }
        all_deltas
    }

    fn set_delta(&mut self) {
        let delta = self.delta();
        for (code, value) in delta {
            self.results.get_mut(&code).unwrap().set_delta(value);
        }
    }

    /// fx exposure in value based, i.e., unit_notional is condeired
    /// fx_exposure = (npv - average_trade_price) * unit_notional
    fn fx_exposure(&self) -> HashMap<String, Real> {
        let mut fx_exposure_map: HashMap<String, Real> = HashMap::new();
        for instrument in &self.instruments {
            let code = instrument.get_code();
            let npv = self.results.get(code)
            .expect(format!("{} is not in the results", code).as_str())
            .get_npv()
            .expect(format!("{} does not have npv", code).as_str());
            let fx_exposure = (npv - instrument.get_average_trade_price()) * instrument.get_unit_notional();
            fx_exposure_map.insert(code.clone(), fx_exposure);
        }
        fx_exposure_map
    }

    fn set_fx_exposure(&mut self) {
        let fx_exposure = self.fx_exposure();
        for (code, value) in fx_exposure {
            self.results.get_mut(&code).unwrap().set_fx_exposure(value);
        }
    }

    fn theta(&self) -> HashMap<String, Real> {
        let mut all_thetas: HashMap<String, Real> = HashMap::new();
        
        // find the minimum maturity in the futures
        let mut min_maturity = self.instruments[0].get_maturity();
        for instrument in &self.instruments {
            if instrument.get_maturity() < min_maturity {
                min_maturity = instrument.get_maturity();
            }
        }
        // adjust theta_day so that it does not exceed the maturity
        let mut theta_sec = (self.configuration.get_theta_day() * 24 * 60 * 60) as Time;

        let gap_sec = (*min_maturity - self.evaluation_date.borrow().get_date_clone()).whole_seconds() as Time;

        if  gap_sec < theta_sec {
            theta_sec = gap_sec;
        };

        let theta_day_string = format!("{}sec", theta_sec);
        *self.evaluation_date.borrow_mut() += theta_day_string.as_str();
        let up_price = self.npv();
        *self.evaluation_date.borrow_mut() -= theta_day_string.as_str();

        for instrument in &self.instruments {
            let code = instrument.get_code();
            let npv = self.results.get(code)
            .expect(format!("{} is not in the results", code).as_str())
            .get_npv()
            .expect(format!("{} does not have npv", code).as_str());
            let theta = (up_price.get(code).unwrap() - npv) / self.configuration.get_theta_day() as Real;
            all_thetas.insert(code.clone(), theta * instrument.get_unit_notional());
        }
        all_thetas
    }

    fn set_theta(&mut self) {
        // set theta day in results
        let theta_day = self.configuration.get_theta_day();
        for (_, result) in &mut self.results {
            result.set_theta_day(theta_day);
        }

        let theta = self.theta();
        // set thetas to the results
        for (code, value) in theta {
            self.results.get_mut(&code).unwrap().set_theta(value);
        }
    }

    fn rho(&self) -> HashMap<String, HashMap<String, Real>> {
        let rho_bump_value = self.configuration.get_rho_bump_value();
        // make a hash map for the VectorData with the key as its name
        let mut curve_map: HashMap<String, Rc<RefCell<VectorData>>> = HashMap::new();
        // collateral curve
        curve_map.insert(
            self.collateral_curve.borrow().get_name_clone(),
            self.collateral_curve.borrow().get_data_clone()
        );
        // borrowing curve
        curve_map.insert(
            self.borrowing_curve.borrow().get_name_clone(),
            self.borrowing_curve.borrow().get_data_clone()
        );

        let mut vec_format: Vec<(String, String, Real)> = vec![]; // inst_code, curve_name, rho

        for (curve_name, curve_data) in curve_map {
            *curve_data.borrow_mut() += rho_bump_value;
            let up_price = self.npv();
            *curve_data.borrow_mut() -= rho_bump_value;
            for instrument in &self.instruments {
                let code = instrument.get_code();
                let npv = self.results.get(code)
                .expect(format!("{} is not in the results", code).as_str())
                .get_npv()
                .expect(format!("{} does not have npv", code).as_str());
                let rho = (up_price.get(code).unwrap() - npv) / rho_bump_value * RHO_PNL_UNIT;
                let unit = instrument.get_unit_notional();
                vec_format.push((code.clone(), curve_name.clone(), rho*unit));
            }
        }
        let mut all_rhos: HashMap<String, HashMap<String, Real>> = HashMap::new();
        // convert vec_format to all_rhos
        for (code, curve_name, rho) in vec_format {
            if all_rhos.contains_key(&code) {
                all_rhos.get_mut(&code).unwrap().insert(curve_name, rho);
            } else {
                let mut curve_rho: HashMap<String, Real> = HashMap::new();
                curve_rho.insert(curve_name, rho);
                all_rhos.insert(code.clone(), curve_rho);
            }
        }
        all_rhos
    }

    fn set_rho(&mut self) {
        let rho = self.rho();
        for (inst_code, value) in rho {
            self.results.get_mut(&inst_code).unwrap().set_rho(value.clone());
        }
    }

    fn get_calculation_result(&self) -> &HashMap<String, CalculationResult> {
        &self.results
    }
    */


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::observable::Observable;
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
                    ZeroCurveCode::KSD,
                    "KSD".to_string(),
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

        let coupons = pricer.coupons(
            &instrument, 
            &evaluation_date.borrow().get_date_clone(),
            &(evaluation_date.borrow().get_date_clone() + Duration::days(1)))
            .expect("failed to calculate coupons");


        /*
        println!();
        println!("stock futures: \n{}", serde_json::to_string(&futures).unwrap());
        println!("spot: {}", spot);
        println!("ksd compound: {:?}", spot*(1.0/ksd_curve.borrow().get_discount_factor_at_date(futures.get_maturity().unwrap())-1.0));
        println!("dividend deduction: {:?}", spot*(1.0-(stock.borrow().get_dividend_deduction_ratio(futures.get_maturity().unwrap()))));
        println!("npv: {}", res);
        println!("average trade price: {}", average_trade_price);
        println!("fx exposure: {}", fx_exposure);
        println!("coupons: {:?}", coupons);
        */

        // Replace these with the expected values
        let expected_npv = 346.38675;
        let expected_fx_exposure = 26.38675;
        let expected_coupons = HashMap::new(); // Assuming coupons is a HashMap

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

        assert!(
            coupons == expected_coupons,
            "Unexpected coupons (expected: {:?}, actual: {:?})",
            expected_coupons,
            coupons
        );
    }
}




