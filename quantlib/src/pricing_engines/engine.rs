use tracing::{info, Level, span};
use crate::instruments::instrument_info::InstrumentInfo;
use crate::parameters::volatilities::local_volatility_surface::LocalVolatilitySurface;
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend,
    zero_curve::ZeroCurve,
    quanto::Quanto,
    volatility::Volatility,
    volatilities::constant_volatility::ConstantVolatility,
    market_price::MarketPrice,
    past_price::DailyClosePrice,
};
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, Instruments, InstrumentTrait};
use crate::definitions::{
    Real, Time, 
    DELTA_PNL_UNIT, VEGA_PNL_UNIT, DIV_PNL_UNIT, RHO_PNL_UNIT, THETA_PNL_UNIT,
};
use crate::currency::{Currency, FxCode};

use crate::data::{
    vector_data::VectorData,
    value_data::ValueData,
    surface_data::SurfaceData,
    daily_value_data::DailyValueData,
};
use crate::util::format_duration;
use crate::utils::string_arithmetic::add_period;
use crate::pricing_engines::{
    pricer::{Pricer, PricerTrait},
    match_parameter::MatchParameter,
    calculation_result::CalculationResult,
    calculation_configuration::CalculationConfiguration,
    npv_result::NpvResult,
    pricer_factory::PricerFactory,
};
use crate::time::{
    calendar_trait::CalendarTrait,
    calendars::nullcalendar::NullCalendar,
};

use std::{
    time::Instant,
    collections::{HashMap, HashSet},
    rc::Rc,
    cell::RefCell,
};
use std::sync::Arc;
use anyhow::{Result, Context, anyhow, bail};
use ndarray::Array2;
use time::{OffsetDateTime, Duration};
/// Engine typically handles a bunch of instruments and calculate the pricing of the instruments.
/// Therefore, the result of calculations is a hashmap with the key being the code of the instrument
/// Engine is a struct that holds the calculation results of the instruments
pub struct Engine {
    engine_id: u64,
    err_tag: String,
    //
    calculation_results: HashMap<String, RefCell<CalculationResult>>,
    calculation_configuration: CalculationConfiguration, // this should be cloned
    //
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fxs: HashMap<FxCode, Rc<RefCell<MarketPrice>>>,
    equities: HashMap<String, Rc<RefCell<MarketPrice>>>,
    zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
    dividends: HashMap<String, Option<Rc<RefCell<DiscreteRatioDividend>>>>,
    volatilities: HashMap<String, Rc<RefCell<Volatility>>>,
    quantos: HashMap<(String, FxCode), Rc<RefCell<Quanto>>>,
    past_daily_close_prices: HashMap<String, Rc<DailyClosePrice>>,
    // instruments
    instruments: Instruments, // all instruments
    pricers: HashMap<String, Pricer>, // pricers for each instrument
    // selected instuments for calculation,
    // e.g., if we calcualte a delta of a single stock, we do not need calculate all instruments
    instruments_in_action: Vec<Rc<Instrument>>, 
    match_parameter: Rc<MatchParameter>, // this must be cloned 
}

impl Engine {
    pub fn builder(
        engine_id: u64,
        calculation_configuration: CalculationConfiguration,
        evaluation_date: EvaluationDate,
        match_parameter: MatchParameter,
    ) -> Engine {
        Engine {
            engine_id,
            err_tag: "".to_string(),
            calculation_results: HashMap::new(),
            calculation_configuration,
            evaluation_date: Rc::new(RefCell::new(evaluation_date)),
            fxs: HashMap::new(),
            equities: HashMap::new(),
            zero_curves: HashMap::new(),
            dividends: HashMap::new(),
            volatilities: HashMap::new(),
            quantos: HashMap::new(),
            past_daily_close_prices: HashMap::new(),
            instruments: Instruments::default(),
            instruments_in_action: vec![],
            pricers: HashMap::new(),
            match_parameter: Rc::new(match_parameter),
        }
    }

    pub fn with_parameter_data(
        mut self,
        fx_data: Arc<HashMap<FxCode, ValueData>>,
        stock_data: Arc<HashMap<String, ValueData>>,
        curve_data: Arc<HashMap<String, VectorData>>,
        dividend_data: Arc<HashMap<String, VectorData>>,
        equity_constant_volatility_data: Arc<HashMap<String, ValueData>>,
        equity_volatility_surface_data: Arc<HashMap<String, SurfaceData>>,
        fx_constant_volatility_data: Arc<HashMap<FxCode, ValueData>>,
        quanto_correlation_data: Arc<HashMap<(String, FxCode), ValueData>>,
        past_daily_value_data: Arc<HashMap<String, DailyValueData>>,
    ) -> Result<Engine> {
        //
        let start_time = Instant::now();
        let engine_span = span!(Level::INFO, "Engine::with_parameter_data\n");
        let _enter = engine_span.enter();
        
        let fx_codes = self.instruments.get_all_fxcodes_for_pricing();
        let mut fxs: HashMap<FxCode, Rc<RefCell<MarketPrice>>> = HashMap::new();
        for fx_code in fx_codes {
            if fx_data.contains_key(&fx_code) {
                let data = fx_data.get(&fx_code).unwrap();
                let rc = Rc::new(RefCell::new(MarketPrice::new(
                    data.get_value(),
                    data.get_market_datetime().unwrap_or(
                        self.evaluation_date.borrow().get_date_clone()),
                    None,
                    fx_code.get_currency2().clone(),
                    fx_code.to_string(),
                    fx_code.to_string(),
                )));
                fxs.insert(fx_code, rc);
            } else if fx_data.contains_key(&fx_code.reciprocal()) {
                let data = fx_data.get(&fx_code.reciprocal()).unwrap();
                let rc = Rc::new(RefCell::new(MarketPrice::new(
                    1.0 / data.get_value(),
                    data.get_market_datetime().unwrap_or(
                        self.evaluation_date.borrow().get_date_clone()),
                    None,
                    fx_code.get_currency2().clone(),
                    fx_code.to_string(),
                    fx_code.to_string(),
                )));
                fxs.insert(fx_code.clone(), rc);
            } else if fx_data.contains_key(&FxCode::new(fx_code.get_currency1().clone(), Currency::KRW)) 
                && fx_data.contains_key(&FxCode::new(fx_code.get_currency2().clone(), Currency::KRW)) {
                let data1 = fx_data.get(&FxCode::new(fx_code.get_currency1().clone(), Currency::KRW)).unwrap();
                let data2 = fx_data.get(&FxCode::new(fx_code.get_currency2().clone(), Currency::KRW)).unwrap();

                let rc = Rc::new(RefCell::new(
                    MarketPrice::new(
                        data1.get_value() / data2.get_value(),
                        data1.get_market_datetime().unwrap_or(
                            self.evaluation_date.borrow().get_date_clone()),
                        None,
                        fx_code.get_currency2().clone(),
                        fx_code.to_string(),
                        fx_code.to_string(),
                    )));
                fxs.insert(fx_code.clone(), rc);
            } else {
                bail!(
                    "({}:{}) failed to get fx data for {}.\n\
                     fx_data must have either of itself, reciprocal,\n\
                    or both Curr1/KRW and Currr2/KRW",
                    file!(), line!(), fx_code
                );
            }
        }
        // curve data
        let mut zero_curves = HashMap::new();
        let all_curve_names = self.instruments.get_all_curve_names(&self.match_parameter)?;
        for curve_name in all_curve_names {
            if curve_data.contains_key(curve_name) {
                let data = curve_data.get(curve_name).unwrap();
                let zero_curve = Rc::new(RefCell::new(
                    ZeroCurve::new(
                        self.evaluation_date.clone(),
                        data,
                        curve_name.clone(),
                        curve_name.clone(),
                )?));
                zero_curves.insert(curve_name.clone(), zero_curve.clone());
            } else {
                bail!(
                    "({}:{}) failed to get curve data for {}",
                    file!(), line!(), curve_name
                );
            }
        }
        // dividend data
        let mut no_dividend_data_msg = format!(
            "missing dividend data list (engine-id: {})\n", self.engine_id);
        let mut dividends = HashMap::new();
        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        for underlying_code in all_underlying_codes {
            if dividend_data.contains_key(underlying_code) {
                let data = dividend_data.get(underlying_code).unwrap();
                let spot = stock_data.get(underlying_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get stock data for {}", 
                        file!(), line!(), underlying_code))?
                    .get_value();
                let dividend = Some(Rc::new(
                    RefCell::new(DiscreteRatioDividend::new(
                        self.evaluation_date.clone(),
                        data,
                        spot,
                        underlying_code.clone(),
                    ).with_context(|| anyhow!(
                        "({}:{}) failed to create discrete ratio dividend for {}", 
                        file!(), line!(), underlying_code))?)));
                dividends.insert(underlying_code.clone(), dividend.clone());
            } else {
                no_dividend_data_msg.push_str(&format!("{}\n", underlying_code));
            }
        }
        if !no_dividend_data_msg.is_empty() { info!("{}\n", no_dividend_data_msg); }
        //
        // borrowing curve parameter
        //
        // equity parameters
        let mut equities = HashMap::new();
        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        for underlying_code in all_underlying_codes {
            if stock_data.contains_key(underlying_code) {
                let data = stock_data.get(underlying_code).unwrap();
                let div = match dividends.get(underlying_code) {
                    Some(div) => div.clone(),
                    None => None,
                };
                let rc = Rc::new(RefCell::new(
                    MarketPrice::new(
                        data.get_value(),
                        data.get_market_datetime().unwrap_or(
                            self.evaluation_date.borrow().get_date_clone()),
                            div,
                            data.get_currency().clone(),
                            data.get_name().clone(),
                            underlying_code.clone(),
                    )));
                equities.insert(underlying_code.clone(), rc);
            } else {
                bail!(
                    "({}:{}) failed to get stock data for {}", 
                    file!(), line!(), underlying_code
                );
            }
        }
        //
        // equity volatility parameter
        let mut volatilities = HashMap::new();
        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        for und_code in all_underlying_codes {
            if equity_constant_volatility_data.contains_key(und_code) {
                let data = equity_constant_volatility_data.get(und_code).unwrap();
                let vega_matrix_spot_moneyness = self.calculation_configuration.get_vega_matrix_spot_moneyness();
                let vega_structure_tenors = self.calculation_configuration.get_vega_structure_tenors();
                let market_price = equities.get(und_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get market price for {}", 
                        file!(), line!(), und_code))?.clone();
                let collateral_curve_map = self.match_parameter.get_collateral_curve_map()
                    .get(und_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get collateral curve map for {} from match_parameter in creating volatility surface",
                        file!(), line!(), und_code))?;
                let collateral_curve = zero_curves.get(collateral_curve_map)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get collateral curve for {} in creating volatility surface", 
                        file!(), line!(), und_code))?.clone();
                let borrowing_curve_map = self.match_parameter.get_borrowing_curve_map()
                    .get(und_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get borrowing curve map for {} from match_parameter in creating volatility surface",
                        file!(), line!(), und_code))?;
                let borrowing_curve = zero_curves.get(borrowing_curve_map)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get borrowing curve for {} in creating volatility surface\n\
                        zero curves list:\n {:?}",
                        file!(), line!(), und_code,
                        zero_curves.keys().into_iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(" | "),
                    ))?.clone();
                let stickyness = self.calculation_configuration.get_stickyness_type();
                let lv_interpolator = self.calculation_configuration.get_lv_interpolator();
                let mut lv = LocalVolatilitySurface::initialize(
                    self.evaluation_date.clone(),
                    market_price,
                    collateral_curve,
                    borrowing_curve,
                    stickyness,
                    lv_interpolator,
                    und_code.clone(),
                    und_code.clone(),
                ).with_constant_volatility(
                    data,
                    vega_structure_tenors.clone(),
                    vega_matrix_spot_moneyness.clone(),
                )?;
                lv.build()?;
                let rc = Rc::new(RefCell::new(
                    Volatility::LocalVolatilitySurface(lv)
                ));
                volatilities.insert(und_code.clone(), rc);
            } else if equity_volatility_surface_data.contains_key(und_code) {
                let data = equity_volatility_surface_data.get(und_code).unwrap();
                let vega_matrix_spot_moneyness = self.calculation_configuration.get_vega_matrix_spot_moneyness();
                let vega_structure_tenors = self.calculation_configuration.get_vega_structure_tenors();
                let market_price = equities.get(und_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get market price for {}", 
                        file!(), line!(), und_code))?.clone();
                let collateral_curve_map = self.match_parameter.get_collateral_curve_map()
                    .get(und_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get collateral curve map for {} from match_parameter in creating volatility surface",
                        file!(), line!(), und_code))?;
                let collateral_curve = zero_curves.get(collateral_curve_map)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get collateral curve for {} in creating volatility surface", 
                        file!(), line!(), und_code))?.clone();
                let borrowing_curve_map = self.match_parameter.get_borrowing_curve_map()
                    .get(und_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get borrowing curve map for {} from match_parameter in creating volatility surface",
                        file!(), line!(), und_code))?;
                let borrowing_curve = zero_curves.get(borrowing_curve_map)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get borrowing curve for {} in creating volatility surface\n\
                        zero curves list:\n {:?}",
                        file!(), line!(), und_code,
                        zero_curves.keys().into_iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(" | "), 
                    ))?.clone();
                let stickyness = self.calculation_configuration.get_stickyness_type();
                let lv_interpolator = self.calculation_configuration.get_lv_interpolator();
                let mut lv = LocalVolatilitySurface::initialize(
                    self.evaluation_date.clone(),
                    market_price,
                    collateral_curve,
                    borrowing_curve,
                    stickyness,
                    lv_interpolator,
                    und_code.clone(),
                    und_code.clone(),
                ).with_market_surface(
                    data,
                    vega_structure_tenors.clone(),
                    vega_matrix_spot_moneyness.clone(),
                )?;
                lv.build()?;
                let rc = Rc::new(RefCell::new(
                    Volatility::LocalVolatilitySurface(lv)
                ));
                volatilities.insert(und_code.clone(), rc);
            } else {
                bail!(
                    "({}:{}) failed to get equity volatility data for {}", 
                    file!(), line!(), und_code
                );
            }
        }
        // 
        // fx volatility parameter
        let quanto_fx_und_pair = self.instruments.get_all_quanto_fxcode_und_pairs();
        let unique_fxcodes: HashSet<&FxCode> = quanto_fx_und_pair.iter().map(|(_, second)| second.clone()).collect();

        let mut fx_volatilities = HashMap::new();
        for fx_code in unique_fxcodes {
            if fx_constant_volatility_data.contains_key(&fx_code) {
                let data = fx_constant_volatility_data.get(&fx_code).unwrap();
                let rc = Rc::new(RefCell::new(
                    Volatility::ConstantVolatility(ConstantVolatility::new(
                        data.get_value(),
                        fx_code.to_string(),
                        fx_code.to_string(),
                    ))));
                fx_volatilities.insert(fx_code.clone(), rc);
            } else {
                bail!(
                    "({}:{}) failed to get fx volatility data for {}", 
                    file!(), line!(), fx_code
                );
            }
        }
        //
        // quanto parameter
        let mut quantos = HashMap::new();
        for (und_code, fxcode) in quanto_fx_und_pair {
            if quanto_correlation_data.contains_key(&(und_code.clone(), fxcode.clone())) {
                let data = quanto_correlation_data.get(&(und_code.clone(), fxcode.clone())).unwrap();
                let rc = Rc::new(RefCell::new(
                    Quanto::new(
                        fx_volatilities.get(fxcode)
                            .with_context(|| anyhow!(
                                "({}:{}) failed to get fx volatility for ({:?} {:?}) in creating quanto correlation parameter", 
                                file!(), line!(), und_code, fxcode))?
                            .clone(),
                        data.get_value(),
                        fxcode.clone(),
                        und_code.clone(),
                    )));
                quantos.insert((und_code.clone(), fxcode.clone()), rc);
            } else {
                bail!(
                    "({}:{}) failed to get quanto correlation data for {:?}", 
                    file!(), line!(), (und_code, fxcode)
                );
            }
        }
        // past price parameter
        let mut past_daily_close_prices = HashMap::new();
        for (key, data) in past_daily_value_data.iter() {
            let daily_close = DailyClosePrice::new_from_data(data)
                .with_context(|| anyhow!(
                    "({}:{}) failed to create daily close price from data for {}", 
                    file!(), line!(), key))?;
            let rc = Rc::new(daily_close);
            past_daily_close_prices.insert(key.clone(), rc);
        }
          
        self.fxs = fxs;
        self.equities = equities;
        self.zero_curves = zero_curves;
        self.dividends = dividends;
        self.volatilities = volatilities;
        self.quantos = quantos;
        self.past_daily_close_prices = past_daily_close_prices;

        let elapsed = start_time.elapsed();
        info!(
            "(id: {}) Engine::with_parameter_data elapsed time: {:?}", 
            self.engine_id,
            elapsed,
        );
        Ok(self)
    }
    // initialize CalculationResult for each instrument
    pub fn with_instruments(
        mut self, 
        instrument_vec: Vec<Instrument>,
    ) -> Result<Engine> {
        if instrument_vec.is_empty() {
            return Err(
                anyhow!("({}:{}) no instruments are given to initialize", file!(), line!())
            );
        }
        let vec_rc_inst: Vec<Rc<Instrument>> = instrument_vec.into_iter().map(Rc::new).collect();
        self.instruments = Instruments::new(vec_rc_inst);
        let all_types = self.instruments.get_all_type_names();
        let curr_str: Vec<&str> = self.instruments.get_all_currencies()?.iter().map(|c| c.as_str()).collect();
        let all_und_codes: Vec<&str> = self.instruments.get_all_underlying_codes().iter().map(|c| c.as_str()).collect();
        self.err_tag = format!(
            "<TAG>\n\
            engine-id: {}\n\
            instrument-types: {}\n\
            currencies: {}\n\
            underlying-codes: {}\n",
            self.engine_id,
            all_types.join(" / "),
            curr_str.join(" / "),
            all_und_codes.join(" / "),
        );

        let dt = self.evaluation_date.borrow().get_date_clone();
        let insts_over_maturity = self.instruments.instruments_with_maturity_upto(None, &dt, None);

        if !insts_over_maturity.is_empty() {
            let mut inst_codes = Vec::<String>::new();
            let mut inst_mat = Vec::<Option<OffsetDateTime>>::new();
            for inst in insts_over_maturity {
                inst_codes.push(inst.get_code().clone());
                let mat = inst.get_maturity();
                match mat {
                    Some(m) => inst_mat.push(Some(m.clone())),
                    None => inst_mat.push(None),
                }
            }

            let display = inst_codes.iter().zip(inst_mat.iter())
                .map(|(code, mat)| {
                    match mat {
                        Some(m) => format!("{}: {:}", code, m),
                        None => format!("{}: None", code),
                    }
                }).collect::<Vec<String>>().join("\n");
            bail!(
                "(Engine::initialize_instruments) There are instruments with maturity within the evaluation date\n\
                evaluation date: {:?}\n\
                {}\n",
                dt, display
            );
        }

        let insts_with_very_short_maturity = self.instruments.instruments_with_maturity_upto(
            None, &(dt + Duration::hours(6)),
            None,
        );

        if !insts_with_very_short_maturity.is_empty() {
            let mut inst_codes = Vec::<String>::new();
            let mut inst_mat = Vec::<Option<OffsetDateTime>>::new();
            for inst in insts_with_very_short_maturity {
                inst_codes.push(inst.get_code().clone());
                let mat = inst.get_maturity();
                match mat {
                    Some(m) => inst_mat.push(Some(m.clone())),
                    None => inst_mat.push(None),
                }
            }

            let display = inst_codes.iter().zip(inst_mat.iter())
                .map(|(code, mat)| {
                    match mat {
                        Some(m) => format!("{}: {:}", code, m),
                        None => format!("{}: None", code),
                    }
                }).collect::<Vec<String>>().join("\n");
            
            println!(
                "\n(Engine::initialize_instruments) There are instruments with a very short maturity (within 6 hours) \n\
                Note that these products may produce numerical errors.
                <LIST>\n{}\n{}\n",
                display,
                self.err_tag
            );
        }

        self.instruments_in_action = self.instruments
            .get_instruments_clone();
    
        for inst in self.instruments.iter() {
            let code = inst.get_code();
            let inst_type = inst.get_type_name();
            let instrument_information = InstrumentInfo::new(
                inst.get_name().to_string(),
                code.to_string(),
                inst_type,
                inst.get_currency().clone(),
                inst.get_unit_notional(),
                inst.get_maturity().clone(),
            );

            let init_res = CalculationResult::new(
                instrument_information,
                self.evaluation_date.borrow().get_date_clone(),
            );

            self.calculation_results.insert(
                inst.get_code().clone(),
                RefCell::new(init_res),
            );
        }
        Ok(self)
    }

    pub fn new (
        engine_id: u64,
        calculation_configuration: CalculationConfiguration,
        evaluation_date: EvaluationDate,
        match_parameter: MatchParameter,
        //
        fx_data: Arc<HashMap<FxCode, ValueData>>,
        stock_data: Arc<HashMap<String, ValueData>>,
        curve_data: Arc<HashMap<String, VectorData>>,
        dividend_data: Arc<HashMap<String, VectorData>>,
        equity_constant_volatility_data: Arc<HashMap<String, ValueData>>,
        equity_volatility_surface_data: Arc<HashMap<String, SurfaceData>>,
        fx_constant_volatility_data: Arc<HashMap<FxCode, ValueData>>,
        quanto_correlation_data: Arc<HashMap<(String, FxCode), ValueData>>,
        past_daily_value_data: Arc<HashMap<String, Rc<DailyValueData>>>,
        //
    ) -> Result<Engine> {
        let evaluation_date = Rc::new(RefCell::new(
            evaluation_date
        )); 

        let mut zero_curves = HashMap::new();
        //let mut curve_data_refcell = HashMap::new();
        for (key, data) in curve_data.as_ref().into_iter() {
            let zero_curve = Rc::new(RefCell::new(
                ZeroCurve::new(
                    evaluation_date.clone(),
                    &data,
                    key.clone(),
                    key.clone(),
                ).with_context(|| anyhow!(
                "({}:{}) failed to create zero curve {}", 
                file!(), line!(), key))?
            ));
            zero_curves.insert(key.to_string(), zero_curve.clone());

            /*
            let ref_cell = RefCell::new(data);
            ref_cell.borrow_mut().add_observer(zero_curve);
            curve_data_refcell.insert(key.to_string(), ref_cell);
             */
        }

        let mut dividends = HashMap::new();
        //let mut dividend_data_refcell = HashMap::new();
        for (key, data) in dividend_data.as_ref().into_iter() {
            let spot = stock_data.get(key)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get dividend to match stock data for {}", 
                    file!(), line!(), key))?
                .get_value();

            let dividend = Rc::new(RefCell::new(
                DiscreteRatioDividend::new(
                    evaluation_date.clone(),
                    &data,
                    spot,
                    key.clone(),
                ).with_context(|| anyhow!(
                    "failed to create discrete ratio dividend: {}", key))?
            ));

            dividends.insert(key.to_string(), Some(dividend.clone()));
            /* 
            let ref_cell = RefCell::new(data);
            ref_cell.borrow_mut().add_observer(dividend);
            dividend_data_refcell.insert(key.to_string(), ref_cell);
             */
        }
        let mut fxs: HashMap<FxCode, Rc<RefCell<MarketPrice>>> = HashMap::new();
        fx_data
            .as_ref()
            .iter()
            .for_each(|(key, data)| {
                let rc = Rc::new(RefCell::new(
                    MarketPrice::new(
                        data.get_value(),
                        data.get_market_datetime().unwrap_or(
                            evaluation_date.borrow().get_date_clone()),
                        None,
                        key.get_currency2().clone(),
                        key.to_string(),
                        key.to_string(),
                    )
                ));
                fxs.insert(key.clone(), rc);
            });
        
        let krwkrw_code = FxCode::new(Currency::KRW, Currency::KRW);
        if !fxs.contains_key(&krwkrw_code) {
            fxs.insert(
                krwkrw_code.clone(),
                Rc::new(RefCell::new(
                    MarketPrice::new(
                        1.0,
                        evaluation_date.borrow().get_date_clone(),
                        None,
                        Currency::KRW,
                        "KRWKRW".to_string(),
                        "KRWKRW".to_string(),
                    )
                ))
            );
        }

        let mut fx_volatilities = HashMap::new();
        for (fx_code, data) in fx_constant_volatility_data.iter() {
            let rc = Rc::new(RefCell::new(
                Volatility::ConstantVolatility(ConstantVolatility::new(
                    data.get_value(),
                    fx_code.to_string(),
                    fx_code.to_string(),
                )
            )));
            fx_volatilities.insert(fx_code.clone(), rc);
        }
        //quanto
        let mut quantos = HashMap::new();
        for ((equity_code, fx_code), data) in quanto_correlation_data.iter() {
            let rc = Rc::new(RefCell::new(
                Quanto::new(
                    fx_volatilities.get(fx_code)
                        .with_context(|| anyhow!(
                            "failed to get fx volatility for {:?}", fx_code))?
                        .clone(),
                    data.get_value(),
                    fx_code.clone(),
                    equity_code.clone(),
                )
            ));
            quantos.insert((equity_code.clone(), fx_code.clone()), rc);
        }
        // making stock Rc -> RefCell for pricing
        let mut equities = HashMap::new();
        for (key, data) in stock_data.iter() {
            let div = match dividends.get(key) {
                Some(div) => div.clone(),
                None => None,
            };

            let rc = Rc::new(RefCell::new(
                MarketPrice::new(
                    data.get_value(),
                    data.get_market_datetime().unwrap_or(
                        evaluation_date.borrow().get_date_clone()),
                    div,
                    data.get_currency().clone(),  
                    data.get_name().clone(),
                    key.to_string(),
                )
            ));
            equities.insert(key.clone(), rc);
            }
        
        let mut volatilities = HashMap::new();
        
        for (key, data) in equity_constant_volatility_data.iter() {
            let vega_matrix_spot_moneyness = calculation_configuration.get_vega_matrix_spot_moneyness();
            let vega_structure_tenors = calculation_configuration.get_vega_structure_tenors();
            let market_price = equities.get(key)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get market price for {}", 
                    file!(), line!(), key))?.clone();
            let collateral_curve_map = match_parameter.get_collateral_curve_map()
                    .get(key)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get collateral curve map for {}", 
                        file!(), line!(), key))?;

            let collateral_curve = zero_curves.get(collateral_curve_map)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get collateral curve for {}", 
                    file!(), line!(), key))?.clone();

            let borrowing_curve_map = match_parameter.get_borrowing_curve_map()
                .get(key)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get borrowing curve map for {}", 
                    file!(), line!(), key))?;
            let borrowing_curve = zero_curves.get(borrowing_curve_map)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get borrowing curve for {}", 
                    file!(), line!(), key))?.clone();
                
            let stickyness = calculation_configuration.get_stickyness_type();
            let lv_interpolator = calculation_configuration.get_lv_interpolator();

            let mut lv = LocalVolatilitySurface::initialize(
                evaluation_date.clone(),
                market_price,
                collateral_curve,
                borrowing_curve,
                stickyness,
                lv_interpolator,
                key.clone(),
                key.clone(),
            ).with_constant_volatility(
                &data,
                vega_structure_tenors.clone(),
                vega_matrix_spot_moneyness.clone(),
            )?;

            lv.build()?;

            let rc = Rc::new(RefCell::new(
                Volatility::LocalVolatilitySurface(lv)
            ));

            volatilities.insert(key.clone(), rc);
        }
        
        for (key, data) in equity_volatility_surface_data.iter() {
            let vega_matrix_spot_moneyness = calculation_configuration.get_vega_matrix_spot_moneyness();
            let vega_structure_tenors = calculation_configuration.get_vega_structure_tenors();
            let market_price = equities.get(key)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get market price for {}", 
                    file!(), line!(), key))?.clone();
            let collateral_curve_map = match_parameter.get_collateral_curve_map()
                    .get(key)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get collateral curve map for {}", 
                        file!(), line!(), key))?;

            let collateral_curve = zero_curves.get(collateral_curve_map)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get collateral curve for {}", 
                    file!(), line!(), key))?.clone();

            let borrowing_curve_map = match_parameter.get_borrowing_curve_map()
                .get(key)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get borrowing curve map for {}", 
                    file!(), line!(), key))?;
            let borrowing_curve = zero_curves.get(borrowing_curve_map)
                .with_context(|| anyhow!(
                    "({}:{}) failed to get borrowing curve for {}", 
                    file!(), line!(), key))?.clone();
                
            let stickyness = calculation_configuration.get_stickyness_type();
            let lv_interpolator = calculation_configuration.get_lv_interpolator();

            let mut lv = LocalVolatilitySurface::initialize(
                evaluation_date.clone(),
                market_price,
                collateral_curve,
                borrowing_curve,
                stickyness,
                lv_interpolator,
                key.clone(),
                key.clone(),
            ).with_market_surface(
                &data,
                vega_structure_tenors.clone(),
                vega_matrix_spot_moneyness.clone(),
            )?;

            lv.build()?;

            let rc = Rc::new(RefCell::new(
                Volatility::LocalVolatilitySurface(lv)
            ));

            volatilities.insert(key.clone(), rc);
        }

        let mut past_daily_close_prices = HashMap::new();
        for (key, data) in past_daily_value_data.iter() {
            let inner_data = data.as_ref();
            let daily_close_price = DailyClosePrice::new_from_data(inner_data)
                .with_context(|| anyhow!(
                    "({}:{}) failed to create daily close price from data for {}", 
                    key, file!(), line!()))?;
            past_daily_close_prices.insert(key.clone(), Rc::new(daily_close_price));
        }

        Ok(Engine {
            engine_id: engine_id,
            err_tag : "".to_string(),
            calculation_results: HashMap::new(),
            calculation_configuration,
            //
            evaluation_date,
            fxs,
            equities,
            zero_curves,
            dividends,
            volatilities,
            quantos,
            past_daily_close_prices,
            //
            instruments: Instruments::default(),
            instruments_in_action: vec![],
            pricers: HashMap::new(),
            match_parameter: Rc::new(match_parameter),
        })
    }

    pub fn initialize(&mut self, instrument_vec: Vec<Rc<Instrument>>) -> Result<()> {
        self.initialize_instruments(instrument_vec)?;
        self.initialize_pricers()?;
        Ok(())
    }

    pub fn initialize_instruments(
        &mut self, 
        instrument_vec: Vec<Rc<Instrument>>,
    ) -> Result<()> {
        if instrument_vec.is_empty() {
            return Err(
                anyhow!("({}:{}) no instruments are given to initialize", file!(), line!())
            );
        }
        self.instruments = Instruments::new(instrument_vec);
        let all_types = self.instruments.get_all_type_names();
        let curr_str: Vec<&str> = self.instruments.get_all_currencies()?.iter().map(|c| c.as_str()).collect();
        let all_und_codes: Vec<&str> = self.instruments.get_all_underlying_codes().iter().map(|c| c.as_str()).collect();
        self.err_tag = format!(
            "<TAG>\n\
            engine-id: {}\n\
            instrument-types: {}\n\
            currencies: {}\n\
            underlying-codes: {}\n",
            self.engine_id,
            all_types.join(" / "),
            curr_str.join(" / "),
            all_und_codes.join(" / "),
        );

        let dt = self.evaluation_date.borrow().get_date_clone();
        let insts_over_maturity = self.instruments.instruments_with_maturity_upto(None, &dt, None);

        if !insts_over_maturity.is_empty() {
            let mut inst_codes = Vec::<String>::new();
            let mut inst_mat = Vec::<Option<OffsetDateTime>>::new();
            for inst in insts_over_maturity {
                inst_codes.push(inst.get_code().clone());
                let mat = inst.get_maturity();
                match mat {
                    Some(m) => inst_mat.push(Some(m.clone())),
                    None => inst_mat.push(None),
                }
            }

            let display = inst_codes.iter().zip(inst_mat.iter())
                .map(|(code, mat)| {
                    match mat {
                        Some(m) => format!("{}: {:}", code, m),
                        None => format!("{}: None", code),
                    }
                }).collect::<Vec<String>>().join("\n");
            bail!(
                "(Engine::initialize_instruments) There are instruments with maturity within the evaluation date\n\
                evaluation date: {:?}\n\
                {}\n",
                dt, display
            );
        }

        let insts_with_very_short_maturity = self.instruments.instruments_with_maturity_upto(
            None, &(dt + Duration::hours(6)),
            None,
        );

        if !insts_with_very_short_maturity.is_empty() {
            let mut inst_codes = Vec::<String>::new();
            let mut inst_mat = Vec::<Option<OffsetDateTime>>::new();
            for inst in insts_with_very_short_maturity {
                inst_codes.push(inst.get_code().clone());
                let mat = inst.get_maturity();
                match mat {
                    Some(m) => inst_mat.push(Some(m.clone())),
                    None => inst_mat.push(None),
                }
            }

            let display = inst_codes.iter().zip(inst_mat.iter())
                .map(|(code, mat)| {
                    match mat {
                        Some(m) => format!("{}: {:}", code, m),
                        None => format!("{}: None", code),
                    }
                }).collect::<Vec<String>>().join("\n");
            
            println!(
                "\n(Engine::initialize_instruments) There are instruments with a very short maturity (within 6 hours) \n\
                Note that these products may produce numerical errors.
                <LIST>\n{}\n{}\n",
                display,
                self.err_tag
            );
        }

        self.instruments_in_action = self.instruments
            .get_instruments_clone();
    
        for inst in self.instruments.iter() {
            let code = inst.get_code();
            let inst_type = inst.get_type_name();
            let instrument_information = InstrumentInfo::new(
                inst.get_name().to_string(),
                code.to_string(),
                inst_type,
                inst.get_currency().clone(),
                inst.get_unit_notional(),
                inst.get_maturity().clone(),
            );

            let init_res = CalculationResult::new(
                instrument_information,
                self.evaluation_date.borrow().get_date_clone(),
            );

            self.calculation_results.insert(
                inst.get_code().clone(),
                RefCell::new(init_res),
            );
        }
        Ok(())
    }

    pub fn initialize_pricers(&mut self) -> Result<()> {
        let inst_vec = self.instruments.get_instruments_clone();
        let pricer_factory = PricerFactory::new(
            self.evaluation_date.clone(),
            self.fxs.clone(),
            self.equities.clone(),
            self.zero_curves.clone(),
            //self.dividends.clone(),
            self.volatilities.clone(),
            self.quantos.clone(),
            self.past_daily_close_prices.clone(),
            self.match_parameter.clone(),
        );
        
        for inst in inst_vec.iter() {
            let pricer = pricer_factory.create_pricer(inst)
                .with_context(|| anyhow!(
                    "({}:{}) failed to create pricer for {} ({})\n{}",
                    file!(), line!(),
                    inst.get_code(),
                    inst.get_type_name(),
                    self.err_tag,
                ))?;
            self.pricers.insert(inst.get_code().clone(), pricer);
        }
        Ok(())
    }

    /// re-initialize instruments_in_action
    pub fn reset_instruments_in_action(&mut self) {
        self.instruments_in_action = self.instruments
            .get_instruments_clone();
    }

    pub fn get_npvs(&self) -> Result<HashMap<String, Real>> {
        let mut npvs = HashMap::new();
        for inst in &self.instruments_in_action {
            let inst_code = inst.get_code();
            let pricer = self.pricers.get(inst_code)
                .with_context(|| anyhow!(
                    "({}:{}) <Egnine::get_npvs> failed to get pricer for {}\n{}", 
                    file!(), line!(),
                    inst_code, self.err_tag))?;

            let npv = pricer
                .npv(inst)
                .with_context(|| anyhow!(
                    "({}:{}) <Egnine::get_npvs> failed to get npv for {}\n{}", 
                    file!(), line!(),
                    inst_code, self.err_tag
                ))?;
    
            npvs.insert(inst_code.clone(), npv);
        }
        Ok(npvs)
    }

    pub fn get_npv_results(&self) -> Result<HashMap<String, NpvResult>> {
        let mut npvs = HashMap::new();
        for inst in &self.instruments_in_action {
            let inst_code = inst.get_code();
            let pricer = self.pricers.get(inst_code)
                .with_context(|| anyhow!(
                    "({}:{}) <Engine::get_npv_results> failed to get pricer for {}\n{}",
                    file!(), line!(), 
                    inst_code,
                    self.err_tag,
                ))?;

            let npv = pricer.npv_result(inst)?;
            npvs.insert(inst.get_code().clone(), npv);
        }
        Ok(npvs)
    }

    pub fn set_npv_results(&mut self) -> Result<()> {
        let npvs = self.get_npv_results()?;
        
        for (code, result) in self.calculation_results.iter() {
            result.borrow_mut().set_npv(
                npvs.get(code)
                .ok_or_else(|| anyhow!("npv is not set for {}\n{}", code, self.err_tag,))?.clone()
            )
        }
        Ok(())
    }

    pub fn set_cashflow_inbetween(&mut self) -> Result<()> {
        for (code, result) in self.calculation_results.iter() {
            let npv_res = result.borrow().get_npv_result()
                .ok_or_else(|| anyhow!(
                    "npv_result is not set for {}\n{}", code, self.err_tag,
                ))?.clone();
                
            let cashflow = npv_res.get_expected_coupon_amount()
                .with_context(|| anyhow!(
                    "failed to get expected coupon amount for {}", code
                ))?;
            (*result).borrow_mut().set_cashflows(cashflow);
        }
        Ok(())
    }

    pub fn set_fx_exposures(&mut self) -> Result<()> {
        let mut fx_exposures = HashMap::new();
        for inst in &self.instruments_in_action {
            let inst_code = inst.get_code();
            let pricer = self.pricers.get(inst_code)
                .ok_or_else(|| anyhow!("failed to get pricer for {} in getting fx-exposure", inst_code))?;
            
            let npv = self.calculation_results.get(inst_code)
                .ok_or_else(|| anyhow!("failed to get npv for {} in getting fx-exposure", inst_code))?
                .borrow()
                .get_npv_result()
                .ok_or_else(|| anyhow!("npv is not set for {} in getting fx-exposure", inst_code))?
                .get_npv();

            let fx_exposure = pricer.fx_exposure(inst, npv)
                .context("failed to get fx exposure")?;
            
            fx_exposures.insert(inst.get_code(), fx_exposure);
        }
        
        for (code, result) in self.calculation_results.iter() {
            (*result).borrow_mut().set_fx_exposure(
                fx_exposures.get(code)
                .ok_or_else(|| anyhow!("fx exposure is not set"))?.clone()
            );
        }
        Ok(())
    }

    /// Set the value of the instruments which means npv * unit_notional
    pub fn set_values(&mut self) -> Result<()> {
        for (_code, result) in self.calculation_results.iter() {
            (*result).borrow_mut().set_value()?;
        }
        Ok(())
    }

    pub fn preprocess_delta_gamma(&mut self) -> Result<()> {
        let preprocess_types = vec!["Stock", "Futures"];
        let insts = self.instruments.instruments_with_types(preprocess_types);
        
        // set delta for values * DELTA_PNL_UNIT, gamma = 0.0
        for inst in insts {
            let inst_code = inst.get_code();
            let unitamt = inst.get_unit_notional();
            let value = self.calculation_results
                .get(inst_code)
                .ok_or_else(|| anyhow!(
                    "({}:{}) result is not set in {} ({})",
                    file!(), line!(), inst_code, inst.get_type_name(),
                ))?
                .borrow()
                .get_value()
                .ok_or_else(|| anyhow!(
                    "({}:{}) value is not set in {} ({})", 
                    file!(), line!(), inst_code, inst.get_type_name(),
                ))?;
            let delta = value * DELTA_PNL_UNIT;
            let gamma = 0.0;
            (*self.calculation_results
                .get(inst_code)
                .ok_or_else(|| anyhow!(
                    "({}:{}) result is not set in {} ({})",
                    file!(), line!(), inst_code, inst.get_type_name(),
                ))?)
                .borrow_mut()
                .set_single_delta(&inst_code, delta * unitamt);
            (*self.calculation_results
                .get(inst_code)
                .ok_or_else(|| anyhow!(
                    "({}:{}) result is not set in {} ({})",
                    file!(), line!(), inst_code, inst.get_type_name(),
                ))?)
                .borrow_mut()
                .set_single_gamma(&inst_code, gamma * unitamt);
        }

        Ok(())
    }
    
    pub fn set_delta_gamma(&mut self) -> Result<()> {
        self.reset_instruments_in_action();

        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        let delta_bump_ratio = self.calculation_configuration.get_delta_bump_ratio();

        let mut delta_up_map: HashMap::<String, Real>;
        let mut delta_down_map: HashMap::<String, Real>;
        
        let mut delta_up: Real;
        let mut delta_down: Real;
        let mut delta: Real;
        let mut gamma: Real;
        let mut mid: Real;
        let mut original_price: Real;

        let up_bump = 1.0 + delta_bump_ratio;
        let down_bump = 1.0 - delta_bump_ratio;
        let exclude_type = vec!["Stock", "Futures"];
        let exclude_type_clone = exclude_type.clone();
        for und_code in all_underlying_codes.iter() {
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(
                    und_code,
                    Some(exclude_type_clone.clone()),
                );
            
            if self.instruments_in_action.is_empty() {
                continue;
            }
            original_price = self.equities
                .get(*und_code)
                .ok_or_else(|| anyhow!("there is no stock {}", und_code))?
                .borrow()
                .get_value();

            // set instruments that needs to be calculated
            {
                let mut stock = (*self.equities
                    .get(*und_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) there is no stock {}", 
                        file!(), line!(),
                        und_code
                    ))?).as_ref().borrow_mut();

                *stock *= up_bump;
            }

            delta_up_map = self.get_npvs().context("failed to get npvs")?;
            {
                let mut stock = (*self.equities
                    .get(*und_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) there is no stock {}", 
                        file!(), line!(),
                        und_code))?).as_ref().borrow_mut();

                stock.set_price(original_price);
                *stock *= down_bump;
            }
            
            delta_down_map = self.get_npvs().context("failed to get npvs")?;
            
            for inst in &self.instruments_in_action {
                let inst_code = inst.get_code();
                let unitamt = inst.get_unit_notional();
                delta_up = delta_up_map
                    .get(inst_code)
                    .ok_or_else(|| anyhow!("delta_up is not set"))?
                    .clone();
                delta_down = delta_down_map
                    .get(inst_code)
                    .ok_or_else(|| anyhow!("delta_down is not set"))?
                    .clone();

                delta = (delta_up - delta_down) / (2.0 * delta_bump_ratio) * DELTA_PNL_UNIT;

                (*self.calculation_results
                    .get(inst_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) result is not set for {}",
                        file!(), line!(), inst_code,
                    ))?)
                    .borrow_mut()
                    .set_single_delta(&und_code, delta * unitamt);

                mid = self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow()
                    .get_npv_result()
                    .ok_or_else(|| anyhow!("npv is not set"))?
                    .get_npv();

                gamma = delta_up - mid + delta_down - mid;
                gamma *= DELTA_PNL_UNIT / delta_bump_ratio;
                gamma *= 0.5 * (DELTA_PNL_UNIT / delta_bump_ratio);

                (*self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!(
                        "({}:{}) result is not set for {}",
                        file!(), line!(), inst.get_code(),
                    ))?)
                    .borrow_mut()
                    .set_single_gamma(&und_code, gamma * unitamt);
            }

            {
                (*self.equities
                    .get(*und_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) there is no stock {}", 
                        file!(), line!(), und_code
                    ))?)
                    .as_ref()
                    .borrow_mut()
                    .set_price(original_price);
            }
        }
        Ok(())
    }

    pub fn set_rho(&mut self) -> Result<()> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_curve_names = self.instruments.get_all_curve_names(&self.match_parameter)?;
        let bump_val = self.calculation_configuration.get_rho_bump_value();
        let exclude_type = vec!["Stock"];
        let exclude_type_clone = exclude_type.clone();
        
        for curve_name in all_curve_names {
            self.instruments_in_action = self.instruments
                .instruments_using_curve(
                    curve_name, 
                    &self.match_parameter,
                    Some(exclude_type_clone.clone()),
                )?;
            if self.instruments_in_action.is_empty() {
                continue;
            }
            // bump the curve but limit the scope that the zero_curve ismutably borrowed
            {
                (*self.zero_curves.get(curve_name)
                    .with_context(|| anyhow!(
                        "({}:{}) no zero curve: {}\n{}", 
                        file!(), line!(),
                        curve_name, self.err_tag,))?)
                        .as_ref()
                        .borrow_mut()
                        .bump_time_interval(None, None, bump_val)?;
            }

            npvs_up = self.get_npvs().context("failed to get npvs")?;

            for inst in &self.instruments_in_action {
                let inst_code = inst.get_code();
                let unitamt = inst.get_unit_notional();
                let npv_up = npvs_up.get(inst_code)
                    .ok_or_else(|| anyhow!("npv_up is not set"))?;
                let npv = self.calculation_results
                    .get(inst_code)
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow()
                    .get_npv_result()
                    .ok_or_else(|| anyhow!("npv is not set"))?
                    .get_npv();

                let rho = (npv_up - npv) / bump_val * RHO_PNL_UNIT * unitamt;
                (*self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!(
                        "({}:{}) result is not set for {}",
                        file!(), line!(), inst.get_code(),
                    ))?)
                    .borrow_mut()
                    .set_single_rho(curve_name, rho);
            }
            // put back the bump value
            {
                (*self.zero_curves.get(curve_name)
                    .with_context(|| anyhow!(
                        "({}:{}) no zero curve: {}\n{}", 
                        file!(), line!(),
                        curve_name, self.err_tag,))?)
                    .as_ref()
                    .borrow_mut()
                    .bump_time_interval(None, None, -bump_val)?;
            }
        }
        Ok(())
    }

    pub fn set_vega(&mut self) -> Result<()> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        let bump_val = self.calculation_configuration.get_vega_bump_value();
        let mut npv: Real;
        let exclude_type = vec!["Futures", "Stock"];
        let exclude_type_clone = exclude_type.clone();
        for vol_code in all_underlying_codes {
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(
                    vol_code,
                    Some(exclude_type_clone.clone()),
                );
            
            if self.instruments_in_action.is_empty() {
                continue;
            }

            // bump the volatility but limit the scope that is mutably borrowed
            {
                (*self.volatilities
                    .get(vol_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) volatility {} is not set\ntag:\n{}", 
                        file!(), line!(),
                        vol_code, self.err_tag
                    ))?)
                    .as_ref()
                    .borrow_mut()
                    .bump_volatility(
                        None, 
                        None, 
                        None,
                        None,
                        bump_val)?;
            }

            npvs_up = self.get_npvs().context("failed to get npvs")?; // instrument code (String) -> npv (Real

            for inst in &self.instruments_in_action {
                let inst_code = inst.get_code();
                let unitamt = inst.get_unit_notional();
                let npv_up = npvs_up.get(inst_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) npv_up is not set for {}", file!(), line!(), inst_code))?;

                npv = self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!(
                        "({}:{}) result is not set for {}", file!(), line!(), inst_code))?
                    .borrow()
                    .get_npv_result()
                    .ok_or_else(|| anyhow!(
                        "({}:{}) npv is not set for {}", file!(), line!(), inst_code))?
                    .get_npv();

                let vega = (npv_up - npv) / bump_val * VEGA_PNL_UNIT * unitamt;
                (*self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!(
                        "({}:{}) result is not set for {}", file!(), line!(), inst.get_code()))?
                    )
                    .borrow_mut()
                    .set_single_vega(vol_code, vega);
            }
            // put back the bump
            {
                (*self.volatilities
                    .get(vol_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) volatility {} is not set\ntag:\n{}", 
                        file!(), line!(),
                        vol_code, self.err_tag
                    ))?).as_ref().borrow_mut()
                    .bump_volatility(
                        None, 
                        None, 
                        None,
                        None,
                        -bump_val
                    )?;
            }
        }
        Ok(())
    }


    // set vega structure performs the bump from the tail
    // this is for the arbitrage condition
    // say the bumped vector is vega_structure_up, with the length N
    // then vega_structure[i] = vege_structure_up[i] - vega_structure_up[i+1]
    // ... for i = 0, 1, ..., N-2 and
    // vega_structure[N-1] = vega_structure_up[N-1] - npv
    pub fn set_vega_structure(&mut self) -> Result<()> {
        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let bump_val = self.calculation_configuration.get_vega_structure_bump_value();
        let calc_tenors = self.calculation_configuration.get_vega_structure_tenors();
        let tenor_length = calc_tenors.len();
        let time_calculator = NullCalendar::default(); 
        let calc_times = calc_tenors.iter()
            .map(|tenor| add_period(&eval_dt, tenor.as_str()))
            .map(|dt| time_calculator.get_time_difference(&eval_dt, &dt))
            .collect::<Vec<Time>>();

        // instrument code (String) -> npv (Real)
        let mut current_npvs_up: HashMap::<String, Real>;
        let mut npv: Real;
        let mut npv_up: Real;
        // inst code (String) -> Vec<Real>
        let mut single_vega_structure: HashMap::<String, Vec<Real>>;
        let exclude_type = vec!["Cash", "Stock", "Futures"];
        let exclude_type_clone = exclude_type.clone();

        for und_code in all_underlying_codes {
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(
                    und_code, 
                    Some(exclude_type_clone.clone()));
            let longest_maturity = self.instruments.get_longest_maturity(Some(&self.instruments_in_action.clone()));
            let longest_mat_time = match longest_maturity {
                Some(m) => time_calculator.get_time_difference(&eval_dt, &m),
                None => 100_000_000.0
            };
            
            if self.instruments_in_action.is_empty() { continue; }
            let inst_codes_in_action = self.instruments.get_all_inst_code_clone(
                Some(&self.instruments_in_action));
            let init_vec: Vec<Vec<Real>> = vec![vec![0.0; tenor_length];inst_codes_in_action.len()];
            single_vega_structure = inst_codes_in_action.clone()
                .into_iter()
                .zip(init_vec.into_iter())
                .collect();
            let mut prev_npvs_up: HashMap::<String, Real> = HashMap::new();
            for inst_code in inst_codes_in_action.iter() {
                prev_npvs_up.insert(
                    inst_code.clone(),
                    self.calculation_results
                        .get(inst_code)
                        .ok_or_else(|| anyhow!(
                            "({}:{}) result is not set for {}",
                            file!(), line!(), inst_code,
                        ))?
                        .borrow()
                        .get_npv_result()
                        .ok_or_else(|| anyhow!(
                            "({}:{}) npv is not set for {}",
                            file!(), line!(), inst_code,
                        ))?
                        .get_npv()
                );
            }

            for i in (0..calc_times.len()).rev() {
                let bump_start = match i {
                    0 => None,
                    _ => Some(calc_times[i-1]),
                };
                let bump_end = Some(calc_times[i]);
                {
                    (*self.volatilities
                        .get(und_code)
                        .ok_or_else(|| anyhow!(
                            "({}:{}) volatility {} is not set\ntag:\n{}", 
                            file!(), line!(), und_code, self.err_tag
                        ))?).as_ref()
                        .borrow_mut()
                        .bump_volatility(
                            bump_start, bump_end, 
                            None, None,
                            bump_val)?;
                }
                if let Some(start) = bump_start {
                    if longest_mat_time < start {
                        continue;
                    }
                }
                current_npvs_up = self.get_npvs().with_context(|| anyhow!(
                    "({}:{}) failed to get npvs in vega structure", file!(), line!()))?;
                
                for inst in self.instruments_in_action.iter() {
                    let inst_code = inst.get_code();
                    let unitamt = inst.get_unit_notional();
                    npv_up = current_npvs_up.get(inst_code)
                        .ok_or_else(|| anyhow!("npv_up is not set for {}", inst_code))?.clone();
                    npv = prev_npvs_up.get(inst_code)
                        .ok_or_else(|| anyhow!("npv is not set for {}", inst_code))?.clone();
                    
                    let vega_structure = (npv_up - npv) / bump_val * VEGA_PNL_UNIT * unitamt;
                    single_vega_structure.get_mut(inst_code)
                        .ok_or_else(|| anyhow!(
                            "vega_structure is not set for {}", 
                            inst_code))?[i] = vega_structure;
                }
                prev_npvs_up = current_npvs_up;
                for (inst_code, vega_structure) in single_vega_structure.iter() {
                    (*self.calculation_results
                        .get(inst_code)
                        .ok_or_else(|| anyhow!(
                            "({}:{}) result is not set for {}",
                            file!(), line!(), inst_code,
                        ))?)
                        .borrow_mut()
                        .set_single_vega_structure(und_code, vega_structure.clone());
                }
            }
            // put back
            {
                (*self.volatilities
                    .get(und_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) volatility {} is not set\ntag:\n{}", 
                        file!(), line!(),
                        und_code, self.err_tag
                    ))?).as_ref()
                    .borrow_mut()
                    .bump_volatility(
                        None, None,
                        None, None,
                        -bump_val)?;
            }
        }
        Ok(())
    }

    pub fn set_vega_matrix(&mut self) -> Result<()> {
        let all_underlying_codes = self.instruments.get_all_underlying_codes();
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let bump_val = self.calculation_configuration.get_vega_structure_bump_value();
        let calc_tenors = self.calculation_configuration.get_vega_structure_tenors();
        
        let time_calculator = NullCalendar::default(); 
        let calc_times = calc_tenors.iter()
            .map(|tenor| add_period(&eval_dt, tenor.as_str()))
            .map(|dt| time_calculator.get_time_difference(&eval_dt, &dt))
            .collect::<Vec<Time>>();

        let spot_moneyness = self.calculation_configuration.get_vega_matrix_spot_moneyness().to_vec();
        
        // instrument code (String) -> npv (Real)
        let mut current_npvs_up: HashMap::<String, Real>;
        let mut npv: Real;
        let mut npv_up: Real;
        // inst code (String) -> Array2<Real>
        let mut single_vega_matrix: HashMap::<String, Array2<Real>> = HashMap::new();
        let exclude_type = vec!["Cash", "Stock", "Futures"];
        let exclude_type_clone = exclude_type.clone();

        for und_code in all_underlying_codes {
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(
                    und_code, 
                    Some(exclude_type_clone.clone()));
            let longest_maturity = self.instruments.get_longest_maturity(Some(&self.instruments_in_action.clone()));
            let longest_mat_time = match longest_maturity {
                Some(m) => time_calculator.get_time_difference(&eval_dt, &m),
                None => 100_000_000.0
            };
            
            if self.instruments_in_action.is_empty() { continue; }
            let inst_codes_in_action = self.instruments.get_all_inst_code_clone(
                Some(&self.instruments_in_action));

            for inst_code in inst_codes_in_action.iter() {
                let init = Array2::zeros((calc_times.len(), spot_moneyness.len()));
                single_vega_matrix.insert(inst_code.clone(), init);
            }

            let mut prev_npvs_up: HashMap::<String, Real> = HashMap::new();

            for inst_code in inst_codes_in_action.iter() {
                prev_npvs_up.insert(
                    inst_code.clone(),
                    self.calculation_results
                        .get(inst_code)
                        .ok_or_else(|| anyhow!(
                            "({}:{}) result is not set for {}",
                            file!(), line!(), inst_code,
                        ))?
                        .borrow()
                        .get_npv_result()
                        .ok_or_else(|| anyhow!(
                            "({}:{}) npv is not set for {}",
                            file!(), line!(), inst_code,
                        ))?
                        .get_npv()
                );
            }

            for i in (0..calc_times.len()).rev() {
                let bump_tenor_start = match i {
                    0 => None,
                    _ => Some(calc_times[i-1]),
                };
                let bump_tenor_end = Some(calc_times[i]);
                if let Some(start) = bump_tenor_start {
                    if longest_mat_time < start {
                        continue;
                    }
                }
                for j in 0..spot_moneyness.len() {
                    let bump_moneyness_start = match j {
                        0 => None,
                        _ => Some(spot_moneyness[j-1]),
                    };

                    let bump_moneyness_end = Some(spot_moneyness[j]);
                    {
                        (*self.volatilities
                            .get(und_code)
                            .ok_or_else(|| anyhow!(
                                "({}:{}) volatility {} is not set\ntag:\n{}", 
                                file!(), line!(), und_code, self.err_tag
                            ))?).as_ref()
                            .borrow_mut()
                            .bump_volatility(
                                bump_tenor_start, bump_tenor_end, 
                                bump_moneyness_start, bump_moneyness_end,
                                bump_val)?;
                    }

                    current_npvs_up = self.get_npvs().with_context(|| anyhow!(
                        "({}:{}) failed to get npvs in vega structure", file!(), line!()))?;
                    
                    //let code = String::from("165XXX3");
                    //println!("i: {}, j: {} current_npvs_up: {:?}", i, j, current_npvs_up.get(&code).unwrap());

                    for inst in self.instruments_in_action.iter() {
                        let inst_code = inst.get_code();
                        let unitamt = inst.get_unit_notional();
                        npv_up = current_npvs_up.get(inst_code)
                            .ok_or_else(|| anyhow!("npv_up is not set for {}", inst_code))?.clone();
                        npv = prev_npvs_up.get(inst_code)
                            .ok_or_else(|| anyhow!("npv is not set for {}", inst_code))?.clone();

                        let vega_matrix = (npv_up - npv) / bump_val * VEGA_PNL_UNIT * unitamt;
                        single_vega_matrix.get_mut(inst_code)
                            .ok_or_else(|| anyhow!(
                                "vega_matrix is not set for {}", 
                                inst_code))?[[i, j]] = vega_matrix;
                    }
                    prev_npvs_up = current_npvs_up;
                    for (inst_code, vega_matrix) in single_vega_matrix.iter() {
                        (*self.calculation_results
                            .get(inst_code)
                            .ok_or_else(|| anyhow!(
                                "({}:{}) result is not set for {}",
                                file!(), line!(), inst_code,
                            ))?)
                            .borrow_mut()
                            .set_single_vega_matrix(und_code, vega_matrix.clone());
                    }
                }
            }
            // put back
            {
                (*self.volatilities
                    .get(und_code)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) volatility {} is not set\ntag:\n{}", 
                        file!(), line!(),
                        und_code, self.err_tag
                    ))?).as_ref()
                    .borrow_mut()
                    .bump_volatility(
                        None, None,
                        None, None,
                        -bump_val)?;
            }
        }
        Ok(())
    }

    pub fn set_div_delta(&mut self) -> Result<()> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_dividend_codes = self.instruments.get_all_underlying_codes();
        let bump_val = self.calculation_configuration.get_div_bump_value();
        let mut npv: Real;
        let exclude_type = vec!["Stock", "Cash"];
        let exclude_type_clone = exclude_type.clone();
        
        for div_code in all_dividend_codes {
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(div_code, Some(exclude_type_clone.clone()));

            if self.instruments_in_action.is_empty() {
                continue;
            }
            // bump dividend but limit the scope that is mutably borrowed
            {
                self.dividends.get(div_code)
                    .unwrap().clone().unwrap().borrow_mut()
                    .bump_date_interval(None, None, bump_val)?;
            }

            npvs_up = self.get_npvs().context("failed to get npvs")?; // instrument code (String) -> npv (Real

            for inst in &self.instruments_in_action {
                let inst_code = inst.get_code();
                let unitamt = inst.get_unit_notional();
                let npv_up = npvs_up.get(inst_code)
                    .ok_or_else(|| anyhow!("npv_up is not set"))?;

                npv = self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow()
                    .get_npv_result()
                    .ok_or_else(|| anyhow!("npv is not set"))?
                    .get_npv();


                let div_delta = (npv_up - npv) / bump_val * DIV_PNL_UNIT * unitamt;
                (*self.calculation_results
                    .get(inst.get_code())
                    .ok_or_else(|| anyhow!("result is not set for {}", inst.get_code()))?)
                    .borrow_mut()
                    .set_single_div_delta(div_code, div_delta);
            }
            // put back the bump
            {
                self.dividends
                    .get(div_code)
                    .unwrap()
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .bump_date_interval(None, None, -bump_val)?;
            }
        }
        Ok(())
    }

    pub fn preprocess_theta(&mut self, inst_type: Vec<&str>) -> Result<()> {
        let insts = self.instruments.instruments_with_types(inst_type);
        for inst in insts {
            let inst_code = inst.get_code();
            (*self.calculation_results
                .get(inst_code)
                .ok_or_else(|| anyhow!(
                    "({}:{}) result is not set for {} ({})",
                    file!(), line!(), inst_code, inst.get_type_name(),
                ))?)
                .borrow_mut()
                .set_theta(0.0);
        }
        Ok(())
    }
    /// Set theta for the given instruments where the evaluation date is bumped to bumped_date. 
    /// Note that the theta result is represented per day. 
    /// Only self.set_theta has the inputs, given_instruments and bumped_dates. 
    /// This is for handling instruments whose maturity is within the evaluation_date + theta_day.
    pub fn set_theta_for_given_instruments(
        &mut self, 
        given_instruments: Vec<Rc<Instrument>>,
        bumped_date: OffsetDateTime,
    ) -> Result<()> {
        // 
        self.instruments_in_action = given_instruments;
        
        let time_calculator = NullCalendar::default();
        let original_evaluation_date = self.evaluation_date.borrow().get_date_clone();
        let time_diff = time_calculator.get_time_difference(&original_evaluation_date, &bumped_date);

        let mut cash_sum: Real;
        let npvs = self.get_npvs()
            .with_context(|| anyhow!(
                "({}:{}) failed to get npvs",
                file!(), line!()))?;

        // limit the scope that the attribute is mutably borrowed
        
        { (*self.evaluation_date).borrow_mut().set_date(bumped_date.clone()); }

        let npvs_theta = self.get_npvs()
            .with_context(|| anyhow!(
                "({}:{}) failed to get npvs",
                file!(), line!()))?;

        let continue_type = vec!["Stock", "Cash"];
        for inst in self.instruments_in_action.iter() {
            let inst_code = inst.get_code();
            let inst_type = inst.get_type_name();
            if !continue_type.contains(&inst_type) {
                continue;
            };

            let result = self.calculation_results
                .get(inst_code)
                .context("result is not set")?;

            let unitamt = result
                .borrow()
                .get_instrument_info()
                .context("instrument_info is not set")?
                .get_unit_notional();

            let npv = npvs.get(inst_code)
                .context("npv is not set")?;

            let npv_theta = npvs_theta.get(inst_code)
                .context("npv_theta is not set")?;

            // deduct the cashflow inbetween
            // the scope bound is for borrowing the result
            {
                let result_borrow = result.borrow();
                let cashflows = result_borrow
                    .get_cashflows()
                    .ok_or_else(|| anyhow!(
                        "cashflows is not set for {} ({})", 
                        inst_code,
                        inst_type,
                    ))?;

                cash_sum = 0.0;
                for (date, cash) in cashflows.iter() {
                    if (original_evaluation_date.date() < date.date()) && (date.date() <= bumped_date.date()) {
                        cash_sum += cash;
                        println!(
                            "# {} ({}) has a cashflow: {} at {}\n", 
                            inst_code, 
                            inst_type,
                            cash, 
                            date);
                    }
                }
            }
            
            let theta = (npv_theta - npv + cash_sum) * unitamt / time_diff / 365.0 * THETA_PNL_UNIT;
                
            result.borrow_mut().set_theta(theta);
        }
        // put back
        { (*self.evaluation_date).borrow_mut().set_date(original_evaluation_date); }

        Ok(())
    }

    pub fn set_rho_structure(&mut self) -> Result<()> {
        let all_curve_codes = self.instruments.get_all_curve_names(&self.match_parameter)?;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let bump_val = self.calculation_configuration.get_rho_bump_value();
        let calc_tenors = self.calculation_configuration.get_rho_structure_tenors();
        let tenor_length = calc_tenors.len();
        let time_calculator = NullCalendar::default(); 
        let calc_dates = calc_tenors.iter()
            .map(|tenor| add_period(&eval_dt, tenor.as_str()))
            .collect::<Vec<_>>();
        let mut calc_times =  Vec::<Time>::new();
        for date in calc_dates.iter() {
            calc_times.push(time_calculator.get_time_difference(&eval_dt, date));
        }

        // instrument code (String) -> npv (Real)
        let mut npvs_up: HashMap::<String, Real>;
        let mut npv_up: Real;
        let mut npv: Real;
        // inst code (String) -> Vec<Real>
        let mut single_rho_structure: HashMap<String, Vec<Real>>;
        let mut val: Real;
        let exclude_type = vec!["Stock", "Cash"];
        let exclude_type_clone = exclude_type.clone();
        for curve_code in all_curve_codes {
            self.instruments_in_action = self.instruments
                .instruments_using_curve(
                    curve_code, 
                    &self.match_parameter,
                    Some(exclude_type_clone.clone()),
                )?;
            
            if self.instruments_in_action.is_empty() {
                continue;
            }

            let inst_codes_in_action = self.instruments.get_all_inst_code_clone(Some(&self.instruments_in_action));
            let init_vec: Vec<Vec<Real>> = vec![vec![0.0; tenor_length];inst_codes_in_action.len()];
            single_rho_structure = inst_codes_in_action.into_iter().zip(init_vec.into_iter()).collect();
            // bump zero_curve by bump_date_interval where calc_dates[i] < date <= calc_dates[i+1]
            for i in 0..calc_times.len() {
                let bump_start = match i {
                    0 => None,
                    _ => Some(calc_times[i-1]),
                };
                let bump_end = Some(calc_times[i]);
                
                // bump the curve with the limit of the scope of mutable borrow
                {
                    (*self.zero_curves.get(curve_code)
                        .with_context(|| anyhow!(
                            "({}:{}) no zero curve: {}\n{}", 
                            file!(), line!(),
                            curve_code, self.err_tag,))?)
                            .as_ref()
                            .borrow_mut()
                            .bump_time_interval(bump_start, bump_end, bump_val)?;
                }
                
                // 
                npvs_up = self.get_npvs().context("failed to get npvs")?;
                for inst in &self.instruments_in_action {
                    let inst_code = inst.get_code();
                    let unitamt = inst.get_unit_notional();
                    npv_up = npvs_up.get(inst_code).context("failed to get npv_up in rho-structure calculation")?.clone();
                    npv = self.calculation_results.get(inst_code)
                        .context("failed to get npv in rho-structure calculation")?
                        .borrow().get_npv_result()
                        .context("failed to get npv_result in rho-structure calculation")?.get_npv();

                    val = (npv_up - npv) / bump_val * RHO_PNL_UNIT * unitamt;
                    single_rho_structure.get_mut(inst_code)
                        .context("failed to get single_rho_structure")?[i] = val;
                }
                // put back
                {
                    (*self.zero_curves.get(curve_code)
                        .with_context(|| anyhow!(
                            "({}:{}) no zero curve: {}\n{}", 
                            file!(), line!(),
                            curve_code, self.err_tag,))?)
                            .as_ref()
                            .borrow_mut()
                            .bump_time_interval(bump_start, bump_end, -bump_val)?;
                }

                // if there is no instrument over the calc_tenors, we do not need to calculate the next bump
                let inst_over_bump_end = self.instruments
                    .instruments_with_maturity_over(
                        Some(&self.instruments_in_action), 
                        &calc_dates[i],
                        Some(exclude_type_clone.clone()),
                    );
                if inst_over_bump_end.is_empty() {
                    break;
                }

            }

            for (inst_code, rho_structure) in single_rho_structure.iter() {
                (*self.calculation_results
                    .get(inst_code)
                    .with_context(|| anyhow!(
                        "({}:{}) failed to get result of {}",
                        file!(), line!(), inst_code,
                    ))?
                ).borrow_mut()
                .set_single_rho_structure(curve_code, rho_structure.clone());
            }
        }
        Ok(())
    }

    pub fn set_div_structure(&mut self) -> Result<()> {
        //let all_dividend_codes = self.instruments.get_all_underlying_codes();
        let all_dividend_codes = self.dividends.keys().collect::<Vec<&String>>();
        let bump_val = self.calculation_configuration.get_div_bump_value();
        let calc_tenors = self.calculation_configuration.get_div_structure_tenors();
        let tenor_length = calc_tenors.len();
        let calc_dates = calc_tenors.iter()
            .map(|tenor| add_period(
                &self.evaluation_date.borrow().get_date_clone(), 
                tenor.as_str(),
            )).collect::<Vec<_>>();
        
        // instrument code (String) -> npv (Real)
        let mut npvs_up: HashMap::<String, Real>;
        let mut npv_up: Real;
        let mut npv: Real;
        // inst code (String) -> Vec<Real>
        let mut single_div_structure: HashMap<String, Vec<Real>>;
        let mut val: Real;
        let exclude_type = vec!["Stock", "Cash"];
        let exclude_type_clone = exclude_type.clone();

        for div_code in all_dividend_codes {
            // reset instruments
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(div_code, Some(exclude_type_clone.clone()));

            if self.instruments_in_action.is_empty() {
                continue;
            }
            // initialize the single_div_structure. insert the inst code and zero vector
            let inst_codes_in_action = self.instruments.get_all_inst_code_clone(Some(&self.instruments_in_action));

            let init_vec: Vec<Vec<Real>> = vec![vec![0.0; tenor_length];inst_codes_in_action.len()];
            single_div_structure = inst_codes_in_action.into_iter().zip(init_vec.into_iter()).collect();

            // bump dividend by bump_date_interval where calc_dates[i] < date <= calc_dates[i+1]
            for i in 0..calc_dates.len() {
                let bump_start = match i {
                    0 => None,
                    _ => Some(&calc_dates[i-1]),
                };
                let bump_end = Some(&calc_dates[i]);
                {            
                    self.dividends
                        .get(div_code)
                        .unwrap()
                        .clone()
                        .unwrap()
                        .borrow_mut()
                        .bump_date_interval(bump_start, bump_end, bump_val)?;
                }
                
                // 
                npvs_up = self.get_npvs()?;
                for inst in &self.instruments_in_action {
                    let inst_code = inst.get_code();
                    let unitamt = inst.get_unit_notional();
                    npv_up = npvs_up.get(inst_code).context("failed to get npv_up in div-structure calculation")?.clone();
                    npv = self.calculation_results.get(inst_code)
                        .context("failed to get npv in div-structure calculation")?
                        .borrow().get_npv_result()
                        .context("failed to get npv_result in div-structure calculation")?.get_npv();

                    val = (npv_up - npv) / bump_val * DIV_PNL_UNIT * unitamt;
                    single_div_structure.get_mut(inst_code)
                        .context("failed to get single_div_structure")?[i] = val;
                }

                {            
                    self.dividends
                        .get(div_code)
                        .unwrap()
                        .clone()
                        .unwrap()
                        .borrow_mut()
                        .bump_date_interval(bump_start, bump_end, -bump_val)?;
                }

                // if there is no instrument over the calc_tenors, we do not need to calculate the next bump
                let inst_over_bump_end = self.instruments
                    .instruments_with_maturity_over(
                        Some(&self.instruments_in_action), 
                        &calc_dates[i],
                        Some(exclude_type_clone.clone()));

                if inst_over_bump_end.is_empty() {
                    break;
                }
            }
            for (inst_code, div_structure) in single_div_structure.iter() {
                self.calculation_results.get(inst_code)
                    .context("failed to get result")?
                    .borrow_mut()
                    .set_single_div_structure(div_code, div_structure.clone());
            }
        }
        Ok(())
    }

    pub fn calculate(&mut self) -> Result<()>{
        let mut timer = std::time::Instant::now();
        let start_time = std::time::Instant::now();

        if self.instruments_in_action.len() < 1 {    
            println!("* no instruments to calculate in engine-{}\n", self.engine_id);
        }

        if self.calculation_configuration.get_npv_calculation() {
            self.set_npv_results()?;
            self.set_values()?;
            self.set_cashflow_inbetween()?;

            println!(
                "* npv calculation is done (engine id: {}, time = {} whole time elapsed: {})"
                , self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }
        
        if self.calculation_configuration.get_fx_exposure_calculation() {
            timer = std::time::Instant::now();
            self.set_fx_exposures()?;
            println!(
                "* fx exposure calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }
        
        if self.calculation_configuration.get_delta_calculation() { 
            timer = std::time::Instant::now();
            self.preprocess_delta_gamma()?;
            self.set_delta_gamma()?;
            println!(
                "* delta calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }

        if self.calculation_configuration.get_theta_calculation() {
            timer = std::time::Instant::now();
            let exclude_type = vec!["Cash", "Stock"];
            let exclude_type_clone = exclude_type.clone();
            self.preprocess_theta(exclude_type_clone.clone())?;
            // we separate instruments by 
            // 1) instruments whose maturity is within the evaluation_date + theta_day
            // 2) instruments whose maturity is not within the evaluation_date + theta_day
            let bumped_day = self.evaluation_date.borrow().get_date_clone() 
                + Duration::days(self.calculation_configuration.get_theta_day() as i64);
                
            let insts_upto_bumped_day = self.instruments
                .instruments_with_maturity_upto(
                    None, 
                    &bumped_day,
                    Some(exclude_type.clone()),
                );

            let insts_over_bumped_day = self.instruments
                .instruments_with_maturity_over(
                    None, 
                    &bumped_day,
                    Some(exclude_type),
                );

            if !insts_upto_bumped_day.is_empty() {
                let shortest_maturity = self.instruments.get_shortest_maturity(Some(&insts_upto_bumped_day)).unwrap();
                println!(
                    "{}:{}\n\
                    (Engine::calculate -> theta calculation)\n\
                    There are instruments whose maturity is within the evaluation_date + theta_day (= {:?}) \n\
                    {}",
                    file!(), line!(), &bumped_day, self.err_tag
                );
                // print the instruments' name and maturity
                for inst in insts_upto_bumped_day.iter() {
                    println!("{}: {}", inst.get_name(), inst.get_maturity().unwrap());
                }
                println!(
                    "For the theta calculation for the above instruments, \n\
                    the evaluation date is bumped to {:?} which is the shortest maturity of the above instruments. \n\
                    Note that the theta calculation period may be too small to get accurate theta.\n", 
                    &shortest_maturity
                );
                self.set_theta_for_given_instruments(insts_upto_bumped_day, shortest_maturity)?;
            }

            if !insts_over_bumped_day.is_empty() {
                self.set_theta_for_given_instruments(insts_over_bumped_day, bumped_day)?;
            }
            println!(
                "* theta calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }

        if self.calculation_configuration.get_vega_calculation() {
            timer = std::time::Instant::now();
            self.set_vega()?;
            println!(
                "* vega calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64())
            );
        }

        if self.calculation_configuration.get_rho_calculation() {
            timer = std::time::Instant::now();
            self.set_rho()?;
            println!(
                "* rho calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64())
            );
        }

        if self.calculation_configuration.get_div_delta_calculation() {
            timer = std::time::Instant::now();
            self.set_div_delta()?;
            println!(
                "* div_delta calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }

        if self.calculation_configuration.get_vega_structure_calculation() {
            timer = std::time::Instant::now();
            self.set_vega_structure()?;
            println!(
                "* vega_structure calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64())
            );
        }

        if self.calculation_configuration.get_rho_structure_calculation() {
            timer = std::time::Instant::now();
            self.set_rho_structure()?;
            println!(
                "* rho calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64())
            );
        }

        if self.calculation_configuration.get_div_structure_calculation() {
            timer = std::time::Instant::now();
            self.set_div_structure()?;
            println!(
                "* div_structure calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }

        if self.calculation_configuration.get_vega_matrix_calculation() {
            timer = std::time::Instant::now();
            self.set_vega_matrix()?;
            println!(
                "* vega_matrix calculation is done (engine id: {}, time = {} whole time elapsed: {})", 
                self.engine_id, 
                format_duration(timer.elapsed().as_secs_f64()),
                format_duration(start_time.elapsed().as_secs_f64()),
            );
        }
        Ok(())
    }

    pub fn get_calculation_result(&self) -> &HashMap<String, RefCell<CalculationResult>> {
        &self.calculation_results
    }

    pub fn get_calculation_result_clone(&self) -> HashMap<String, CalculationResult> {
        let mut result = HashMap::new();
        for (key, value) in self.calculation_results.iter() {
            result.insert(key.clone(), value.borrow().clone());
        }
        result
    }
}
