use crate::instruments::instrument_info::InstrumentInfo;
use crate::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, Instruments};
use crate::pricing_engines::calculation_result::CalculationResult;
use crate::definitions::{Real, Time, DELTA_PNL_UNIT, DIV_PNL_UNIT, RHO_PNL_UNIT, THETA_PNL_UNIT};
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
use anyhow::{Result, Context, anyhow, bail};
use time::{OffsetDateTime, Duration};
use crate::pricing_engines::npv_result::NpvResult;
use crate::util::format_duration;
use crate::utils::string_arithmetic::add_period;
/// Engine typically handles a bunch of instruments and calculate the pricing of the instruments.
/// Therefore, the result of calculations is a hashmap with the key being the code of the instrument
/// Engine is a struct that holds the calculation results of the instruments
pub struct Engine {
    engine_id: u64,
    err_tag: String,
    //
    calculation_results: HashMap<String, RefCell<CalculationResult>>,
    calculation_configuration: CalculationConfiguration, // this should be cloned
    //stock_data: HashMap<String, ValueData>,
    //fx_data: HashMap<String, ValueData>,
    //curve_data: HashMap<String, RefCell<VectorData>>,
    //dividend_data: HashMap<String, RefCell<VectorData>>,
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
    ) -> Result<Engine> {
        let evaluation_date = Rc::new(RefCell::new(
            evaluation_date
        ));

        let mut zero_curves = HashMap::new();
        //let mut curve_data_refcell = HashMap::new();
        for (key, data) in curve_data.into_iter() {
            let zero_curve = Rc::new(RefCell::new(
                ZeroCurve::new(
                    evaluation_date.clone(),
                    &data,
                    String::from(&key),
                    String::from(&key),
                ).with_context(|| anyhow!("failed to create zero curve {}", key))?
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
        for (key, data) in dividend_data.into_iter() {
            let spot = stock_data.get(&key)
                .with_context(|| anyhow!(
                    "failed to get dividend to match stock data for {}", key))?
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

            dividends.insert(key.to_string(), dividend.clone());
            /* 
            let ref_cell = RefCell::new(data);
            ref_cell.borrow_mut().add_observer(dividend);
            dividend_data_refcell.insert(key.to_string(), ref_cell);
             */
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
            // stock_data,
            // fx_data,
            // curve_data: curve_data_refcell,
            // dividend_data: dividend_data_refcell,
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
    ) -> Result<()> {
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
                anyhow!("no instruments are given to initialize")
            );
        }

        self.instruments = Instruments::new(instrument_vec);
        let all_types = self.instruments.get_all_type_names();
        let curr_str: Vec<&str> = self.instruments.get_all_currencies().iter().map(|c| c.as_str()).collect();
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
        let insts_over_maturity = self.instruments.instruments_with_maturity_upto(None, &dt);

        if !insts_over_maturity.is_empty() {
            let mut inst_codes = Vec::<String>::new();
            let mut inst_mat = Vec::<Option<OffsetDateTime>>::new();
            for inst in insts_over_maturity {
                inst_codes.push(inst.as_trait().get_code().clone());
                let mat = inst.as_trait().get_maturity();
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

        let insts_with_very_short_maturity = self.instruments.instruments_with_maturity_upto(None, &(dt + Duration::hours(6)));
        if !insts_with_very_short_maturity.is_empty() {
            let mut inst_codes = Vec::<String>::new();
            let mut inst_mat = Vec::<Option<OffsetDateTime>>::new();
            for inst in insts_with_very_short_maturity {
                inst_codes.push(inst.as_trait().get_code().clone());
                let mat = inst.as_trait().get_maturity();
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

    pub fn initialize_pricers(&mut self) -> Result<()> {
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
                        anyhow!("not implemented pricer for {} {}", inst_type, inst_name)
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

    pub fn get_npvs(&self) -> Result<HashMap<String, Real>> {
        let mut npvs = HashMap::new();
        for inst in &self.instruments_in_action {
            let inst_code = inst.as_trait().get_code();
            let pricer = self.pricers.get(inst_code)
                .with_context(|| anyhow!("(Egnine::get_npvs) failed to get pricer for {}\n{}", inst_code, self.err_tag))?;

            let npv = pricer
                .as_trait()
                .npv(inst)
                .with_context(|| anyhow!("(Egnine::get_npvs) failed to get npv for {}\n{}", inst_code, self.err_tag))?;
    
            npvs.insert(inst_code.clone(), npv);
        }
        Ok(npvs)
    }

    pub fn get_npv_results(&self) -> Result<HashMap<String, NpvResult>> {
        let mut npvs = HashMap::new();
        for inst in &self.instruments_in_action {
            let inst_code = inst.as_trait().get_code();
            let pricer = self.pricers.get(inst_code)
                .with_context(|| anyhow!(
                    "(Engine::get_npv_results) failed to get pricer for {}\n{}",
                    inst_code,
                    self.err_tag,
                ))?;

            let npv = pricer.as_trait().npv_result(inst)?;
            npvs.insert(inst.as_trait().get_code().clone(), npv);
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
        for (code, result) in self.calculation_results.iter_mut() {
            let npv_res = result.borrow().get_npv_result()
                .ok_or_else(|| anyhow!(
                    "npv_result is not set for {}\n{}", code, self.err_tag,
                ))?.clone();
                
            let cashflow = npv_res.get_expected_coupon_amount()
                .with_context(|| anyhow!(
                    "failed to get expected coupon amount for {}", code
                ))?;
            result.borrow_mut().set_cashflow_inbetween(cashflow);
        }
        Ok(())
    }

    pub fn set_fx_exposures(&mut self) -> Result<()> {
        let mut fx_exposures = HashMap::new();
        for inst in &self.instruments_in_action {
            let inst_code = inst.as_trait().get_code();
            let pricer = self.pricers.get(inst_code)
                .ok_or_else(|| anyhow!("failed to get pricer for {} in getting fx-exposure", inst_code))?;
            
            let npv = self.calculation_results.get(inst_code)
                .ok_or_else(|| anyhow!("failed to get npv for {} in getting fx-exposure", inst_code))?
                .borrow()
                .get_npv_result()
                .ok_or_else(|| anyhow!("npv is not set for {} in getting fx-exposure", inst_code))?
                .get_npv();

            let fx_exposure = pricer.as_trait().fx_exposure(inst, npv)
                .context("failed to get fx exposure")?;
            
            fx_exposures.insert(inst.as_trait().get_code(), fx_exposure);
        }
        for (code, result) in self.calculation_results.iter_mut() {
            result.borrow_mut().set_fx_exposure(
                fx_exposures.get(code)
                .ok_or_else(|| anyhow!("fx exposure is not set"))?.clone()
            );
        }
        Ok(())
    }

    /// Set the value of the instruments which means npv * unit_notional
    pub fn set_values(&mut self) -> Result<()> {
        for (_code, result) in self.calculation_results.iter_mut() {
            result.borrow_mut().set_value()?;
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
        
        for und_code in all_underlying_codes.iter() {
            original_price = self.stocks
                .get(*und_code)
                .ok_or_else(|| anyhow!("there is no stock {}", und_code))?
                .borrow()
                .get_last_price();

            // set instruments that needs to be calculated
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(und_code);

            {
                let mut stock = self.stocks
                    .get(*und_code)
                    .ok_or_else(|| anyhow!("there is no stock {}", und_code))?
                    .borrow_mut();
                *stock *= up_bump;
            }

            delta_up_map = self.get_npvs().context("failed to get npvs")?;
            {
                let mut stock = self.stocks
                    .get(*und_code)
                    .ok_or_else(|| anyhow!("there is no stock {}", und_code))?
                    .borrow_mut();

                stock.set_price(original_price);
                *stock *= down_bump;
            }
            
            delta_down_map = self.get_npvs().context("failed to get npvs")?;
            
            for inst in &self.instruments_in_action {
                let inst_code = inst.as_trait().get_code();
                let unitamt = inst.as_trait().get_unit_notional();
                delta_up = delta_up_map
                    .get(inst_code)
                    .ok_or_else(|| anyhow!("delta_up is not set"))?
                    .clone();
                delta_down = delta_down_map
                    .get(inst_code)
                    .ok_or_else(|| anyhow!("delta_down is not set"))?
                    .clone();

                delta = (delta_up - delta_down) / (2.0 * delta_bump_ratio) * DELTA_PNL_UNIT;

                self.calculation_results
                    .get_mut(inst_code)
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow_mut()
                    .set_single_delta(&und_code, delta * unitamt);

                mid = self.calculation_results
                    .get(inst.as_trait().get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow()
                    .get_npv_result()
                    .ok_or_else(|| anyhow!("npv is not set"))?
                    .get_npv();

                gamma = delta_up - mid + delta_down - mid;
                gamma *= DELTA_PNL_UNIT / delta_bump_ratio;
                gamma *= 0.5 * (DELTA_PNL_UNIT / delta_bump_ratio);

                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow_mut()
                    .set_single_gamma(&und_code, gamma * unitamt);
            }

            {
                let mut stock = self.stocks
                    .get(*und_code)
                    .ok_or_else(|| anyhow!("there is no stock {}", und_code))?
                    .borrow_mut();

                stock.set_price(original_price);
            }
        }
        Ok(())
    }

    pub fn set_rho(&mut self) -> Result<()> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_curve_names = self.instruments.get_all_curve_names(&self.match_parameter);
        let bump_val = self.calculation_configuration.get_rho_bump_value();

        for curve_name in all_curve_names {
            self.instruments_in_action = self.instruments
                .instruments_using_curve(curve_name, &self.match_parameter);

            // bump the curve but limit the scope that the zero_curve ismutably borrowed
            {
                self.zero_curves.get(curve_name)
                    .with_context(|| anyhow!("no zero curve: {}\n{}", curve_name, self.err_tag,))?
                    .borrow_mut()
                    .bump_time_interval(None, None, bump_val)?;
            }

            npvs_up = self.get_npvs().context("failed to get npvs")?;

            for inst in &self.instruments_in_action {
                let inst_code = inst.as_trait().get_code();
                let unitamt = inst.as_trait().get_unit_notional();
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
                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow_mut()
                    .set_single_rho(curve_name, rho);
            }
            // put back the bump value
            {
                self.zero_curves.get(curve_name)
                    .with_context(|| anyhow!("no zero curve: {}\n{}", curve_name, self.err_tag,))?
                    .borrow_mut()
                    .bump_time_interval(None, None, -bump_val)?;
            }
        }
        Ok(())
    }

    pub fn set_div_delta(&mut self) -> Result<()> {
        let mut npvs_up: HashMap::<String, Real>;
        let all_dividend_codes = self.instruments.get_all_underlying_codes();
        let bump_val = self.calculation_configuration.get_div_bump_value();
        let mut npv: Real;

        for div_code in all_dividend_codes {

            // bump dividend but limit the scope that is mutably borrowed
            {
                self.dividends
                    .get(div_code)
                    .ok_or_else(|| anyhow::anyhow!(
                        "dividend {} is not set\ntag:\n{}", div_code, self.err_tag
                    ))?.borrow_mut()
                    .bump_date_interval(None, None, bump_val)?;
            }

            self.instruments_in_action = self.instruments
                .instruments_with_underlying(div_code);

            npvs_up = self.get_npvs().context("failed to get npvs")?; // instrument code (String) -> npv (Real

            for inst in &self.instruments_in_action {
                let inst_code = inst.as_trait().get_code();
                let unitamt = inst.as_trait().get_unit_notional();
                let npv_up = npvs_up.get(inst_code)
                    .ok_or_else(|| anyhow!("npv_up is not set"))?;

                npv = self.calculation_results
                    .get(inst.as_trait().get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow()
                    .get_npv_result()
                    .ok_or_else(|| anyhow!("npv is not set"))?
                    .get_npv();


                let div_delta = (npv_up - npv) / bump_val * DIV_PNL_UNIT * unitamt;
                self.calculation_results
                    .get_mut(inst.as_trait().get_code())
                    .ok_or_else(|| anyhow!("result is not set"))?
                    .borrow_mut()
                    .set_single_div_delta(div_code, div_delta);
            }
            // put back the bump
            {
                self.dividends
                    .get(div_code)
                    .ok_or_else(|| anyhow::anyhow!(
                        "dividend {} is not set\ntag:\n{}", div_code, self.err_tag
                    ))?.borrow_mut()
                    .bump_date_interval(None, None, -bump_val)?;
            }
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
            .context("failed to get npvs")?;

        // limit the scope that the attribute is mutably borrowed
        
        { self.evaluation_date.borrow_mut().set_date(bumped_date.clone()); }

        let npvs_theta = self.get_npvs()
            .context("failed to get npvs")?;

        for inst in self.instruments_in_action.iter() {
            let inst_code = inst.as_trait().get_code();
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
            cash_sum = result.borrow().get_cashflow_inbetween()
                .context("cashflow_inbetween is not set")?
                .iter()
                .filter( |(date, _)| (original_evaluation_date.date() < date.date()) && (date.date() <= bumped_date.date()))
                .map(|(_, cash)| cash)
                .sum();
                

            let theta = (npv_theta.clone() - npv - cash_sum) * unitamt / time_diff / 365.0 * THETA_PNL_UNIT;
            result.borrow_mut().set_theta(theta);
        }
        // put back
        { self.evaluation_date.borrow_mut().set_date(original_evaluation_date); }
        Ok(())
    }

    pub fn set_rho_structure(&mut self) -> Result<()> {
        let all_curve_codes = self.instruments.get_all_curve_names(&self.match_parameter);
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
        
        for curve_code in all_curve_codes {
            self.instruments_in_action = self.instruments
                .instruments_using_curve(curve_code, &self.match_parameter);
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
                    self.zero_curves.get(curve_code)
                        .with_context(|| anyhow!("no zero curve: {}\n{}", curve_code, self.err_tag,))?
                        .borrow_mut()
                        .bump_time_interval(bump_start, bump_end, bump_val)?;
                }
                
                // 
                npvs_up = self.get_npvs().context("failed to get npvs")?;
                for inst in &self.instruments_in_action {
                    let inst_code = inst.as_trait().get_code();
                    let unitamt = inst.as_trait().get_unit_notional();
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
                    self.zero_curves.get(curve_code)
                        .with_context(|| anyhow!("no zero curve: {}\n{}", curve_code, self.err_tag,))?
                        .borrow_mut()
                        .bump_time_interval(bump_start, bump_end, -bump_val)?;
                }

                // if there is no instrument over the calc_tenors, we do not need to calculate the next bump
                let inst_over_bump_end = self.instruments
                    .instruments_with_maturity_over(
                        Some(&self.instruments_in_action), 
                        &calc_dates[i]);
                if inst_over_bump_end.is_empty() {
                    break;
                }

            }

            for (inst_code, rho_structure) in single_rho_structure.iter() {
                self.calculation_results.get_mut(inst_code)
                    .context("failed to get result")?
                    .borrow_mut()
                    .set_single_rho_structure(curve_code, rho_structure.clone());
            }
        }
        Ok(())
    }

    pub fn set_div_structure(&mut self) -> Result<()> {
        let all_dividend_codes = self.instruments.get_all_underlying_codes();
        let bump_val = self.calculation_configuration.get_div_bump_value();
        let calc_tenors = self.calculation_configuration.get_div_structure_tenors();
        let tenor_length = calc_tenors.len();
        let calc_dates = calc_tenors.iter()
            .map(|tenor| add_period(
                &self.evaluation_date.borrow().get_date_clone(), 
                tenor.as_str()
            ))
            .collect::<Vec<_>>();
        
        // instrument code (String) -> npv (Real)
        let mut npvs_up: HashMap::<String, Real>;
        let mut npv_up: Real;
        let mut npv: Real;
        // inst code (String) -> Vec<Real>
        let mut single_div_structure: HashMap<String, Vec<Real>>;
        let mut val: Real;
        

        for div_code in all_dividend_codes {
            // reset instruments
            self.instruments_in_action = self.instruments
                .instruments_with_underlying(div_code);
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
                        .ok_or_else(|| anyhow::anyhow!(
                            "dividend {} is not set\ntag:\n{}", div_code, self.err_tag
                        ))?.borrow_mut()
                        .bump_date_interval(bump_start, bump_end, bump_val)?;
                }
                
                // 
                npvs_up = self.get_npvs()?;
                for inst in &self.instruments_in_action {
                    let inst_code = inst.as_trait().get_code();
                    let unitamt = inst.as_trait().get_unit_notional();
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
                        .ok_or_else(|| anyhow::anyhow!(
                            "dividend {} is not set\ntag:\n{}", div_code, self.err_tag
                        ))?.borrow_mut()
                        .bump_date_interval(bump_start, bump_end, -bump_val)?;
                }

                // if there is no instrument over the calc_tenors, we do not need to calculate the next bump
                let inst_over_bump_end = self.instruments
                    .instruments_with_maturity_over(
                        Some(&self.instruments_in_action), 
                        &calc_dates[i]);
                if inst_over_bump_end.is_empty() {
                    break;
                }
            }
            for (inst_code, div_structure) in single_div_structure.iter() {
                self.calculation_results.get_mut(inst_code)
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
            // we separate instruments by 
            // 1) instruments whose maturity is within the evaluation_date + theta_day
            // 2) instruments whose maturity is not within the evaluation_date + theta_day
            let bumped_day = self.evaluation_date.borrow().get_date_clone() 
                + Duration::days(self.calculation_configuration.get_theta_day() as i64);
                
            let insts_upto_bumped_day = self.instruments
                .instruments_with_maturity_upto(None, &bumped_day);

            let insts_over_bumped_day = self.instruments
                .instruments_with_maturity_over(None, &bumped_day);

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
                    println!("{}: {}", inst.as_trait().get_name(), inst.as_trait().get_maturity().unwrap());
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