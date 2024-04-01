use crate::definitions::Real;
use crate::currency::Currency;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use ndarray::{Array1, Array2};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurfaceData {
    spot: Option<Real>,
    value: Array2<Real>,
    dates: Vec<OffsetDateTime>,
    strikes: Array1<Real>,
    market_datetime: Option<OffsetDateTime>,
    currency: Currency,
    name: String,
    code: String,
}

impl SurfaceData {
    pub fn new(
        spot: Option<Real>,
        value: Array2<Real>,
        dates: Vec<OffsetDateTime>,
        strikes: Array1<Real>,
        market_datetime: Option<OffsetDateTime>,
        currency: Currency,
        name: String,
        code: String,
    ) -> SurfaceData {
        SurfaceData {
            spot,
            value,
            dates,
            strikes,
            market_datetime,
            currency,
            name,
            code,
        }
    }

    pub fn get_spot(&self) -> Option<Real> {
        self.spot
    }

    pub fn get_value(&self) -> &Array2<Real> {
        &self.value
    }

    pub fn get_dates(&self) -> &Vec<OffsetDateTime> {
        &self.dates
    }

    pub fn get_market_datetime(&self) -> Option<OffsetDateTime> {
        self.market_datetime
    }

    pub fn get_strike(&self) -> &Array1<Real> {
        &self.strikes
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }
}
