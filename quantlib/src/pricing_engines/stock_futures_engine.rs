use crate::instruments::instrument_info::InstrumentInfo;
use time::OffsetDateTime;
use crate::evaluation_date::EvaluationDate;
use crate::parameters::zero_curve::ZeroCurve;
use crate::assets::stock::Stock;
use crate::instruments::stock_futures::StockFutures;
use crate::definitions::Real;
use crate::pricing_engines::engine::Engine;
use crate::pricing_engines::calculation_result::CalculationResult;
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use std::collections::HashMap;
use std::vec;
use crate::data::vector_data::VectorData;
use crate::instrument::Instrument;
use crate::definitions::Time;
use crate::definitions::{DELTA_PNL_UNIT, RHO_PNL_UNIT};
//
use std::rc::Rc;
use std::cell::RefCell;

pub struct StockFuturesEngine {
    stock: Rc<RefCell<Stock>>,
    collateral_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    borrowing_curve: Rc<RefCell<ZeroCurve>>, // or repo
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    instruments: Vec<StockFutures>,
    results: HashMap<String, CalculationResult>, // Code -> CalculationResult
    configuration: CalculationConfiguration,
}

impl StockFuturesEngine {
    pub fn initialize(
        stock: Rc<RefCell<Stock>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        ) -> StockFuturesEngine {
        StockFuturesEngine {
            stock,
            collateral_curve,
            borrowing_curve,
            evaluation_date,
            instruments: vec![],
            results: HashMap::new(),
            configuration: CalculationConfiguration::default(),
        }
    }

    pub fn set_instruments(&mut self, instruments: &Vec<StockFutures>) {
        // clone the instruments to self.instruments and initialize the results
        self.instruments = instruments.clone();
        // sanity check: the stock futures must have the same stock as its underlying
        for instrument in instruments {
            assert_eq!(
                &instrument.get_underlying_asset()[0], 
                self.stock.borrow().get_name(),
                "(StockFuturesEngine::set_instruments) the underlying asset of the stock futures is not the same as the stocks"
            );
        }
        
        for instrument in instruments {
            let code = instrument.get_code().clone();
            let result = CalculationResult::default();
            self.results.insert(code, result);
        }
    }

    pub fn with_instruments(mut self, instruments: &Vec<StockFutures>) -> StockFuturesEngine {
        self.set_instruments(instruments);
        // initialize the results with the 
        for instrument in instruments {
            let code = instrument.get_code().clone();
            
            let base_info = InstrumentInfo::new(
                instrument.get_name().clone(),
                code.clone(),
                instrument.get_currency().clone(),
                instrument.type_name().to_string(),
                instrument.get_unit_notional(),
                Some(instrument.get_maturity().clone()),
            );

            let result = CalculationResult::new(
                base_info, 
                self.evaluation_date.borrow().get_date_clone()
            );

            self.results.insert(code, result);
        }
        self
    }

    pub fn with_configuration(mut self, configuration: CalculationConfiguration) -> StockFuturesEngine {
        self.set_configuration(configuration);
        self
    }

    pub fn set_configuration(&mut self, configuration: CalculationConfiguration) {
        self.configuration = configuration;
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

    pub fn get_configuration(&self) -> &CalculationConfiguration {
        &self.configuration
    }
}

impl Engine for StockFuturesEngine {
    fn calculate(&mut self) {
        self.set_npv();
        self.set_value();
        self.set_fx_exposure();
        if self.configuration.get_delta_calculation() {
            self.set_delta();
        }
        if self.configuration.get_theta_calculation() {
            self.set_theta();
        }
        if self.configuration.get_rho_calculation() {
            self.set_rho();
        }

        if self.configuration.get_theta_calculation() {
            self.set_theta();
        }

    }
    fn npv(&self) -> HashMap<String, Real> {
        let mut npv: HashMap<String, Real> = HashMap::new();
        for instrument in &self.instruments {
            let code = instrument.get_code();
            let maturity = instrument.get_maturity();
            let fwd = self.fair_forward(maturity);
            npv.insert(code.clone(), fwd);
        }
        npv
    }

    fn set_npv(&mut self) {
        let npv = self.npv();
        for (code, value) in npv {
            self.results.get_mut(&code).unwrap().set_npv(value);
        }
    }

    fn set_value(&mut self) {
        for instrument in &self.instruments {
            let code = instrument.get_code();
            self.results.get_mut(code)
            .expect(format!("{} is not in the results", code).as_str())
            .set_value();
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::observable::Observable;
    use crate::{assets::currency::Currency, instruments::stock_futures::StockFutures, parameters::discrete_ratio_dividend::DiscreteRatioDividend};
    use time::{macros::datetime, UtcOffset};
    use crate::definitions::SEOUL_OFFSET;
    use crate::parameters::enums::ZeroCurveCode;
    use crate::data::vector_data::VectorData;
    use ndarray::Array1;

    #[test]
    fn test_stock_futures_engine() {
        let market_datetime = datetime!(2024-01-02 00:00:00 +09:00);
        let evaluation_date = Rc::new(
            RefCell::new(EvaluationDate::new(market_datetime.clone()))
        );
        let (h, m, s) = SEOUL_OFFSET;
        let offset = UtcOffset::from_hms(h, m, s).unwrap();

        let spot: Real = 350.0;
        let name = "KOSPI2";

        // make a vector data for dividend ratio
        let dividend_data = VectorData::new(
            Array1::from(vec![3.0, 3.0]),
            Some(vec![datetime!(2024-01-15 00:00:00 +09:00), datetime!(2024-02-15 00:00:00 +09:00)]),
            None,
            market_datetime.clone(),
            "KOSPI2".to_string(),
        );

        let dividend = DiscreteRatioDividend::new(
            evaluation_date.clone(),
            &dividend_data,
            offset,
            spot,
            name.to_string(),
        );

        // make a stock
        let stock = Rc::new(
            RefCell::new(
                Stock::new(
                    spot,
                    market_datetime.clone(),
                    Some(dividend),
                    Currency::KRW,
                    name.to_string(),
                    name.to_string(),
                )
            )
        );

        // make a zero curve which represents KSD curve which is equivelantly KRWGOV - 5bp
        let _ksd_data = VectorData::new(
            Array1::from(vec![0.0345, 0.0345]),
            Some(vec![datetime!(2021-01-02 16:00:00 +09:00), datetime!(2022-01-01 00:00:00 +09:00)]),
            None,
            market_datetime.clone(),
            "KSD".to_string(),
        );

        let ksd_data = Rc::new(
            RefCell::new(
                _ksd_data
            )
        );

        let ksd_curve = Rc::new(
            RefCell::new(
                ZeroCurve::new(
                    evaluation_date.clone(),
                    ksd_data.clone(),
                    ZeroCurveCode::KSD,
                    "KSD".to_string(),
                )
            )
        );

        ksd_data.borrow_mut().add_observer(ksd_curve.clone());

        let dummy_curve = Rc::new(
            RefCell::new(ZeroCurve::dummy_curve())
        );

        // make a stock futures with maturity 2024-03-14
        let average_trade_price = 320.0;
        let futures_maturity = datetime!(2024-03-14 13:40:00 +09:00);
        let futures = StockFutures::new(
            average_trade_price,
            datetime!(2023-01-15 09:00:00 +09:00),
            market_datetime.clone(),
            futures_maturity.clone(),
            futures_maturity.clone(),
            250_000.0,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 Fut Mar24".to_string(),
            "165XXXX".to_string(),
        );

        // make a stock futures engine
        let configuration = CalculationConfiguration::default()
        .with_delta_calculation(true)
        .with_delta_bump_ratio(0.01) // 1% bump for delta calculation
        .with_theta_calculation(true)
        .with_theta_day(20) // 1 day for theta calculation
        .with_rho_calculation(true)
        .with_rho_bump_value(0.0001); // 0.01% bump for rho calculation

        let mut engine = StockFuturesEngine::initialize(
            stock.clone(),
            ksd_curve.clone(),
            dummy_curve.clone(),
            evaluation_date.clone())
            .with_instruments(&vec![futures])
            .with_configuration(configuration);

        // test calculate
        engine.calculate();
        println!("stock futures calculation example:\n");
        println!("  *configuration = {:?}\n", engine.configuration);
        println!("  *stock = {:?}\n", engine.stock.borrow());
        println!("  *instruments = {:?}\n", engine.instruments);
        println!("  *results = {:?}\n", engine.results);
        
        assert!(engine.results.len() > 0);
        assert!(
            (engine.stock.borrow().get_last_price() - 350.0).abs() < 1.0e-6,
            "stock price: {}, expected: 350.0. Have you changed any logic in stock and StockFuturesEngine::theta?",
            engine.stock.borrow().get_last_price()
        );

        //check delta is approximately 865,966.75
        assert!(
            (engine.results.get("165XXXX").as_ref().unwrap().get_delta().as_ref().unwrap().get("KOSPI2").unwrap() - 865_966.75).abs() < 1.0e-6,
            "delta is not approximately 865,966.75. delta: {}, expected: 865,966.75",
            engine.results.get("165XXXX").as_ref().unwrap().get_delta().as_ref().unwrap().get("KOSPI2").unwrap()
        );

        //check theta is approximately -8,153.9155
        assert!(
            (engine.results.get("165XXXX").as_ref().unwrap().get_theta().unwrap() + 8_153.9155).abs() < 1.0e-6,
            "theta is not approximately -8,153.9155. theta: {}, expected: -8,153.9155",
            engine.results.get("165XXXX").as_ref().unwrap().get_theta().unwrap()
        );

    }
}




