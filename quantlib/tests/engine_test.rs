#[cfg(test)]
mod tests {
    use quantlib::instruments::stock_futures::StockFutures;
    use quantlib::instrument::Instrument;
    use quantlib::definitions::Real;
    use quantlib::data::vector_data::VectorData;
    use quantlib::data::value_data::ValueData;
    use quantlib::assets::currency::Currency;
    use time::macros::datetime;
    use ndarray::array;
    use ndarray::Array1;
    use std::rc::Rc;
    use quantlib::evaluation_date::EvaluationDate;
    use quantlib::pricing_engines::calculation_configuration::CalculationConfiguration;
    use quantlib::pricing_engines::match_parameter::MatchParameter;
    use std::collections::HashMap;
    use quantlib::pricing_engines::engine::Engine;

    #[test]
    fn test_engine() {
        // spot = 350.0
        let spot: Real = 350.0;
        // evaluation date = 2021-01-01 00:00:00 +09:00
        let dt = datetime!(2021-01-01 00:00:00 +09:00);
        let evaluation_date = EvaluationDate::new(dt);

        // make zero curve named "KSD". First make vector data whose values are 0.03 and 0.04
        // then make it as hash map whose key is "KSD"
        let value = array![0.03, 0.04];
        let dates = vec![
            datetime!(2021-06-01 00:00:00 +09:00),
            datetime!(2022-01-01 00:00:00 +09:00),
        ];
        let times = None;
        let market_datetime = datetime!(2021-01-01 00:00:00 +09:00);
        let name = "KSD".to_string();
        let zero_curve_data = VectorData::new(
            value, 
            Some(dates), 
            times, 
            market_datetime, 
            Currency::KRW,
            name.clone(),
        ).expect("Failed to create VectorData for zero curve");

        // the borrowing fee curve which amounts to 0.005
        let borrowing_curve_data = VectorData::new(
            array![0.005, 0.005],
            Some(vec![datetime!(2021-03-01 00:00:00 +09:00), datetime!(2021-12-01 00:00:00 +09:00)]),
            None,
            market_datetime.clone(),
            Currency::KRW,
            "KOSPI2".to_string(),
        ).expect("failed to make a vector data for borrowing fee");

        let mut zero_curve_map = HashMap::new();
        zero_curve_map.insert(name, zero_curve_data);
        zero_curve_map.insert("KOSPI2".to_string(), borrowing_curve_data);
        
        // make a vector data for dividend ratio
        let dividend_data = VectorData::new(
            Array1::from(vec![3.0, 3.0]),
            Some(vec![datetime!(2021-03-01 00:00:00 +09:00), datetime!(2021-06-01 00:00:00 +09:00)]),
            None,
            market_datetime.clone(),
            Currency::KRW,
            "KOSPI2".to_string(),
        ).expect("failed to make a vector data for dividend ratio");

        let mut dividend_data_map = HashMap::new();
        dividend_data_map.insert("KOSPI2".to_string(), dividend_data.clone());
        
        // make a stock data
        let stock_data = ValueData::new(
            spot,
            market_datetime.clone(),
            Currency::KRW,
            "KOSPI2".to_string(),
        ).expect("failed to make a stock data");

        let mut stock_data_map = HashMap::new();
        stock_data_map.insert("KOSPI2".to_string(), stock_data.clone());
        
        // make two stock futures of two maturities with the same other specs
        // then make a Instruments object with the two stock futures
        let stock_futures1 = StockFutures::new(
            350.0,
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2021-03-14 00:00:00 +09:00),
            datetime!(2021-03-14 00:00:00 +09:00),
            datetime!(2021-03-14 00:00:00 +09:00),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 Fut Mar21".to_string(),
            "165XXX1".to_string(),
        );

        let stock_futures2 = StockFutures::new(
            350.0,
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2022-06-14 00:00:00 +09:00),
            datetime!(2022-06-14 00:00:00 +09:00),
            datetime!(2022-06-14 00:00:00 +09:00),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 Fut Jun21".to_string(),
            "165XXX2".to_string(),
        );

        let inst1 = Instrument::StockFutures(stock_futures1);
        let inst2 = Instrument::StockFutures(stock_futures2);
        let inst_vec = vec![Rc::new(inst1), Rc::new(inst2)];

        // make a calculation configuration
        let calculation_configuration = CalculationConfiguration::default()
        .with_delta_calculation(true)
        .with_gamma_calculation(true)
        .with_rho_calculation(true)
        .with_div_delta_calculation(true)
        .with_rho_structure_calculation(true)
        .with_theta_calculation(true)
        .with_div_structure_calculation(true);
        
        // make a match parameter
        let mut collateral_curve_map = HashMap::new();
        collateral_curve_map.insert(String::from("KOSPI2"), String::from("KSD"));

        let mut borrowing_curve_map = HashMap::new();
        borrowing_curve_map.insert(String::from("KOSPI2"), String::from("KOSPI2"));

        let bond_discount_curve_map = HashMap::new();
        let rate_index_curve_map = HashMap::new();

        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_curve_map,
        );

        // make an engine
        let mut engine = Engine::new(
            1,
            calculation_configuration.clone(),
            evaluation_date.clone(),
            //
            HashMap::new(),
            stock_data_map,
            zero_curve_map,
            dividend_data_map,
            HashMap::new(),
            //
            Rc::new(match_parameter.clone()),
        ).expect("Failed to create an engine");

        engine.initialize(inst_vec).expect("Failed to initialize");
        engine.calculate().expect("Failed to calculate");

        let result = engine.get_calculation_result();

        println!("result: {:?}", result);
    }
}
