use crate::time::{
    calendars::nullcalendar::NullCalendar,
    calendar_trait::CalendarTrait,
};
use crate::evaluation_date::EvaluationDate;
use crate::parameters::market_price::MarketPrice;
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use crate::parameters::{
    zero_curve::ZeroCurve,
    volatility::Volatility,
    quanto::Quanto,
};
use crate::pricing_engines::{
    npv_result::NpvResult,
    futures_pricer::FuturesPricer,
};
use crate::instrument::InstrumentTrait;
use crate::enums::OptionType;
//
use anyhow::{anyhow, Context, Result};

use std::{
    rc::Rc,
    cell::RefCell,
};
use statrs::distribution::{Normal, ContinuousCDF};

pub struct OptionAnalyticPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    market_price: Rc<RefCell<MarketPrice>>,   
    futures_helper: FuturesPricer,
    discount_curve: Rc<RefCell<ZeroCurve>>,
    volatility: Rc<RefCell<Volatility>>,
    quanto: Option<Rc<RefCell<Quanto>>>,
    time_calculator: NullCalendar,
}

impl OptionAnalyticPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        market_price: Rc<RefCell<MarketPrice>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>, 
        discount_curve: Rc<RefCell<ZeroCurve>>,
        volatility: Rc<RefCell<Volatility>>,
        quanto: Option<Rc<RefCell<Quanto>>>,
    ) -> OptionAnalyticPricer {
        let futures_helper = FuturesPricer::new(
            //evaluation_date.clone(),
            market_price.clone(),
            collateral_curve.clone(),
            borrowing_curve.clone(),
        );

        OptionAnalyticPricer {
            evaluation_date,
            market_price,
            futures_helper,
            discount_curve,
            volatility,
            quanto,
            time_calculator: NullCalendar::new(),
        }
    }
}

impl PricerTrait for OptionAnalyticPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let maturity = instrument.get_maturity()
            .context("(OptionAnalyticPricer:npv) Failed to get maturity")?;
        let fwd = self.futures_helper.fair_forward(maturity)?;
        let strike = instrument.get_strike()?;
        let forward_moneyness = strike / fwd;
        let t = self.time_calculator.get_time_difference(
            self.evaluation_date.borrow().get_date(),
            maturity,
        );
        
        let total_variance = self.volatility
            .borrow()
            .total_variance(t, forward_moneyness)?;
        let total_deviation = self.volatility
            .borrow()
            .total_deviation(t, forward_moneyness)?;

        if instrument.get_currency() != instrument.get_underlying_currency()? &&
        self.quanto.is_none() 
        {
            return Err(anyhow!(
                "({}:{}) {} ({}) has different currency from underlying market_price ({}) but no quanto is provided",
                file!(), line!(),
                instrument.get_name(), instrument.get_code(), self.market_price.borrow().get_name(),
            ));
        }

        let vol = self.volatility.borrow()
            .get_value(t, forward_moneyness);
        let quanto_adjustment = match &self.quanto {
            Some(quanto) => {
                vol * t * quanto.borrow().quanto_adjust(t, forward_moneyness)
            }
            None => 0.0,
        };

        let y = forward_moneyness.ln();
        let option_type = instrument.get_option_type()?;

        let dsc = self.discount_curve.borrow().get_discount_factor(t)?;

        let d1 = (-y + total_variance / 2.0 - quanto_adjustment) / total_deviation;
        let d2 = d1 - total_deviation;

        let normal = Normal::new(0.0, 1.0).unwrap(); 
        let nd1 = normal.cdf(d1 as f64) as Real;
        let nd2 = normal.cdf(d2 as f64) as Real;

        match option_type {
            OptionType::Call => {
                Ok(dsc * (fwd * nd1 - strike * nd2))
            }
            OptionType::Put => {
                Ok(dsc * (strike * (1.0 - nd2) - fwd * (1.0 - nd1)))
            }
        }
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::enums::{OptionDailySettlementType, OptionExerciseType, OptionType, StickynessType};
    use crate::instrument::Instrument;
    use crate::instruments::vanilla_option::VanillaOption;
    use crate::parameters::volatilities::local_volatility_surface::LocalVolatilitySurface;
    use crate::parameters::market_price::MarketPrice;
    use crate::parameters::{
        quanto::Quanto,
        volatilities::volatiltiy_interpolator::VolatilityInterplator,
    };
    use crate::currency;
    use crate::currency::Currency;
    use crate::utils;
    use crate::{
        vectordatasample,
        surfacedatasample,
    };
    use crate::data;
    use time::macros::datetime;
    use std::{
        rc::Rc,
        cell::RefCell,
    };
    use anyhow::Result;
    use ndarray::Array1;

    #[test]
    fn test_option_analytic_pricer_npv() -> Result<()> {
        let eval_date = datetime!(2024-01-02 16:30:00 +09:00);
        let evaluation_date = Rc::new(RefCell::new(
            EvaluationDate::new(eval_date.clone())
        ));
        let spot = 357.38;
        let market_price = Rc::new(RefCell::new(
            MarketPrice::new(
                spot,
                eval_date.clone(),
                None,
                Currency::KRW,
                "KOSPI2".to_string(),
                "KOSPI2".to_string(),
            )
        ));

        let discount_curve_data = vectordatasample!(0.03, Currency::KRW, "Option Test Curve")?;
        let discount_curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &discount_curve_data,
                "Option Test Curve".to_string(),
                "Option Test Curve".to_string(),
            )?
        ));

        let surface_data = surfacedatasample!(&eval_date, spot);
        let vega_structure_tenors = vec![
            String::from("1M"),
            String::from("2M"),
            String::from("3M"),
            String::from("6M"),
            String::from("9M"),
            String::from("1Y"),
            String::from("2Y"),
            String::from("3Y"),
        ];
        let vega_matrix_spot_moneyness = Array1::linspace(0.6, 1.4, 17);
        
        let local_volatility = LocalVolatilitySurface::initialize(
            evaluation_date.clone(),
            market_price.clone(),
            discount_curve.clone(),
            discount_curve.clone(),
            StickynessType::StickyToMoneyness,
            VolatilityInterplator::default(),
            "KOSPI2 Local Volatility".to_string(),
            "KOSPI2 Local Volatility".to_string(),
        ).with_market_surface(
            &surface_data,
            vega_structure_tenors.clone(),
            vega_matrix_spot_moneyness.clone(),
        )?;
        
        let vol = Volatility::LocalVolatilitySurface(local_volatility);
        
        let volatility = Rc::new(RefCell::new(vol));

        volatility.borrow_mut().build()?;
        
        let quanto = Rc::new(RefCell::new(
            Quanto::default()
        ));

        let pricer = OptionAnalyticPricer::new(
            evaluation_date.clone(),
            market_price.clone(),
            discount_curve.clone(),
            discount_curve.clone(),
            discount_curve.clone(),
            volatility.clone(),
            Some(quanto.clone()),
        );

        let issue_date = datetime!(2023-09-15 16:30:00 +09:00);
        let maturity = datetime!(2024-09-15 16:30:00 +09:00);
        let option = VanillaOption::new(
            spot * 0.85,
            250_000.0,
            issue_date.clone(),
            maturity.clone(),
            maturity.clone(),
            maturity.clone(),
            vec!["KOSPI2".to_string()],
            Currency::KRW,
            Currency::KRW,
            OptionType::Put,
            OptionExerciseType::European,
            OptionDailySettlementType::NotSettled,
            "KOSPI2 Put Option".to_string(),
            "KOSPI2 Put Option".to_string(),
        );

        let inst = Instrument::VanillaOption(option);
        let npv = pricer.npv(&inst)?;
        let expected_npv = 6.41674;

        assert!((npv - expected_npv).abs() < 1.0e-5, "npv: {}, expected_npv: {}", npv, expected_npv);
        
        Ok(())
    }

}