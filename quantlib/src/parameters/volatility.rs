use ndarray::{Array1, Array2};


pub struct ConstantVolatility {
    value: Real,
}

pub struct VolatilitySurface {
    spot_moneyness: Array1<Real>,
    expiries: Array1<Real>,
    vols: Array2<Real>,
    name: String,
    code: String,
}

impl VolatilitySurface {
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