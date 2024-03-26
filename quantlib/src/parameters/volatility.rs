use ndarray::{Array1, Array2};
use anyhow::{Result, anyhow};
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

impl ImpliedSurface {
    pub fn new(
        strikes: Array1<Real>,
        expiries: Array1<Real>,
        vols: Array2<Real>,
        name: String,
        code: String,
    ) -> VolatilitySurface {
        VolatilitySurface {
            strikes,
            expiries,
            vols,
            name,
            code,
            None,
            None,
        }
    }

    pub fn get_strikes(&self) -> &Array1<Real> {
        &self.strikes
    }

    pub fn get_expiries(&self) -> &Array1<Real> {
        &self.expiries
    }

    pub fn get_vols(&self) -> &Array2<Real> {
        &self.vols
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_vol_on_forward_value(&self, strike: Real, expiry: Real) -> Result<Real> {
        // interpolate the vol on the forward value
        // using the strikes and expiries
        // and the vols
        // return the interpolated vol
    }
}