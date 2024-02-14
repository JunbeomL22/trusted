use crate::parameters::enums::ZeroCurveCode;
use crate::parameter::EvaluationDate;
use crate::data::vector_data::VectorData;
use crate::definitions::{Real, Time};
use crate::parameter::Parameter;
use crate::math::interpolators::linear_interpolator::LinearInterpolator1D;
use crate::math::interpolator::InterpolatorReal1D;
use crate::math::interpolator::ExtraPolationType;
//use time::OffsetDateTime;
/// ZeroCurve is a curve of zero rates which implements Parameter (Observer) trait.
/// Input is a vector of dates and a vector of zero rates of Data (observable) type.
/// when the zero rates are updated, the zero curve will be updated.
pub struct ZeroCurve {
    evaluation_date: EvaluationDate,
    data: VectorData,
    discount_times: Vec<Time>,
    discount_factors: Vec<Real>,
    discount_interpolator: LinearInterpolator1D,
    code: ZeroCurveCode,
    name: String,
}

impl ZeroCurve {
    pub fn new(evaluation_date: EvaluationDate, data: VectorData, extrapolate_type: ExtraPolationType, code: ZeroCurveCode, name: String) -> ZeroCurve {
        let discount_times = data.get_times();
        let discount_factors = data.get_value();
        let discount_interpolator = linear_interpolator::LinearInterpolator1D::new(discount_times.clone(),
         discount_factors.clone(), 
         extrapolate_type,
         true);
        ZeroCurve {
            evaluation_date,
            data,
            discount_times,
            discount_factors,
            discount_interpolator,
            code,
            name,
        }
    }
    pub fn get_discount_factor(&self, time: Time) -> Real {
        self.discount_interpolator.interpolate(time)
    }
    pub fn get_discount_factors(&self) -> Vec<Real> {
        self.discount_factors.clone()
    }
    pub fn get_discount_times(&self) -> Vec<Time> {
        self.discount_times.clone()
    }
    pub fn get_code(&self) -> ZeroCurveCode {
        self.code
    }

    pub fn get_data(&self) -> &VectorData {
        &self.data
    }

    pub fn get_evaluation_date(&self) -> EvaluationDate {
        self.evaluation_date
    }
    pub fn set_data(&mut self, data: VectorData) {
        self.data = data;
        self.update();
    }
}
impl Parameter for ZeroCurve {
    fn update(&mut self) {
        self.discount_times = self.data.get_times();
        self.discount_factors = self.data.get_value();
        self.discount_interpolator = LinearInterpolator1D::new(self.discount_times.clone(), self.discount_factors.clone());
    }
}