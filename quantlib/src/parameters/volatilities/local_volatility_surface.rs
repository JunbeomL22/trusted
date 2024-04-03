use crate::enums::StickynessType;
use crate::definitions::{Time, Real};
use crate::data::surface_data::SurfaceData;
use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
use crate::parameters::{
    volatility::{
        VolatilityTrait,
        VolatilityType,
    },
    volatilities::volatiltiy_interpolator::VolatilityInterplator,
    zero_curve::ZeroCurve,
};
use crate::evaluation_date::EvaluationDate;
use crate::math::interpolators::bilinear_interpolator::BilinearInterpolator;
use crate::market_price::MarketPrice;
use crate::time::calendar_trait::CalendarTrait;
use crate::time::calendars::nullcalendar::NullCalendar;
use crate::utils::string_arithmetic::add_period;
use crate::math::interpolator::ExtraPolationType;
use std::{
    rc::Rc,
    cell::RefCell,
};
use anyhow::{Result, Context, anyhow};
//
use ndarray::{Array1, Array2};
use time::OffsetDateTime;
#[derive(Clone, Debug)]
pub struct LocalVolatilitySurface {
    interpolated_imvol: Array2<Real>,
    imvol_maturity_dates: Vec<OffsetDateTime>,
    imvol_maturity_times: Array1<Time>,
    imvol_spot_moneyness: Array1<Real>,
    imvol_forward_vector: Array1<Real>,
    imvol_spot: Real,
    forward_monenyess_imvol: BilinearInterpolator,
    //
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    market_price: Rc<RefCell<MarketPrice>>,
    collateral_curve: Rc<RefCell<ZeroCurve>>,
    borrowing_curve: Rc<RefCell<ZeroCurve>>,
    //
    stickyness_type: StickynessType,
    lv_interpolator: VolatilityInterplator,
    local_volatility: BilinearInterpolator,
    //
    name: String,
    code: String,
}

impl LocalVolatilitySurface {
    pub fn initialize(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        market_price: Rc<RefCell<MarketPrice>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
        stickyness_type: StickynessType,
        lv_interpolator: VolatilityInterplator,
        name: String,
        code: String,
    ) -> LocalVolatilitySurface {
        LocalVolatilitySurface {
            interpolated_imvol: Array2::default((0, 0)),
            imvol_maturity_dates: Vec::new(),
            imvol_maturity_times: Array1::default(0),
            imvol_spot_moneyness: Array1::default(0),
            imvol_forward_vector: Array1::default(0),
            forward_monenyess_imvol: BilinearInterpolator::default(),
            imvol_spot: 0.0,
            //
            evaluation_date,
            market_price,
            collateral_curve,
            borrowing_curve,
            //
            stickyness_type,
            lv_interpolator,
            local_volatility: BilinearInterpolator::default(),
            //
            name,
            code,
        }
    }

    pub fn with_market_surface(
        mut self,
        market_implied_volatility_surface: &SurfaceData,
        vega_structure_tenors: Vec<String>,
        vega_matrix_spot_moneyness: Array1<Real>,
    ) -> Result<LocalVolatilitySurface> {
        let given_dates = market_implied_volatility_surface.get_dates();
        let eval_date = self.evaluation_date.borrow().get_date_clone();

        if given_dates.windows(2).any(|w| w[0] > w[1]) {
            return Err(anyhow!(
                "({}:{}) Maturity dates of {} ({}) are not sorted: {:?}",
                file!(), line!(), self.name, self.code, given_dates
            ));
        }

        for date in given_dates.iter() {
            if date < &eval_date {
                return Err(anyhow!(
                    "({}:{}) Maturity date of {} ({}) is before evaluation date: {}",
                    file!(), line!(), self.name, self.code, date
                ));
            }
        }

        let time_calculator = NullCalendar::new();
        let given_times: Array1<Time> = given_dates.iter()
            .map(|date| time_calculator.get_time_difference(&eval_date, date))
            .collect::<Vec<Time>>().into();

        self.imvol_spot = market_implied_volatility_surface.get_spot()
            .ok_or_else(|| anyhow!(
                "({}:{}) Error getting spot from market_implied_volatility_surface of {}",
                file!(), line!(), market_implied_volatility_surface.get_name()
            ))?;

        self.imvol_forward_vector = Array1::default(given_dates.len());
        for (i, date) in given_dates.iter().enumerate() {
            let fwd = self.get_forward(self.imvol_spot, date)?;
            self.imvol_forward_vector[i] = fwd;
        }

        let given_strikes = market_implied_volatility_surface.get_strike();
        let given_spot_moneyness = given_strikes / self.imvol_spot;
        
        let bilinear_interpolator = BilinearInterpolator::new_from_rectangle_data(
            given_times,
            given_spot_moneyness,
            market_implied_volatility_surface.get_value().to_owned(),
            true,
            ExtraPolationType::Flat,
            true,
            ExtraPolationType::Flat,
        )?;

        self.imvol_maturity_dates = vega_structure_tenors.iter()
            .map(|tenor| add_period(&eval_date, tenor))
            .collect::<Vec<OffsetDateTime>>();

        if !self.imvol_maturity_dates.windows(2).all(|w| w[0] <= w[1]) {
            return Err(anyhow!(
                "({}:{}) Maturity dates of {} ({}) are not sorted: {:?}",
                file!(), line!(), self.name, self.code, self.imvol_maturity_dates
            ));
        }
        let mut tv = Vec::new();
        for tenor in vega_structure_tenors.iter() {
            tv.push(time_calculator.get_time_difference(&eval_date, &add_period(&eval_date, tenor)));
        }
        // sanity check if the time vector is sorted
        if !tv.windows(2).all(|w| w[0] <= w[1]) {
            return Err(anyhow!(
                "({}:{}) Time vector of {} ({}) are not sorted: {:?}",
                file!(), line!(), self.name, self.code, tv
            ));
        }
        self.imvol_maturity_times = Array1::from_vec(tv);
        self.imvol_spot_moneyness = vega_matrix_spot_moneyness;

        self.interpolated_imvol = Array2::zeros((self.imvol_maturity_times.len(), self.imvol_spot_moneyness.len()));
        for i in 0..self.imvol_maturity_times.len() {
            for j in 0..self.imvol_spot_moneyness.len() {
                self.interpolated_imvol[[i, j]] = bilinear_interpolator.interpolate(
                    self.imvol_maturity_times[i],
                    self.imvol_spot_moneyness[j],
                )?;
            }
        }

        Ok(self)
    }

    pub fn build(&mut self) -> Result<()> {
        let calculating_forward_vector = match self.stickyness_type {
            StickynessType::StickyToMoneyness => {
                let mut forward_vector: Vec<Real> = Vec::new();
                for i in 0..self.imvol_maturity_dates.len() {
                    let fwd = self.get_forward(
                        self.imvol_spot,
                        &self.imvol_maturity_dates[i])?;
                    forward_vector.push(fwd);
                }
                Array1::from_vec(forward_vector)
            },

            StickynessType::StickyToStrike => {
                let mut forward_vector: Vec<Real> = Vec::new();
                for i in 0..self.imvol_maturity_dates.len() {
                    let fwd = self.get_forward(
                        self.market_price.borrow().get_value(),
                        &self.imvol_maturity_dates[i])?;
                    forward_vector.push(fwd);
                }
                Array1::from_vec(forward_vector)
            },
        };

        // println!("forward_vector: {:?}", calculating_forward_vector);
        // println!("self.imvol_forward_vector: {:?}", self.imvol_forward_vector);
        // println!("imvol_maturity_dates: {:?}", self.imvol_maturity_dates);
        let mut forward_monenyess_array: Vec<Array1<Real>> = Vec::new();
        for i in 0..self.imvol_maturity_dates.len() {
            let fwd = calculating_forward_vector[i];
            forward_monenyess_array.push(&self.imvol_spot_moneyness * self.imvol_spot / fwd);
        }
        
        let mut f_interpolators: Vec<LinearInterpolator1D> = Vec::new();
        for i in 0..self.imvol_maturity_dates.len() {
            let interp = LinearInterpolator1D::new(
                forward_monenyess_array[i].to_owned(),
                self.interpolated_imvol.row(i).to_owned(),
                ExtraPolationType::Flat,
                true,
            ).with_context(|| anyhow!(
                "({}:{}) failed to create LinearInterpolator1D\n\
                forward_monenyess_array: {:?}\n\
                interpolated_imvol: {:?}",
                file!(), line!(),
                forward_monenyess_array[i].to_owned(),
                self.interpolated_imvol.row(i).to_owned()
            ))?;
            f_interpolators.push(interp);
        }


        self.forward_monenyess_imvol = BilinearInterpolator::new(
            self.imvol_maturity_times.clone(),
            f_interpolators,
            true,
            ExtraPolationType::Flat,
        )?;

        Ok(())
    }

    fn get_forward(&self, spot: Real, maturity: &OffsetDateTime) -> Result<Real> {
        let collateral_discount = self.collateral_curve
            .borrow()
            .get_discount_factor_at_date(maturity)
            .with_context(|| anyhow!(
                "({}:{}) failed to get collateral discount factor\n\
                maturity: {}, name: {}, code: {}",
                file!(), line!(),
                maturity, self.name, self.code
            ))?;
            
        let borrowing_discount = self.borrowing_curve
            .borrow()
            .get_discount_factor_at_date(maturity)
            .with_context(|| anyhow!(
                "({}:{}) failed to get borrowing discount factor\n\
                maturity: {}, name: {}, code: {}",
                file!(), line!(),
                maturity, self.name, self.code
            ))?;
            
        let dividend_deduction_ratio = self.market_price
            .borrow()
            .get_dividend_deduction_ratio(maturity)
            .with_context(|| anyhow!(
                "({}:{}) failed to get dividend deduction ratio\n\
                maturity: {}, name: {}, code: {}",
                file!(), line!(),
                maturity, self.name, self.code
            ))?;

        let fwd: Real = spot * borrowing_discount / collateral_discount * dividend_deduction_ratio;

        Ok(fwd)
    }

    pub fn get_imvol_matuirty_dates(&self) -> &Vec<OffsetDateTime> {
        &self.imvol_maturity_dates
    }

    pub fn get_imvol_maturity_times(&self) -> &Array1<Time> {
        &self.imvol_maturity_times
    }

    pub fn get_interpolated_imvol(&self) -> &Array2<Real> {
        &self.interpolated_imvol
    }

}

impl VolatilityTrait for LocalVolatilitySurface {
    fn get_value(&self, t: Time, forward_moneyness: Real) -> Real {
        self.forward_monenyess_imvol.interpolate(t, forward_moneyness)
            .expect("Failed to interpolate implied volatility")
        
    }

    fn get_local_volatility(&self, _t: Time, _forward_moneyness: Real) -> Real {
        panic!("Not implemented")
    }
    
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn total_variance(&self, t: Time, forward_moneyness: Real) -> Result<Real> {
        let iv = self.forward_monenyess_imvol.interpolate(t, forward_moneyness)
            .with_context(|| anyhow!(
                "({}:{}) failed to interpolate implied volatility\n\
                t: {}, forward_moneyness: {}, name: {}, code: {}",
                file!(), line!(),
                t, forward_moneyness, self.name, self.code
            ))?;

        Ok(iv*iv*t)
    }

    fn total_deviation(&self, t: Time, forward_moneyness: Real) -> Result<Real> {
        let iv = self.forward_monenyess_imvol.interpolate(t, forward_moneyness)
            .with_context(|| anyhow!(
                "({}:{}) failed to interpolate implied volatility\n\
                t: {}, forward_moneyness: {}, name: {}, code: {}",
                file!(), line!(),
                t, forward_moneyness, self.name, self.code
            ))?;

        Ok(iv * t.sqrt())
    }

    /// bump self.interpolated_imvol and remake forward_monenyess_imvol
    /// time1<=t<=time2, left_spot_moneyness<=x<=right_spot_moneyness
    fn bump_volatility(
        &mut self, 
        time1: Option<Time>,
        time2: Option<Time>,
        left_spot_moneyness: Option<Real>,
        right_spot_moneyness: Option<Real>,
        bump: Real
    ) -> Result<()> {
        let time1 = time1.unwrap_or(Time::MIN + 10.0);
        let time2 = time2.unwrap_or(Time::MAX - 10.0);
        let left_spot_moneyness = left_spot_moneyness.unwrap_or(Real::MIN + 10.0);
        let right_spot_moneyness = right_spot_moneyness.unwrap_or(Real::MAX - 10.0);

        let eps = 1.0e-4;
        let time_length = self.imvol_maturity_times.len();
        let spot_moneyness_length = self.imvol_spot_moneyness.len();
        for i in 0..time_length {
            let t = self.imvol_maturity_times[i];
            if !(time1 + eps < t && t <= time2 + eps) {
                continue;
            }
            for j in 0..spot_moneyness_length {
                let x = self.imvol_spot_moneyness[j];
                if !(left_spot_moneyness + eps < x && x <= right_spot_moneyness + eps) {
                    continue;
                }
                self.interpolated_imvol[[i, j]] += bump;
            }
        }
        self.build()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        surface_data::SurfaceData,
        vector_data::VectorData,
    };
    use crate::{
        vectordatasample,
        surfacedatasample,
    };
    use crate::parameters::volatilities::volatiltiy_interpolator::{AndreasenHuge, VolatilityInterplator};
   
    use crate::parameters::{
        volatility::VolatilityTrait,
        zero_curve::ZeroCurve,
    };
    use crate::evaluation_date::EvaluationDate;
    use crate::market_price::MarketPrice;
    use crate::enums::StickynessType;
    use crate::definitions::{Time, Real};
    use crate::currency::{self, Currency};
    use crate::utils;
    use crate::data;
    use std::{
        rc::Rc,
        cell::RefCell,
    };
    use ndarray::{Array1, Array2, prelude::*};
    use time::macros::datetime;
    use anyhow::Result;
    //
    #[test]
    fn test_equity_local_volatility_surface() -> Result<()> {
        let eval_date = datetime!(2024-01-02 00:00:00 +09:00);
        let spot = 350.0;

        let equity = Rc::new(
            RefCell::new(
                MarketPrice::new(
                    spot,
                    eval_date.clone(),
                    None,
                    Currency::KRW,
                    "KOSPI2".to_string(),
                    "KOSPI2".to_string(),
                )
            )
        );
        let evaluation_date = Rc::new(
            RefCell::new(EvaluationDate::new(eval_date.clone()))
        );

        let dummy_data = vectordatasample!(0.00, Currency::KRW, "mock curve data")?;
        let zero_curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &dummy_data,
                "KRWGOV".to_string(),
                "zero curve".to_string(),
            )?
        ));
        
        let surface_data = surfacedatasample!(&eval_date, spot);

        //println!("vector_data: {:?}", dummy_data);
        //println!("surface_data: {:?}", surface_data);

        let mut local_volatility_surface = LocalVolatilitySurface::initialize(
            evaluation_date.clone(),
            equity.clone(),
            zero_curve.clone(),
            zero_curve.clone(),
            StickynessType::default(),
            VolatilityInterplator::AndreasenHuge(AndreasenHuge::default()),
            "local vol".to_string(),
            "local vol".to_string(),
        );

        let vega_structure_tenors = vec!["1M", "2M", "3M", "6M", "9M", "1Y", "1Y6M", "2Y", "3Y"]
            .iter().map(|tenor| tenor.to_string()).collect::<Vec<String>>();

        let time_calculator = NullCalendar::new();
        let dates = surface_data.get_dates();
        let times: Vec<Time> = dates.iter().map(|date| time_calculator.get_time_difference(&eval_date, date))
            .collect::<Vec<Time>>().into();

        let vega_spot_moneyness = Array1::linspace(0.6, 1.4, 17);

        local_volatility_surface = local_volatility_surface.with_market_surface(
            &surface_data,
            vega_structure_tenors,
            vega_spot_moneyness.clone(),
        )?;

        local_volatility_surface.build()?;

        let mut calc_vol = Array2::zeros((times.len(), vega_spot_moneyness.len()));

        for i in 0..times.len() {
            for j in 0..vega_spot_moneyness.len() {
                calc_vol[[i, j]] = local_volatility_surface.get_value(
                    times[i], 
                    vega_spot_moneyness[j]
                );
            }
        }

        let original_value = surface_data.get_value();
        let original_value_sliced = original_value.slice(
            s![0..times.len(), 6..23]
        ).to_owned();

        let relative_error = &original_value_sliced - &calc_vol;
        let max_error = relative_error.mapv(|x| x.abs())
            .iter()
            .fold(0.0, |acc: Real, &x| acc.max(x));

        assert!(max_error < 1.0e-6, "max error: {}", max_error);

        println!("local_volatility_surface.get_value(2.5, 0.65): {}", local_volatility_surface.get_value(2.5, 0.65));
        local_volatility_surface.bump_volatility(
            Some(2.0),
            None,
            None,
            None,
            0.01
        )?;
        println!("after tail bump local_volatility_surface.get_value(2.5, 0.65): {}", local_volatility_surface.get_value(2.5, 0.65));
        let mut bumped_calc_vol1 = Array2::zeros((times.len(), vega_spot_moneyness.len()));

        for i in 0..times.len() {
            for j in 0..vega_spot_moneyness.len() {
                bumped_calc_vol1[[i, j]] = local_volatility_surface.get_value(
                    times[i], 
                    vega_spot_moneyness[j]
                );
            }
        }
        
        local_volatility_surface.bump_volatility(
            Some(1.5),
            Some(2.0),
            None,
            Some(0.7),
            0.01
        )?;

        println!("after tail bump local_volatility_surface.get_value(2.5, 0.65): {}", local_volatility_surface.get_value(2.5, 0.65));
        let mut bumped_calc_vol2 = Array2::zeros((times.len(), vega_spot_moneyness.len()));
        for i in 0..times.len() {
            for j in 0..vega_spot_moneyness.len() {
                bumped_calc_vol2[[i, j]] = local_volatility_surface.get_value(
                    times[i], 
                    vega_spot_moneyness[j]
                );
            }
        }
        let diff_8th = &bumped_calc_vol1.row(7) - &calc_vol.row(7);
        assert!(diff_8th.iter().all(|&x| x.abs() < 1.0e-6), "diff_8th: {:?}", diff_8th);
        let diff_9th = &bumped_calc_vol1.row(8) - &calc_vol.row(8);
        assert!(diff_9th.iter().all(|&x| (x - 0.01).abs() < 1.0e-6), "diff_9th: {:?}", diff_9th);

        let diff2_8th = &bumped_calc_vol2.row(7) - &calc_vol.row(7);
        for (i, x) in diff2_8th.iter().enumerate() {
            if i <= 2 {
                assert!((x - 0.01).abs() < 1.0e-6, "diff2_8th: {:?}", diff2_8th);
            } else {
                assert!(x.abs() < 1.0e-6, "diff2_8th: {:?}", diff2_8th);
            }
        }

        local_volatility_surface.bump_volatility(
            Some(2.0),
            None,
            None,
            None,
            -0.01
        )?;

        local_volatility_surface.bump_volatility(
            Some(1.5),
            Some(2.0),
            None,
            Some(0.7),
            -0.01
        )?;

        let mut bumped_calc_vol3 = Array2::zeros((times.len(), vega_spot_moneyness.len()));
        for i in 0..times.len() {
            for j in 0..vega_spot_moneyness.len() {
                bumped_calc_vol3[[i, j]] = local_volatility_surface.get_value(
                    times[i], 
                    vega_spot_moneyness[j]
                );
            }
        }

        let diff_last = &bumped_calc_vol3 - &calc_vol;
        let max_diff = diff_last.mapv(|x| x.abs())
            .iter()
            .fold(0.0, |acc: Real, &x| acc.max(x));

        assert!(max_diff < 1.0e-6, "diff_last: {:?}", diff_last);   

        Ok(())
    }
}