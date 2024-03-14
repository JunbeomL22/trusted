use quantlib::assets::currency::Currency;
use quantlib::definitions::THETA_PNL_UNIT;
use quantlib::instruments::stock_futures::StockFutures;
use quantlib::instrument::Instrument;
use quantlib::definitions::Real;
use time::{macros::datetime, Duration};
use ndarray::array;
use ndarray::Array1;
use std::rc::Rc;
use quantlib::evaluation_date::EvaluationDate;
use quantlib::pricing_engines::calculation_configuration::CalculationConfiguration;
use quantlib::pricing_engines::match_parameter::MatchParameter;
use std::collections::HashMap;
use quantlib::pricing_engines::engine::Engine;
use quantlib::data::value_data::ValueData;
use quantlib::data::vector_data::VectorData;
use serde_json;
use quantlib::instruments::fixed_coupon_bond::FixedCouponBond;
use quantlib::enums::{IssuerType, CreditRating, RankType};
use quantlib::time::calendars::{southkorea::SouthKorea, southkorea::SouthKoreaType};
use quantlib::time::calendar::{Calendar, NullCalendarWrapper, SouthKoreaWrapper, UnitedStatesWrapper};
use quantlib::time::jointcalendar::JointCalendar;
use quantlib::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let theta_day = 30;
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
    let zero_curve1 = "KSD".to_string();
    let zero_curve_data1 = VectorData::new(
        value.clone(), 
        Some(dates.clone()), 
        times.clone(), 
        market_datetime, 
        Currency::KRW,
        zero_curve1.clone(),
    ).expect("Failed to create VectorData for KSD");

    let zero_curve2 = "KRWGOV".to_string();
    let zero_curve_data2 = VectorData::new(
        value + 0.0005, 
        Some(dates), 
        times, 
        market_datetime, 
        Currency::KRW,
        zero_curve2.clone(),
    ).expect("Failed to create VectorData for KRWGOV");

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
    zero_curve_map.insert(zero_curve1, zero_curve_data1);
    zero_curve_map.insert(zero_curve2, zero_curve_data2);
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

    let issuedate = datetime!(2020-01-01 16:30:00 +09:00);
    let maturity = issuedate + Duration::days(365 * 4);
    let issuer_name = "Korea Gov";
    let bond_name = "KRW Fixed Coupon Bond";
    let bond_code = "KR1234567890";
    let sk = SouthKoreaWrapper{c: SouthKorea::new(SouthKoreaType::Settlement)};
    let calendar = JointCalendar::new(vec![Calendar::SouthKorea(sk)]);

    let bond_currency = Currency::KRW;
    let issuer_type = IssuerType::Government;
    let credit_rating = CreditRating::None;

    let bond = FixedCouponBond::new_from_conventions(
        bond_currency,
        issuer_type,
        credit_rating,     
        RankType::Senior, 
        false, 
        0.03, 
        10_000.0, 
        issuedate.clone(), 
        issuedate.clone(),
        maturity,
        None, 
        DayCountConvention::ActActIsda, 
        BusinessDayConvention::Unadjusted, 
        PaymentFrequency::SemiAnnually, 
        issuer_name.to_string(), 
        0, 
        calendar, 
        bond_name.to_string(), 
        bond_code.to_string(),
    )?;

    let inst1 = Instrument::StockFutures(Box::new(stock_futures1));
    let inst2 = Instrument::StockFutures(Box::new(stock_futures2));
    let inst3: Instrument = Instrument::FixedCouponBond(Box::new(bond));

    let inst_vec = vec![Rc::new(inst1), Rc::new(inst2), Rc::new(inst3)];

    // make a calculation configuration
    let calculation_configuration = CalculationConfiguration::default()
        .with_delta_calculation(true)
        .with_gamma_calculation(true)
        .with_rho_calculation(true)
        .with_div_delta_calculation(true)
        .with_rho_structure_calculation(true)
        .with_theta_calculation(true)
        .with_div_structure_calculation(true)
        .with_theta_day(theta_day);
        
    // make a match parameter
    let mut collateral_curve_map = HashMap::new();
    collateral_curve_map.insert(String::from("KOSPI2"), String::from("KSD"));

    let mut borrowing_curve_map = HashMap::new();
    borrowing_curve_map.insert(String::from("KOSPI2"), String::from("KOSPI2"));

    let mut bond_discount_curve_map = HashMap::new();
    bond_discount_curve_map.insert(
        (issuer_name.to_string(), issuer_type, credit_rating, bond_currency), "KRWGOV".to_string()
    );

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
        //
        match_parameter,
    ).expect("Failed to create an engine");

    engine.initialize(inst_vec)?;
    engine.calculate().context("Failed to calculate")?;

    let result1 = engine.get_calculation_result().get(&String::from("165XXX1")).unwrap();
    let result2 = engine.get_calculation_result().get(&String::from("165XXX2")).unwrap();
    let result3 = engine.get_calculation_result().get(&String::from(bond_code)).unwrap();
    // display div-delta of RefCell<CalculationResult>
    
    println!("\n165XXX1");
    println!("result1 delta: {:?}", result1.borrow().get_delta());
    println!("result1 gamma: {:?}", result1.borrow().get_gamma());
    println!("result1 theta: {:?}", result1.borrow().get_theta());
    println!("result1 rho: {:?}", result1.borrow().get_rho());
    println!("result1 rho-structure: {:?}", result1.borrow().get_rho_structure());
    println!("result1 div-delta: {:?}", result1.borrow().get_div_delta());
    println!("result1 div-structure: {:?}", result1.borrow().get_div_structure());
    println!("\n165XXX2");
    println!("result2 delta: {:?}", result2.borrow().get_delta());
    println!("result2 gamma: {:?}", result2.borrow().get_gamma());
    println!("result2 theta: {:?}", result2.borrow().get_theta());
    println!("result2 rho: {:?}", result2.borrow().get_rho());
    println!("result2 rho-structure: {:?}", result2.borrow().get_rho_structure());
    println!("result2 div-delta: {:?}", result2.borrow().get_div_delta());
    println!("result2 div-structure: {:?}", result2.borrow().get_div_structure());
    println!("\nKR1234567890");
    println!("result3 delta: {:?}", result3.borrow().get_delta());
    println!("result3 gamma: {:?}", result3.borrow().get_gamma());
    println!("result3 theta: {:?}", result3.borrow().get_theta());
    println!("result3 rho: {:?}", result3.borrow().get_rho());
    println!("result3 rho-structure: {:?}", result3.borrow().get_rho_structure());
    println!("result3 div-delta: {:?}", result3.borrow().get_div_delta());
    println!("result3 div-structure: {:?}", result3.borrow().get_div_structure());
    //println!("\n\n{:?}", result1);
    // println!("result1:\n{}", serde_json::to_string_pretty(&result1).unwrap());
    // println!("result2:\n{}", serde_json::to_string_pretty(&result2).unwrap());
    Ok(())
}