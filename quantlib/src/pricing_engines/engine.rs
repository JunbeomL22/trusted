use crate::data::observable::Observable;
use crate::instruments::instrument_info::InstrumentInfo;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, Instruments};
use crate::pricing_engines::calculation_result::CalculationResult;
use crate::definitions::{Time, Real, DELTA_PNL_UNIT, RHO_PNL_UNIT};
use crate::assets::stock::Stock;
use crate::data::vector_data::VectorData;
use crate::data::value_data::ValueData;
use crate::pricing_engines::pricer::Pricer;
use crate::pricing_engines::stock_futures_pricer::StockFuturesPricer;
use crate::pricing_engines::match_parameter::MatchParameter;
use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::calendars::nullcalendar::NullCalendar;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use anyhow::{Result, Context};
use crate::utils::myerror::MyError;
use crate::pricing_engines::npv_result::NpvResult;
/// Engine typically handles a bunch of instruments and calculate the pricing of the instruments.
/// Therefore, the result of calculations is a hashmap with the key being the code of the instrument
/// Engine is a struct that holds the calculation results of the instruments
pub struct Engine {
    engine_id: u64,
    err_tag: String,
    //
    calculation_results: HashMap<String, RefCell<CalculationResult>>,
    calculation_configuration: CalculationConfiguration, // this should be cloned
    stock_data: HashMap<String, ValueData>,
    fx_data: HashMap<String, ValueData>,
    curve_data: HashMap<String, RefCell<VectorData>>,
    dividend_data: HashMap<String, RefCell<VectorData>>,
    //
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fxs: HashMap<String, Rc<RefCell<Real>>>,
    stocks: HashMap<String, Rc<RefCell<Stock>>>,
    zero_curves: HashMap<String, Rc<RefCell<ZeroCurve>>>,
    dividends: HashMap<String, Rc<RefCell<DiscreteRatioDividend>>>,
    // instruments
    instruments: Instruments, // all instruments
    pricers: HashMap<String, Pricer>, // pricers for each instrument
    // selected instuments for calculation,
    // e.g., if we calcualte a delta of a single stock, we do not need calculate all instruments
    instruments_in_action: Vec<Rc<Instrument>>, 
    match_parameter: MatchParameter, // this must be cloned 
}

impl Engine {
    pub fn new (
        engine_id: u64,
        calculation_configuration: CalculationConfiguration,
        evaluation_date: EvaluationDate,
        //
        fx_data: HashMap<String, ValueData>,
        stock_data: HashMap<String, ValueData>,
        curve_data: HashMap<String, VectorData>,
        dividend_data: HashMap<String, VectorData>,
        //
        match_parameter: MatchParameter,
    ) -> Result<Engine, MyError> {
        let evaluation_date = Rc::new(RefCell::new(
            evaluation_date
        ));

        let mut zero_curves = HashMap::new();
        let mut curve_data_refcell = HashMap::new();
        for (key, data) in curve_data.into_iter() {
            let zero_curve = Rc::new(RefCell::new(
                ZeroCurve::new(
                    evaluation_date.clone(),
                    &data,
                    String::from(&key),
                    String::from(&key),
                ).context("failed to create zero curve")?
            ));
            zero_curves.insert(key.to_string(), zero_curve.clone());

            let ref_cell = RefCell::new(data);
            ref_cell.borrow_mut().add_observer(zero_curve);
            curve_data_refcell.insert(key.to_string(), ref_cell);
        }

        let mut dividends = HashMap::new();
        let mut dividend_data_refcell = HashMap::new();
        for (key, data) in dividend_data.into_iter() {
            let spot = stock_data.get(&key)
                .context("Failed to find stock data matching the dividend data")?
                .get_value();

            let dividend = Rc::new(RefCell::new(
                DiscreteRatioDividend::new(
                    evaluation_date.clone(),
                    &data,
                    spot,
                    key.to_string(),
                ).context("failed to create discrete ratio dividend")?
            ));

            dividends.insert(key.to_string(), dividend.clone());
            let ref_cell = RefCell::new(data);
            ref_cell.borrow_mut().add_observer(dividend);
            dividend_data_refcell.insert(key.to_string(), ref_cell);
        }
        // making fx Rc -> RefCell for pricing
        let mut fxs: HashMap<String, Rc<RefCell<Real>>> = HashMap::new();
        fx_data
            .iter()
            .map(|(key, data)| {
                let rc = Rc::new(RefCell::new(data.get_value()));
                fxs.insert(key.clone(), rc)
            });

        // making stock Rc -> RefCell for pricing
        let mut stocks = HashMap::new();
        for (key, data) in stock_data.iter() {
            let div = match dividends.get(key) {
                Some(div) => Some(div.clone()),
                None => None,
            };

            let rc = Rc::new(RefCell::new(
                Stock::new(
                    data.get_value(),
                    data.get_market_datetime().clone(),
                    div,
                    data.get_currency().clone(),  
                    data.get_name().clone(),
                    key.to_string(),
                )
            ));
            stocks.insert(key.clone(), rc);
            }
        
        
        Ok(Engine {
            engine_id: engine_id,
            err_tag : "".to_string(),
            calculation_results: HashMap::new(),
            calculation_configuration,
            //
            stock_data,
            fx_data,
            curve_data: curve_data_refcell,
            dividend_data: dividend_data_refcell,
            //
            evaluation_date,
            fxs,
            stocks,
            zero_curves,
            dividends,
            //
            instruments: Instruments::default(),
            instruments_in_action: vec![],
            pricers: HashMap::new(),
            match_parameter: match_parameter,
        })
    }

    // initialize CalculationResult for each instrument
    pub fn initialize(
        &mut self, 
        instrument_vec: Vec<Rc<Instrument>>,
    ) -> Result<(), MyError> {
        self.initialize_instruments(instrument_vec)
            .with_context(|| format!(
                "(Engine::initialize) Failed to initialize instruments\n\
                occuring at {file}:{line}",
                file = file!(),
                line = line!(),
            ))?;

        self.initialize_pricers()?;
            //.with_context(|| anyhow::anyhow!(
            //    "(Engine::initialize) Failed to initialize pricers\n\
            //    tag:\n{}",
            //    self.err_tag,
            //))?;

        Ok(())
    }

    pub fn initialize_instruments(
        &mut self, 
        instrument_vec: Vec<Rc<Instrument>>,
    ) -> Result<(), MyError> {
        if instrument_vec.is_empty() {
            return Err(
                MyError::EmptyVectorError {
                    file: file!().to_string(),
                    line: line!(),
                    other_info: "(Engine::initialize_instruments) instruments are empty".to_string(),
                }
            );
        }

        self.instruments = Instruments::new(instrument_vec);
        let all_types = self.instruments.get_all_type_names();
        let curr_str: Vec<&str> = self.instruments.get_all_currencies().iter().map(|c| c.as_str()).collect();
        let all_und_codes: Vec<&str> = self.instruments.get_all_underlying_codes().iter().map(|c| c.as_str()).collect();
        self.err_tag = format!(
            "engine-id: {}\n\
            instrument-types: {}\n\
            currencies: {}\n\
            underlying-codes: {}\n",
            self.engine_id,
            all_types.join(" / "),
            curr_str.join(" / "),
            all_und_codes.join(" / "),
        );

        self.instruments_in_action = self.instruments
            .get_instruments_clone();
    
        for instrument in self.instruments.iter() {
            let inst = instrument.as_trait();
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

    pub fn initialize_pricers(&mut self) -> Result<(), MyError> {
        let inst_vec = self.instruments.get_instruments_clone();
        for inst in inst_vec.iter() {
            let inst_type = inst.as_trait().get_type_name();
            let inst_name = inst.as_trait().get_name();
            let inst_code = inst.as_trait().get_code();
            let undertlying_codes = inst.as_trait().get_underlying_codes();

            let pricer = match Rc::as_ref(inst) {
                Instrument::StockFutures(_) => {
                    let stock = self.stocks.get(undertlying_codes[0]).unwrap().clone();
                    let collatral_curve_name = self.match_parameter.get_collateral_curve_names(inst)[0];
                    let borrowing_curve_name = self.match_parameter.get_borrowing_curve_names(inst)[0];
                    let core = StockFuturesPricer::new(
                        stock,
                        self.zero_curves.get(collatral_curve_name)
                        .ok_or_else(|| anyhow::anyhow!(
                            "failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                            inst_code,
                            collatral_curve_name,
                        ))?.clone(),
                        self.zero_curves.get(borrowing_curve_name)
                        .ok_or_else(|| anyhow::anyhow!(
                            "failed to get borrowing curve of {}.\nself.zero_curves does not have {}",
                            inst_code,
                            borrowing_curve_name,
                        ))?.clone(),
                        self.evaluation_date.clone(),
                    );
                    Pricer::StockFuturesPricer(Box::new(core))
                },
                _ => {
                    return Err(
                        MyError::BaseError {
                            file: file!().to_string(), 
                            line: line!(), 
                            contents: format!(
                                "Not implemented yet (type = {}, name =  {})\n\
                                error tag of engine:\n{}", 
                                inst_type,
                                inst_name,
                                self.err_tag,
                            )
                        }
                    );
                }
            };
            self.pricers.insert(inst_code.clone(), pricer);
        }
        Ok(())
    }

    /// re-initialize instruments_in_action
    pub fn reset_instruments_in_action(&mut self) {
        self.instruments_in_action = self.instruments
            .get_instruments_clone();
    }

    pub fn get_npvs(&self) -> Result<HashMap<String, Real>, MyError> {
        let mut npvs = HashMap::new();
        for inst in &self.instruments_in_action {
            let pricer = self.pricers.get(inst.as_trait().get_code())
                .with_context(|| format!(
                    "(Engine::get_npv) failed to get pricer for {}\n\
                    error tag:\n{}",
                    inst.as_trait().get_code(),
                    self.err_tag,
                ))?;

            let npv = pricer.as_trait().npv(inst)
                .context("calculation failed")?;
            npvs.insert(inst.as_trait().get_code().clone(), npv);
        }
        Ok(npvs)
    }

    pub fn get_npv_results(&self) -> Result<HashMap<String, NpvResult>, MyError> {
        let mut npvs = HashMap::new();
        for inst in &self.instruments_in_action {
            let pricer = self.pricers.get(inst.as_trait().get_code())
                .with_context(|| format!(
                    "(Engine::get_npv_results) failed to get pricer for {}\n\
                    error tag:\n{}",
                    inst.as_trait().get_code(),
                    self.err_tag,
                ))?;

            let npv = pricer.as_trait().npv_result(inst)
                .context("calculation failed")?;
            npvs.insert(inst.as_trait().get_code().clone(), npv);
        }
        Ok(npvs)
    }

    pub fn set_npv_results(&mut self) -> Result<(), MyError> {
        let npvs = self.get_npv_results()?;
        
        for (code, result) in self.calculation_results.iter() {
            result.borrow_mut().set_npv(npvs.get(code)
                .context("npv is not set")?.clone());
            }
        Ok(())
    }

    pub fn set_fx_exposures(&mut self) -> Result<(), MyError> {
        let mut fx_exposures = HashMap::new();
        for inst in &self.instruments_in_action {
            let pricer = self.pricers.get(inst.as_trait().get_code())
                .context("failed to get pricer")?;
            let fx_exposure = pricer.as_trait().fx_exposure(inst)
                .context("calculation failed")?;
            fx_exposures.insert(inst.as_trait().get_code(), fx_exposure);
        }
        for (code, result) in self.calculation_results.iter_mut() {
            result.borrow_mut().set_fx_exposure(
                fx_exposures.get(code)
                    .context("fx_exposure is not set")?.clone()
                );
        }
        Ok(())
    }

    /// Set the value of the instruments which means npv * unit_notional
    pub fn set_values(&mut self) -> Result<(), MyError> {
        for (_code, result) in self.calculation_results.iter_mut() {
            result.borrow_mut().set_value()?;
        }
        Ok(())
    }

    
    pub fn set_delta_gamma(&mut self) -> Result<(), MyError> {
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

        for und_code in all_underlying_codes.iter() {
            // set instruments that needs to be calculated
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(und_code, None);

            let stock = self.stocks
                .get(*und_code)
                .context("there is no stock")?
                .clone();

            *stock.borrow_mut() *= 1.0 + delta_bump_ratio;
            delta_up_map = self.get_npvs().context("failed to get npvs")?;
            *stock.borrow_mut() *= 1.0 / (1.0 + delta_bump_ratio);
            *stock.borrow_mut() *= 1.0 - delta_bump_ratio;
            delta_down_map = self.get_npvs().context("failed to get npvs")?;
            
            for inst in &self.instruments_in_action {
                delta_up = delta_up_map
                    .get(inst.as_trait().get_code())
                    .context("delta_up is not set")?
                    .clone();
                delta_down = delta_down_map
                    .get(inst.as_trait().get_code())
                    .context("delta_down is not set")?
                    .clone();

                delta = (delta_up - delta_down) / (2.0 * delta_bump_ratio) * DELTA_PNL_UNIT;

                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow_mut()
                    .to_owned()
                    .set_single_delta(&und_code, delta);

                mid = self.calculation_results
                    .get(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow()
                    .get_npv_result()
                    .context("npv is not set")?
                    .get_npv();

                gamma = delta_up + delta_down - mid * 2.0;
                gamma /= delta_bump_ratio * delta_bump_ratio;
                gamma *= 0.5 * DELTA_PNL_UNIT * DELTA_PNL_UNIT;

                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow_mut()
                    .to_owned()
                    .set_single_gamma(&und_code, gamma);
            }

            *stock.borrow_mut() *= 1.0 / (1.0 - delta_bump_ratio);
        }
        Ok(())
    }
    
    /*
    pub fn set_coupons(&mut self) -> Result<(), MyError> {
        for inst in &self.instruments_in_action {
            let start_date = self.evaluation_date.borrow().get_date_clone();
            let theta_day = self.calculation_configuration.get_theta_day();
            let end_date = start_date + Duration::days(theta_day as i64);
            let coupons = self.pricers
                    .get(inst.as_trait().get_code())
                    .context("pricer is not set")?
                    .as_trait()
                    .coupons(inst, &start_date, &end_date)
                    .with_context(||
                        format!(
                            "coupon calculation failed for {}\ntag:\n{}", 
                            inst.as_trait().get_code(),
                            self.err_tag
                        ))?;

            self.calculation_results.get_mut(inst.as_trait().get_code())
                .context("result is not set")?
                .borrow_mut()
                .set_cashflow_inbetween(coupons);
        }
        Ok(())
    }
    */

    pub fn set_rho(&mut self) -> Result<(), MyError> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_curve_names = self.instruments.get_all_curve_names(&self.match_parameter);
        let bump_val = self.calculation_configuration.get_rho_bump_value();

        for curve_name in all_curve_names {
            let mut curve_data = self.curve_data.get(curve_name)
                .with_context(|| format!(
                    "no curve data: {}\ntag:\n{}", 
                    curve_name,
                    self.err_tag,
            ))?.borrow_mut();

            self.instruments_in_action = self.instruments
                .instruments_using_curve(curve_name, &self.match_parameter, None);

            *curve_data += bump_val;

            npvs_up = self.get_npvs()
                .context("failed to get npvs")?;

            for inst in &self.instruments_in_action {
                let npv_up = npvs_up.get(inst.as_trait().get_code())
                    .context("npv_up is not set")?;
                let npv = self.calculation_results
                    .get(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow()
                    .get_npv_result()
                    .context("npv is not set")?
                    .get_npv();

                let rho = (npv_up.clone() - npv) / bump_val * RHO_PNL_UNIT;
                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow_mut()
                    .to_owned()
                    .set_single_rho(curve_name, rho);
            }

            *curve_data -= bump_val;
        }
        Ok(())
    }

    pub fn set_div_delta(&mut self) -> Result<(), MyError> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_dividend_codes = self.instruments.get_all_underlying_codes();
        let bump_val = self.calculation_configuration.get_div_bump_value();
        let mut npv: Real;

        for div_code in all_dividend_codes {
            let mut div_data = self.dividend_data
                .get(div_code)
                .context("dividend data is not set")?
                .borrow_mut();

            self.instruments_in_action = self.instruments
                .instruments_with_underlying(div_code, None);

            *div_data += bump_val;

            npvs_up = self.get_npvs()
                .context("failed to get npvs")?; // instrument code (String) -> npv (Real

            for inst in &self.instruments_in_action {
                let npv_up = npvs_up.get(inst.as_trait().get_code())
                    .context("npv_up is not set")?;
                npv = self.calculation_results
                    .get(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow()
                    .get_npv_result()
                    .context("npv is not set")?
                    .get_npv();

                let div_delta = (npv_up.clone() - npv) / bump_val * DELTA_PNL_UNIT;
                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .context("result is not set")?
                    .borrow_mut()
                    .to_owned()
                    .set_single_div_delta(div_code, div_delta);
            }

            *div_data -= bump_val;
        }
        Ok(())
    }

    pub fn set_rho_structure(&mut self) -> Result<(), MyError> {
        let all_curve_names = self.instruments.get_all_curve_names(&self.match_parameter);
        let bump_val = self.calculation_configuration.get_rho_bump_value();
        let calc_tenors = self.calculation_configuration.get_rho_structure_tenors();
        let time_calculator = NullCalendar::default(); 

        // rho_structure_up = instrument code (String) -> curve code (String) -> tenor (String) -> rho (Real)
        let mut accumulative_rho_structure_up = HashMap::<String, HashMap::<String, HashMap::<String, Real>>>::new();

        // instrument code (String) -> npv (Real)
        let mut npvs_up: HashMap::<String, Real>;

        for curve_name in all_curve_names {
            let mut curve_data = self.curve_data.get(curve_name)
                .with_context(|| format!(
                    "curve_data {} is not set\ntag:\n{}", curve_name, self.err_tag
                ))?.borrow_mut();
                
            for tenor in calc_tenors {
                let end_date = self.evaluation_date.borrow().clone() + tenor.as_str();

                self.instruments_in_action = self.instruments
                    .instruments_using_curve(curve_name, &self.match_parameter, Some(end_date.clone()));

                if self.instruments_in_action.is_empty() {
                    continue;
                }

                let tenor_time: Time = time_calculator.get_time_difference(
                    &self.evaluation_date.borrow().get_date_clone(),
                    &end_date,
                );

                curve_data.bump_time_interval(-0.0, tenor_time, bump_val)
                    .context("failed to bump time interval")?;

                npvs_up = self.get_npvs()
                    .context("failed to get npvs")?;
                
                for inst in &self.instruments_in_action {
                    let npv_up = npvs_up.get(inst.as_trait().get_code())
                        .context("npv_up is not set")?;
                    let npv = self.calculation_results
                        .get(inst.as_trait().get_code())
                        .context("result is not set")?
                        .borrow()
                        .get_npv_result()
                        .context("npv is not set")?
                        .get_npv();

                    let rho = (npv_up - npv) / bump_val * RHO_PNL_UNIT;
                    accumulative_rho_structure_up
                        .entry(inst.as_trait().get_code().clone())
                        .or_insert(HashMap::new())
                        .entry(curve_name.clone())
                        .or_insert(HashMap::new())
                        .insert(tenor.clone(), rho);
                }
            }
        }
        // now using accumulative_rho_structure_up, we can calculate rho_structure
        // For the first element, it becomes rho_structure[0] = accumulative_rho_structure_up[0]
        // For others, it becomes rho_structure[i] = accumulative_rho_structure_up[i] - accumulative_rho_structure_up[i-1]
        for (inst_code, result) in self.calculation_results.iter_mut() {
            // curve code (String) -> tenor (String) -> accmulative rho (Real)
            let single_inst_accum_up = accumulative_rho_structure_up
                    .get(inst_code)
                    .context("accumulative_rho_structure_up is not set")?;
            // curve code (String) -> tenor (String) -> rho in the tener interval (Real)
            for (curve_code, accum_up) in single_inst_accum_up.iter() {
                let mut prev = 0.0;
                let mut single_rho_structure = HashMap::<String, Real>::new();
                for (tenor, rho) in accum_up.iter() {
                    single_rho_structure.insert(
                        tenor.clone(),
                        *rho - prev,
                    );
                    prev = *rho;
                }
                result.borrow_mut().set_single_rho_structure(curve_code, single_rho_structure)
            }          
        }
        Ok(())
    }
    pub fn set_theta(&mut self) -> Result<(), MyError> {
        self.reset_instruments_in_action();

        let theta_day = self.calculation_configuration.get_theta_day();
        let mut cash_sum: Real;
        let npvs = self.get_npvs()
            .context("failed to get npvs")?;

        let theta_day_str = format!("{}D", theta_day);
        let period_str = theta_day_str.as_str();

        *self.evaluation_date.borrow_mut() += period_str;

        let npvs_theta = self.get_npvs()
            .context("failed to get npvs")?;

        for (code, result) in self.calculation_results.iter_mut() {
            let npv = npvs.get(code)
                .context("npv is not set")?;

            let npv_theta = npvs_theta.get(code)
                .context("npv_theta is not set")?;

            // deduct the cashflow inbetween
            cash_sum = result.borrow().get_cashflow_inbetween().as_ref()
                .context("cashflow_inbetween is not set")?
                .values()
                .sum();

            let theta = (npv_theta.clone() - npv - cash_sum) / (theta_day as Real);
            result.borrow_mut().set_theta(theta);
        }

        *self.evaluation_date.borrow_mut() -= theta_day_str.as_str();
        Ok(())
    }

    pub fn set_div_structure(&mut self) -> Result<(), MyError> {
        let all_dividend_codes = self.instruments.get_all_underlying_codes();
        let bump_val = self.calculation_configuration.get_div_bump_value();
        let calc_tenors = self.calculation_configuration.get_div_structure_tenors();
        let time_calculator = NullCalendar::default(); 
        // div_structure_up = instrument code (String) -> div code (String) -> tenor (String) -> div (Real)
        let mut accumulative_div_structure_up = HashMap::<String, HashMap::<String, HashMap::<String, Real>>>::new();
        // instrument code (String) -> npv (Real)
        let mut npvs_up: HashMap::<String, Real>;

        let mut npv_up: &Real;
        let mut npv: &Real;

        for div_code in all_dividend_codes {
            let mut div_data = self.dividend_data
                .get(div_code)
                .ok_or_else(|| anyhow::anyhow!(
                    "dividend data {} is not set\ntag:\n{}", div_code, self.err_tag
                ))?.borrow_mut();

            for tenor in calc_tenors {
                let end_date = self.evaluation_date.borrow().clone() + tenor.as_str();

                self.instruments_in_action = self.instruments
                    .instruments_with_underlying(div_code, Some(end_date.clone()));

                if self.instruments_in_action.is_empty() {
                    continue;
                }

                let tenor_time: Time = time_calculator.get_time_difference(
                    &self.evaluation_date.borrow().get_date_clone(),
                    &end_date,
                );

                div_data.bump_time_interval(
                    -0.0, 
                    tenor_time, 
                    bump_val
                ).context("failed to bump time interval")?;

                npvs_up = self.get_npvs().context("failed to get npvs")?;
                
                for inst in &self.instruments_in_action {
                    let inst_code = inst.as_trait().get_code();
                    npv_up = npvs_up.get(inst_code)
                        .ok_or_else(|| anyhow::anyhow!("npv_up is not set for {}", inst_code))?;
                    
                    let npv = self.calculation_results
                            .get(inst_code)
                            .context("result is not set")?
                            .borrow()
                            .get_npv_result()
                            .context("npv is not set")?
                            .get_npv();
                        
                    let div = (npv_up - npv) / bump_val * DELTA_PNL_UNIT;
                    accumulative_div_structure_up
                        .entry(inst.as_trait().get_code().clone())
                        .or_insert(HashMap::new())
                        .entry(div_code.clone())
                        .or_insert(HashMap::new())
                        .insert(tenor.clone(), div);
                }
            }
        }
        // now using accumulative_div_structure_up, we can calculate div_structure
        // For the first element, it becomes div_structure[0] = accumulative_div_structure_up[0]
        // For others, it becomes div_structure[i] = accumulative_div_structure_up[i] - accumulative_div_structure_up[i-1]
        for (inst_code, result) in self.calculation_results.iter_mut() {
            // div code (String) -> tenor (String) -> accmulative div (Real)
            let single_inst_accum_up = accumulative_div_structure_up
                    .get(inst_code)
                    .context("accumulative_div_structure_up is not set")?;
            // div code (String) -> tenor (String) -> div in the tener interval (Real)
            for (div_code, accum_up) in single_inst_accum_up.iter() {
                let mut prev = 0.0;
                let mut single_div_structure = HashMap::<String, Real>::new();
                for (tenor, div) in accum_up.iter() {
                    single_div_structure.insert(
                        tenor.clone(),
                        *div - prev,
                    );
                    prev = *div;
                }
                result.borrow_mut().set_single_div_structure(div_code, single_div_structure)
            }          
        }
        Ok(())
    }

    pub fn calculate(&mut self) -> Result<(), MyError>{
        if self.instruments_in_action.len() < 1 {
            return Err(
                MyError::BaseError { 
                    file: file!().to_string(), 
                    line: line!(), 
                    contents: format!(
                        "instruments_in_action is empty\n\
                        error tag of engine:\n{}", 
                        self.err_tag,
                    )
                });
        }
        if self.calculation_configuration.get_npv_calculation() {
            self.set_npv_results().with_context(|| format!("failed to set npvs.\ntag:\n{}", self.err_tag))?;
            self.set_values().with_context(|| format!("failed to set values.\ntag:\n{}", self.err_tag))?;
            //self.set_coupons().with_context(|| format!("failed to set coupons.\ntag:\n{}", self.err_tag))?;
        }
        
        if self.calculation_configuration.get_fx_exposure_calculation() {
            self.set_fx_exposures().with_context(|| format!("failed to set fx exposures.\ntag:\n{}", self.err_tag))?;
        }
        
        if self.calculation_configuration.get_delta_calculation() { 
            self.set_delta_gamma().with_context(|| format!("failed to set delta_gamma.\ntag:\n{}", self.err_tag))?;
        }

        if self.calculation_configuration.get_theta_calculation() {
            self.set_theta().with_context(|| format!("failed to set theta.\ntag:\n{}", self.err_tag))?;
        }

        if self.calculation_configuration.get_rho_calculation() {
            self.set_rho().with_context(|| format!("failed to set rho.\ntag:\n{}", self.err_tag))?;
        }

        if self.calculation_configuration.get_div_delta_calculation() {
            self.set_div_delta().with_context(|| format!("failed to set div_delta.\ntag:\n{}", self.err_tag))?;
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