use ndarray::{Array1, Array2};
use anyhow::{Result, anyhow};
use crate::definitions::Real;

pub trait Volatility {
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_total_variance(
        &self, 
        forward_moneyness: Real, 
        expiry: Real) -> Result<Real> {
            Err(anyhow!("({}:{}) Not implemented", file!(), line!()))
        }
        
    fn get_vol_on_forward(
        &self, 
        forward_moneyness: Real, 
        expiry: Real) -> Result<Real> {
            Err(anyhow!("({}:{}) Not implemented", file!(), line!()))
        }
}

pub struct ConstantVolatility {
    vol: Real,
    name: String,
    code: String,
}

pub struct SurfaceVolatility {
    spot_moneyness: Array1<Real>,
    expiries: Array1<Real>,
    vols: Array2<Real>,
    forward_moneyness: Option<Array1<Real>>,
    vols_on_forward: Option<Array2<Real>>,
    name: String,
    code: String,
}

pub enum LocalVolatility {
    ConstantVolatility(ConstantVolatility),
    SurfaceVolatility(SurfaceVolatility),
}
