#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use ndarray::array;
    use ndarray::Array1;
    use quantlib::currency::{Currency, FxCode};
    use quantlib::data::value_data::ValueData;
    use quantlib::data::vector_data::VectorData;
    use quantlib::definitions::Real;
    use quantlib::enums::{CreditRating, IssuerType, RankType};
    use quantlib::enums::{OptionDailySettlementType, OptionExerciseType, OptionType};
    use quantlib::instrument::{Instrument, Instruments};
    use quantlib::instruments::{
        bond::Bond, cash::Cash, futures::Futures, stock::Stock, vanilla_option::VanillaOption,
    };
    use quantlib::pricing_engines::engine_generator::{EngineGenerator, InstrumentCategory};
    use quantlib::pricing_engines::match_parameter::MatchParameter;
    use quantlib::pricing_engines::{
        calculation_configuration::CalculationConfiguration, calculation_result::CalculationResult,
    };
    use quantlib::time::calendar::Calendar;
    use quantlib::time::calendars::{southkorea::SouthKorea, southkorea::SouthKoreaType};
    use quantlib::time::conventions::{
        BusinessDayConvention, DayCountConvention, PaymentFrequency,
    };
    use quantlib::time::jointcalendar::JointCalendar;
    use quantlib::utils::tracing_timer::CustomOffsetTime;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::time::Instant;
    use time::{macros::datetime, Duration};
    use tracing::{info, span, Level};
    use tracing_appender::non_blocking;
    use tracing_appender::rolling;
    use tracing_subscriber::fmt::{self, writer::MakeWriterExt};
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    fn test_engine() -> Result<()> {
        let theta_day = 100;
        let start_time = Instant::now();
        // Set up rolling file appender
        let file_appender = rolling::daily("test_logs", "engine-test.log");
        let (non_blocking_appender, _guard) = non_blocking(file_appender);

        // Set up console layer
        let custom_time = CustomOffsetTime::new(9, 0, 0);

        let err_layer = fmt::layer()
            .with_writer(std::io::stderr.with_max_level(Level::ERROR))
            .with_timer(custom_time.clone());

        let console_layer = fmt::layer()
            .with_writer(std::io::stdout.with_max_level(Level::DEBUG))
            .with_timer(custom_time.clone());

        let file_layer = fmt::layer()
            .with_writer(
                non_blocking_appender
                    .with_max_level(Level::DEBUG)
                    .with_min_level(Level::INFO),
            )
            .with_timer(custom_time);

        // Combine console and file layers into a subscriber
        let subscriber = tracing_subscriber::registry()
            .with(file_layer)
            .with(err_layer)
            .with(console_layer);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Setting default subscriber failed");

        // Create a new span with an `info` level
        let main_span = span!(Level::INFO, "main (engine)");
        let _enter = main_span.enter();

        let spot: Real = 350.0;
        // evaluation date = 2021-01-01 00:00:00 +09:00
        let dt = datetime!(2024-03-13 16:30:00 +09:00);

        // make zero curve named "KSD". First make vector data whose values are 0.03 and 0.04
        // then make it as hash map whose key is "KSD"
        let value = array![0.03358, 0.03358];
        let dates = vec![
            datetime!(2025-03-13 00:00:00 +09:00),
            datetime!(2026-03-13 00:00:00 +09:00),
        ];

        let times = None;
        let market_datetime = dt.clone();
        let zero_curve1 = "KSD".to_string();
        let zero_curve_data1 = VectorData::new(
            &value - 0.0005,
            Some(dates.clone()),
            times.clone(),
            Some(market_datetime),
            Currency::KRW,
            zero_curve1.clone(),
            zero_curve1.clone(),
        )
        .expect("Failed to create VectorData for KSD");

        let zero_curve2 = "KRWGOV".to_string();
        let zero_curve_data2 = VectorData::new(
            value,
            Some(dates.clone()),
            times,
            Some(market_datetime),
            Currency::KRW,
            zero_curve2.clone(),
            zero_curve2.clone(),
        )
        .expect("Failed to create VectorData for KRWGOV");

        let funding_curve1 = "Discount(KRW)".to_string();
        let funding_curve_data1 = VectorData::new(
            array![0.04, 0.04],
            Some(dates.clone()),
            None,
            Some(market_datetime),
            Currency::KRW,
            funding_curve1.clone(),
            funding_curve1.clone(),
        )
        .expect("failed to make a vector data for funding curve");

        // the borrowing fee curve which amounts to 0.005
        let bor_curve_name = "KOSPI2".to_string();
        let borrowing_curve_data = VectorData::new(
            array![0.005, 0.005],
            Some(dates.clone()),
            None,
            Some(market_datetime),
            Currency::KRW,
            bor_curve_name.clone(),
            bor_curve_name.clone(),
        )
        .expect("failed to make a vector data for borrowing fee");

        //
        // mapping construction
        let mut zero_curve_map = HashMap::new();
        zero_curve_map.insert(zero_curve1, zero_curve_data1);
        zero_curve_map.insert(zero_curve2, zero_curve_data2);
        zero_curve_map.insert("KOSPI2".to_string(), borrowing_curve_data);
        zero_curve_map.insert(funding_curve1.clone(), funding_curve_data1);

        let mut equity_vol_map = HashMap::new();
        let equity_surface_map = HashMap::new();

        //let _equity_surface_data = surfacedatasample!(&market_datetime, spot);
        let equity_constant_vol1 = ValueData::new(
            0.2,
            Some(market_datetime),
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2".to_string(),
        )
        .expect("failed to make a value data for equity volatility");

        //equity_surface_map.insert("KOSPI2".to_string(), equity_surface_data);
        equity_vol_map.insert("KOSPI2".to_string(), equity_constant_vol1);

        let fx_str1 = "USDKRW";
        let fx_code1 = FxCode::from(fx_str1);
        let fx1 = ValueData::new(
            1300.0,
            Some(market_datetime),
            Currency::KRW,
            fx_str1.to_string(),
            fx_str1.to_string(),
        )
        .expect("failed to make a value data for fx rate");
        let mut fx_data_map = HashMap::new();

        fx_data_map.insert(fx_code1, fx1);

        // make a vector data for dividend ratio
        let div_name = "KOSPI2".to_string();
        let dividend_data = VectorData::new(
            Array1::from(vec![3.0, 3.0]),
            Some(vec![
                datetime!(2024-06-01 00:00:00 +09:00),
                datetime!(2025-01-01 00:00:00 +09:00),
            ]),
            None,
            Some(market_datetime),
            Currency::KRW,
            div_name.clone(),
            div_name.clone(),
        )
        .expect("failed to make a vector data for dividend ratio");

        let mut dividend_data_map = HashMap::new();
        dividend_data_map.insert("KOSPI2".to_string(), dividend_data.clone());

        // make a stock data
        let stock_name = "KOSPI2".to_string();
        let stock_data = ValueData::new(
            spot,
            Some(market_datetime),
            Currency::KRW,
            stock_name.clone(),
            stock_name.clone(),
        )
        .expect("failed to make a stock data");

        let mut stock_data_map = HashMap::new();
        stock_data_map.insert("KOSPI2".to_string(), stock_data.clone());

        // make two stock futures of two maturities with the same other specs
        // then make a Instruments object with the two stock futures
        let stock_futures1 = Futures::new(
            350.0,
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2021-01-11 00:00:00 +09:00),
            datetime!(2024-06-14 00:00:00 +09:00),
            datetime!(2024-06-14 00:00:00 +09:00),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 Fut Mar21".to_string(),
            "165XXX1".to_string(),
        );

        let stock_futures2 = Futures::new(
            350.0,
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2025-06-14 00:00:00 +09:00),
            datetime!(2025-06-14 00:00:00 +09:00),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 Fut Jun21".to_string(),
            "165XXX2".to_string(),
        );

        let issuedate = datetime!(2020-01-01 16:30:00 +09:00);
        let maturity = issuedate + Duration::days(365 * 6);
        let issuer_name = "Korea Gov";
        let bond_name = "Virtual KTB";
        let bond_code = "KRxxxxxxxxxx";
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;

        let bond_currency = Currency::KRW;
        let issuer_type = IssuerType::Government;
        let credit_rating = CreditRating::None;

        let bond = Bond::new_from_conventions(
            issuer_type,
            credit_rating,
            issuer_name.to_string(),
            RankType::Senior,
            bond_currency,
            10_000.0,
            false,
            issuedate.clone(),
            issuedate.clone(),
            None,
            maturity,
            Some(0.03),
            None,
            None,
            None,
            calendar,
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
            bond_name.to_string(),
            bond_code.to_string(),
        )?;

        let issuedate2 = datetime!(2022-12-10 16:30:00 +09:00);
        let maturity2 = datetime!(2025-12-10 16:30:00 +09:00);
        let issuer_name2 = "Korea Gov";
        let bond_name2 = "국고채권 04250-2512(22-13)";
        let bond_code2 = "KR103501GCC0";
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;

        let bond_currency2 = Currency::KRW;
        let issuer_type2 = IssuerType::Government;
        let credit_rating2 = CreditRating::None;

        let bond2 = Bond::new_from_conventions(
            issuer_type2,
            credit_rating2,
            issuer_name2.to_string(),
            RankType::Senior,
            bond_currency2,
            10_000.0,
            false,
            issuedate2.clone(),
            issuedate2.clone(),
            None,
            maturity2,
            Some(0.0425),
            None,
            None,
            None,
            calendar,
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
            bond_name2.to_string(),
            bond_code2.to_string(),
        )?;

        // option
        let option1 = VanillaOption::new(
            285.0,
            250_000.0,
            datetime!(2021-01-01 00:00:00 +09:00),
            datetime!(2024-09-13 00:00:00 +09:00),
            datetime!(2024-09-13 00:00:00 +09:00),
            datetime!(2024-09-13 00:00:00 +09:00),
            vec![String::from("KOSPI2")],
            Currency::KRW,
            Currency::KRW,
            OptionType::Put,
            OptionExerciseType::European,
            OptionDailySettlementType::NotSettled,
            "KOSPI2 Call Sep21".to_string(),
            "165XXX3".to_string(),
        );

        let cash = Cash::new(
            Currency::USD,
            "USD Cash".to_string(),
            "USD Cash".to_string(),
        );

        let stock = Stock::new(
            "KOSPI2".to_string(),
            "KOSPI2".to_string(),
            vec!["KOSPI2".to_string()],
            Currency::KRW,
            None,
        );

        let inst1 = Instrument::Futures(stock_futures1);
        let inst2 = Instrument::Futures(stock_futures2);
        let inst3: Instrument = Instrument::Bond(bond);
        let inst4: Instrument = Instrument::Bond(bond2);
        let inst5 = Instrument::VanillaOption(option1);
        let inst6 = Instrument::Cash(cash);
        let inst7 = Instrument::Stock(stock);

        let inst_vec = vec![
            Rc::new(inst1),
            Rc::new(inst2),
            Rc::new(inst3),
            Rc::new(inst4),
            Rc::new(inst5),
            Rc::new(inst6),
            Rc::new(inst7),
        ];

        // make a calculation configuration
        let calculation_configuration = CalculationConfiguration::default()
            .with_delta_calculation(true)
            .with_gamma_calculation(true)
            .with_theta_calculation(true)
            .with_rho_calculation(true)
            .with_vega_calculation(true)
            .with_vega_structure_calculation(true)
            .with_div_delta_calculation(true)
            .with_rho_structure_calculation(true)
            .with_div_structure_calculation(true)
            .with_vega_matrix_calculation(true)
            .with_theta_day(theta_day);

        // make a match parameter
        let mut collateral_curve_map = HashMap::new();
        collateral_curve_map.insert(String::from("KOSPI2"), String::from("KSD"));

        let mut borrowing_curve_map = HashMap::new();
        borrowing_curve_map.insert(String::from("KOSPI2"), String::from("KOSPI2"));

        let mut bond_discount_curve_map = HashMap::new();
        bond_discount_curve_map.insert(
            (
                issuer_name.to_string(),
                issuer_type,
                credit_rating,
                bond_currency,
            ),
            "KRWGOV".to_string(),
        );

        let rate_index_curve_map = HashMap::new();

        let mut crs_curve_map = HashMap::new();
        crs_curve_map.insert(Currency::KRW, "KRWCRS".to_string());
        crs_curve_map.insert(Currency::USD, "USDOIS".to_string());

        let mut funding_cost_map = HashMap::new();
        funding_cost_map.insert(Currency::KRW, funding_curve1.clone());

        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            crs_curve_map,
            rate_index_curve_map,
            funding_cost_map,
        );

        let category1 = InstrumentCategory::new(
            Some(vec![
                "Futures".to_string(),
                "VanillaCall".to_string(),
                "VanillaPut".to_string(),
                "IRS".to_string(),
                "CRS".to_string(),
                "FxFutures".to_string(),
            ]),
            Some(vec![Currency::KRW]),
            Some(vec!["KOSPI2".to_string()]),
        );

        let category2 = InstrumentCategory::new(
            Some(vec![
                "Bond".to_string(),
                "Cash".to_string(),
                "Stock".to_string(),
            ]),
            Some(vec![Currency::KRW, Currency::USD]),
            Some(vec!["KOSPI2".to_string()]),
        );

        let instrument_categories = vec![category1, category2];

        let mut engine_builder = EngineGenerator::builder();
        let engine_generator = engine_builder
            .with_configuration(calculation_configuration, dt.clone(), match_parameter)?
            .with_instruments(Instruments::new(inst_vec))?
            .with_instrument_categories(instrument_categories)?
            .with_data(
                fx_data_map,
                stock_data_map,
                zero_curve_map,
                dividend_data_map,
                equity_vol_map,
                equity_surface_map,
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
            )?;

        engine_generator
            .distribute_instruments()
            .context("Failed to distribute instruments")?;
        engine_generator
            .calculate()
            .context("Failed to calculate")?;

        let calculation_results: &HashMap<String, CalculationResult> =
            engine_generator.get_calculation_results();

        let key_npv = vec![
            ("KRxxxxxxxxxx".to_string(), 0.99930),
            ("165XXX1".to_string(), 349.466208),
            ("165XXX2".to_string(), 356.310592),
            ("165XXX3".to_string(), 1.3148708),
            ("USD Cash".to_string(), 1.0),
            ("KOSPI2".to_string(), 350.0),
            ("KR103501GCC0".to_string(), 1.0254111),
        ];

        for (key, npv) in key_npv.iter() {
            let result = calculation_results
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("No result found for key {}", key))?;
            let npv_comp = result
                .get_npv_result()
                .ok_or_else(|| anyhow::anyhow!("No npv result found for key {}", key))?
                .get_npv();

            assert!(
                (npv - npv_comp).abs() < 1e-6,
                "npv comparison failed for key {}: expected {}, got {}",
                key,
                npv,
                npv_comp,
            );
        }

        let elapsed = start_time.elapsed();
        info!("engine test finished {:?}", elapsed);

        Ok(())
    }
}
