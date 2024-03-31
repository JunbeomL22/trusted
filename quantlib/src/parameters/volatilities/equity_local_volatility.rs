use crate::enums::StickynessType;
use crate::definitions::{Time, Real};
use crate::data::surface_data::SurfaceData;
use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
use crate::parameters::{
    volatility::VolatilityTrait,
    volatilities::volatiltiy_interpolator::VolatilityInterplator,
    zero_curve::ZeroCurve,
};
use crate::evaluation_date::{self, EvaluationDate};
use crate::math::interpolators::bilinear_interpolator::{self, BilinearInterpolator};
use crate::assets::equity::Equity;
use crate::time::calendar_trait::CalendarTrait;
use crate::time::calendars::nullcalendar::NullCalendar;
use crate::utils::string_arithmetic::from_period_string_to_float;
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
pub struct EquityLocalVolatilitySurface {
    interpolated_imvol: Array2<Real>,
    imvol_maturity_dates: Vec<OffsetDateTime>,
    imvol_maturity_times: Array1<Time>,
    imvol_spot_moneyness: Array1<Real>,
    imvol_forward_vector: Array1<Real>,
    imvol_spot: Real,
    forward_monenyess_imvol: BilinearInterpolator,
    //
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    underlying_equity: Rc<RefCell<Equity>>,
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

impl EquityLocalVolatilitySurface {
    pub fn initialize(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        underlying_equity: Rc<RefCell<Equity>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
        stickyness_type: StickynessType,
        lv_interpolator: VolatilityInterplator,
        local_volatility: BilinearInterpolator,
        name: String,
        code: String,
    ) -> EquityLocalVolatilitySurface {
        EquityLocalVolatilitySurface {
            interpolated_imvol: Array2::default((0, 0)),
            imvol_maturity_dates: Vec::new(),
            imvol_maturity_times: Array1::default(0),
            imvol_spot_moneyness: Array1::default(0),
            imvol_forward_vector: Array1::default(0),
            forward_monenyess_imvol: BilinearInterpolator::default(),
            imvol_spot: 0.0,
            //
            evaluation_date,
            underlying_equity,
            collateral_curve,
            borrowing_curve,
            //
            stickyness_type,
            lv_interpolator,
            local_volatility,
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
    ) -> Result<EquityLocalVolatilitySurface> {
        let given_dates = market_implied_volatility_surface.get_dates();
        let time_calculator = NullCalendar::new();
        let eval_date = self.evaluation_date.borrow().get_date();
        let given_times: Array1<Time> = given_dates.iter()
            .map(|date| time_calculator.get_time_difference(eval_date, date))
            .collect::<Vec<Time>>().into();

        self.imvol_spot = market_implied_volatility_surface.get_spot()
            .ok_or_else(|| anyhow!(
                "({}:{}) Error getting spot from market_implied_volatility_surface of {}",
                file!(), line!(), market_implied_volatility_surface.get_name()
            ))?;

        let given_strikes = market_implied_volatility_surface.get_strike();
        let given_spot_moneyness = given_strikes / self.imvol_spot;
        
        let bilinear_interpolator = BilinearInterpolator::new_from_rectangle_data(
            given_times,
            given_spot_moneyness,
            market_implied_volatility_surface.get_value().to_owned(),
            false,
            ExtraPolationType::Flat,
            false,
            ExtraPolationType::Flat,
        )?;

        self.imvol_maturity_dates = vega_structure_tenors.iter()
            .map(|tenor| add_period(eval_date, tenor))
            .collect::<Vec<OffsetDateTime>>();

        if !self.imvol_maturity_dates.windows(2).all(|w| w[0] <= w[1]) {
            return Err(anyhow!(
                "({}:{}) Maturity dates of {} ({}) are not sorted: {:?}",
                file!(), line!(), self.name, self.code, self.imvol_maturity_dates
            ));
        }
        let mut tv = Vec::new();
        for tenor in vega_structure_tenors.iter() {
            tv.push(time_calculator.get_time_difference(eval_date, &add_period(eval_date, tenor)));
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

        self.set_forward_moneyness_volatility()?;

        Ok(self)
    }

    fn set_forward_moneyness_volatility(&mut self) -> Result<()> {
        let calculating_forward_vector = match self.stickyness_type {
            StickynessType::StickyToMoneyness => self.imvol_forward_vector.clone(),
            StickynessType::StickyToStrike => {
                let mut forward_vector: Vec<Real> = Vec::new();
                for i in 0..self.imvol_maturity_dates.len() {
                    let fwd = self.get_forward_vector(
                        self.underlying_equity.borrow().get_last_price(),
                        &self.imvol_maturity_dates[i])?;
                    forward_vector.push(fwd);
                }
                Array1::from_vec(forward_vector)
            },
        };

        let mut forward_monenyess_array: Vec<Array1<Real>> = Vec::new();
        for i in 0..self.imvol_maturity_dates.len() {
            let fwd = calculating_forward_vector[i];
            forward_monenyess_array[i] = self.imvol_spot_moneyness * self.imvol_spot / fwd;
        }

        let f_interpolators: Vec<LinearInterpolator1D> = (0..self.imvol_maturity_dates.len())
            .map(|i| LinearInterpolator1D::new(
                forward_monenyess_array[i],
                self.interpolated_imvol.row(i).to_owned(),
                ExtraPolationType::Flat,
                true,
            ))
            .collect::<Result<Vec<LinearInterpolator1D>>>()?;

        self.forward_monenyess_imvol = BilinearInterpolator::new(
            self.imvol_maturity_times,
            f_interpolators,
            true,
            ExtraPolationType::Flat,
        )?;

        Ok(())
    }

    fn get_forward_vector(&self, spot: Real, maturity: &OffsetDateTime) -> Result<Real> {
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
            
        let dividend_deduction_ratio = self.underlying_equity
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
}

impl VolatilityTrait for EquityLocalVolatilitySurface {
    fn get_value(&self, t: Time, forward_moneyness: Real) -> Real {
        self.forward_monenyess_imvol.interpolate(t, forward_moneyness)
            .expect("Failed to interpolate implied volatility")
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
        let time1 = time1.unwrap_or(Real::MIN);
        let time2 = time2.unwrap_or(Real::MAX);
        let left_spot_moneyness = left_spot_moneyness.unwrap_or(Real::MIN);
        let right_spot_moneyness = right_spot_moneyness.unwrap_or(Real::MAX);

        let time_length = self.imvol_maturity_times.len();
        let spot_moneyness_length = self.imvol_spot_moneyness.len();
        for i in 0..time_length {
            let t = self.imvol_maturity_times[i];
            if t <= time1 || t > time2 {
                continue;
            }
            for j in 0..spot_moneyness_length {
                let x = self.imvol_spot_moneyness[j];
                if x <= left_spot_moneyness || x > right_spot_moneyness {
                    continue;
                }
                self.interpolated_imvol[[i, j]] += bump;
            }
        }

        self.set_forward_moneyness_volatility()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::surface_data::SurfaceData;
    use crate::parameters::volatilities::volatiltiy_interpolator::VolatilityInterplator;
    use crate::math::interpolators::bilinear_interpolator::BilinearInterpolator;
    use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
    use crate::parameters::volatility::VolatilityTrait;
    use crate::parameters::volatilities::equity_local_volatility::EquityLocalVolatilitySurface;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::evaluation_date::EvaluationDate;
    use crate::assets::equity::Equity;
    use crate::time::calendars::nullcalendar::NullCalendar;
    use crate::enums::StickynessType;
    use crate::math::interpolator::ExtraPolationType;
    use crate::utils::string_arithmetic::from_period_string_to_float;
    use crate::utils::string_arithmetic::add_period;
    use crate::definitions::{Time, Real};
    use std::rc::Rc;
    use std::cell::RefCell;
    use ndarray::{Array1, Array2};
    use time::OffsetDateTime;
    use anyhow::Result;
    #[test]
    fn test_equity_local_volatility_surface() -> Result<()> {
        