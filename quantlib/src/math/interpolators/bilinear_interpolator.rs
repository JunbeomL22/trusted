use crate::definitions::Real;
use crate::utils::find_index_ndarray::binary_search_index_ndarray;
use crate::math::{
    interpolator::{InterpolatorReal1D, ExtraPolationType},
    interpolators::linear_interpolator::LinearInterpolator1D,
};
use crate::util::is_ndarray_sorted;
use ndarray::{Array1, Array2};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct BilinearInterpolator
{
    t_domain: Array1<Real>,
    x_interpolator: Vec<LinearInterpolator1D>,
    t_domain_allow_extrapolation: bool,
    t_domain_extrapolation_type: ExtraPolationType,
}

impl Default for BilinearInterpolator
{
    fn default() -> BilinearInterpolator {
        BilinearInterpolator {
            t_domain: Array1::default(0),
            x_interpolator: Vec::new(),
            t_domain_allow_extrapolation: false,
            t_domain_extrapolation_type: ExtraPolationType::Flat,
        }
    }
}

impl BilinearInterpolator {
    pub fn new(
        t_domain: Array1<Real>,
        x_interpolator: Vec<LinearInterpolator1D>,
        t_domain_allow_extrapolation: bool,
        t_domain_extrapolation_type: ExtraPolationType,
    ) -> Result<BilinearInterpolator> {
        let n = t_domain.len();
        if n != x_interpolator.len() {
            return Err(anyhow!(
                "({}:{}) t_domain and x_interpolator must have the same length\n\
                t_domain: {:?}\n\
                x_interpolator: {:?}",
                file!(), line!(), t_domain, x_interpolator
            ));
        }
        if !is_ndarray_sorted(&t_domain) {
            return Err(anyhow!(
                "({}:{}) t_domain must be sorted: \n{:?}",
                file!(), line!(), 
                &t_domain
            ));
        }
        Ok(BilinearInterpolator {
            t_domain,
            x_interpolator,
            t_domain_allow_extrapolation,
            t_domain_extrapolation_type,
        })
    }

    pub fn new_from_rectangle_data(
        t_domain: Array1<Real>,
        x_domain: Array1<Real>,
        values: Array2<Real>,
        x_domain_allow_extrapolation: bool,
        x_domain_extrapolation_type: ExtraPolationType,
        t_domain_allow_extrapolation: bool,
        t_domain_extrapolation_type: ExtraPolationType,
    ) -> Result<BilinearInterpolator> {
        let n = t_domain.len();
        let m = x_domain.len();

        if n < 2 || m < 2 {
            return Err(anyhow!(
                "({}:{}) t_domain and x_domain must have at least 2 elements\n\
                t_domain: {:?}\n\
                x_domain: {:?}",
                file!(), line!(), t_domain, x_domain
            ));
        }

        if n != values.shape()[0] {
            return Err(anyhow!(
                "({}:{}) t_domain and z_values must have the same length\n\
                t_domain: {:?}\n\
                z_values: {:?}",
                file!(), line!(), t_domain, values
            ));
        }
        if m != values.shape()[1] {
            return Err(anyhow!(
                "({}:{}) x_domain and z_values must have the same length\n\
                x_domain: {:?}\n\
                z_values: {:?}",
                file!(), line!(), x_domain, values
            ));
        }
        let mut x_interpolator: Vec<LinearInterpolator1D> = Vec::new();
        for i in 0..n {
            /*
            println!(
                "({}:{}) x_domain: {:?} \nvalues.row(i): {:?}\n", 
                file!(), line!(),
                x_domain.clone(),
                values.row(i).to_owned());
            
            let x_values = values.row(i); //Array1::from_vec(values.row(i).to_vec());
             */
            let x_interpolator_i = LinearInterpolator1D::new(
                x_domain.clone(),
                values.row(i).to_owned(),
                x_domain_extrapolation_type,
                x_domain_allow_extrapolation,
            )?;
            x_interpolator.push(x_interpolator_i);
        }
        Ok(BilinearInterpolator {
            t_domain,
            x_interpolator,
            t_domain_allow_extrapolation,
            t_domain_extrapolation_type,
        })
    }

    pub fn interpolate(&self, t: Real, x: Real) -> Result<Real> {
        let n = self.t_domain.len();
        if t < self.t_domain[0] {
            if self.t_domain_allow_extrapolation {
                match self.t_domain_extrapolation_type {
                    ExtraPolationType::Flat => Ok(self.x_interpolator[0].interpolate(x)?),
                    ExtraPolationType::Linear => {
                        let v_first = self.x_interpolator[0].interpolate(x)?;
                        let v_second = self.x_interpolator[1].interpolate(x)?;
                        let t_first = self.t_domain[0];
                        let t_second = self.t_domain[1];
                        Ok(v_first + (t - t_first) * (v_second - v_first) / (t_second - t_first))
                    },
                    ExtraPolationType::None => Err(anyhow!(
                        "({}:{}) {}: extrapolation has not been implemented yet",
                        file!(),
                        line!(),
                        self.t_domain_extrapolation_type
                    )),
                }
            } else {
                Err(anyhow!(
                    "({}:{}) t (={}) is out of range where\n\
                    t_domain: Array1<Real> = {:?}",
                    file!(),
                    line!(),
                    t,
                    self.t_domain
                ))
            }
        } else if t >= self.t_domain[n - 1] {
            if self.t_domain_allow_extrapolation {
                match self.t_domain_extrapolation_type {
                    ExtraPolationType::Flat => Ok(self.x_interpolator[n - 1].interpolate(x)?),
                    ExtraPolationType::Linear => {
                        let v_last = self.x_interpolator[n - 1].interpolate(x)?;
                        let v_prev = self.x_interpolator[n - 2].interpolate(x)?;
                        let t_last = self.t_domain[n - 1];
                        let t_prev = self.t_domain[n - 2];
                        return Ok(v_last + (t - t_last) * (v_last - v_prev) / (t_last - t_prev));
                    },
                    ExtraPolationType::None => return Err(anyhow!(
                        "({}:{}) t (={}) is out of range where\n\
                        t_domain: Array1<Real> = {:?}",
                        file!(),
                        line!(),
                        t,
                        self.t_domain
                    )),
                }
            } else {
                return Err(anyhow!(
                    "({}:{}) t (={}) is out of range where\n\
                    t_domain: Array1<Real> = {:?}",
                    file!(),
                    line!(),
                    t,
                    self.t_domain
                ));
            }
        } else {
            let t_index = binary_search_index_ndarray(&self.t_domain, t);
            let t_prev = self.t_domain[t_index];
            let t_next = self.t_domain[t_index + 1];
            let v_prev = self.x_interpolator[t_index].interpolate(x)?;
            let v_next = self.x_interpolator[t_index + 1].interpolate(x)?;

            return Ok(v_prev + (t - t_prev) * (v_next - v_prev) / (t_next - t_prev));
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
    use crate::math::interpolator::ExtraPolationType;
    use ndarray::array;

    #[test]
    fn test_bilinear_interpolator() {
        let t_domain = array![0.0, 1.0, 2.0];
        let x_domain = array![0.0, 1.0, 2.0];
        let values = array![[0.0, 1.0, 2.0], [1.0, 2.0, 3.0], [2.0, 3.0, 4.0]];
        let x_interpolator = vec![
            LinearInterpolator1D::new(
                x_domain.clone(),
                Array1::from_vec(values.row(0).to_vec()),
                ExtraPolationType::Flat,
                true,
            )
            .unwrap(),
            LinearInterpolator1D::new(
                x_domain.clone(),
                Array1::from_vec(values.row(1).to_vec()),
                ExtraPolationType::Flat,
                true,
            )
            .unwrap(),
            LinearInterpolator1D::new(
                x_domain.clone(),
                Array1::from_vec(values.row(2).to_vec()),
                ExtraPolationType::Flat,
                true,
            )
            .unwrap(),
        ];
        let bilinear_interpolator = BilinearInterpolator::new(
            t_domain.clone(),
            x_interpolator,
            true,
            ExtraPolationType::Flat,
        )
        .unwrap();
        assert_eq!(bilinear_interpolator.interpolate(0.5, 0.5).unwrap(), 1.0);
        assert_eq!(bilinear_interpolator.interpolate(1.5, 1.5).unwrap(), 3.0);
        assert_eq!(bilinear_interpolator.interpolate(0.5, 1.5).unwrap(), 2.0);
        assert_eq!(bilinear_interpolator.interpolate(1.5, 0.5).unwrap(), 2.0);
    }

    #[test]
    fn test_new_from_rectangle() -> Result<()> {
        let t_domain = array![0.0, 1.0, 2.0];
        let x_domain = array![0.0, 1.0, 2.0];
        let values = array![[0.0, 1.0, 2.0], [1.0, 2.0, 3.0], [2.0, 3.0, 4.0]];
        let bilinear_interpolator = BilinearInterpolator::new_from_rectangle_data(
            t_domain.clone(),
            x_domain.clone(),
            values,
            true,
            ExtraPolationType::Flat,
            true,
            ExtraPolationType::Flat,
        )?;
        
        assert_eq!(bilinear_interpolator.interpolate(-1.0, 0.5).unwrap(), 0.5);
        assert_eq!(bilinear_interpolator.interpolate(0.5, 0.5).unwrap(), 1.0);
        assert_eq!(bilinear_interpolator.interpolate(1.5, 1.5).unwrap(), 3.0);
        assert_eq!(bilinear_interpolator.interpolate(0.5, 1.5).unwrap(), 2.0);
        assert_eq!(bilinear_interpolator.interpolate(1.5, 0.5).unwrap(), 2.0);
        Ok(())
    }
}
