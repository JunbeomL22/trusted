use crate::parameters::enums::ZeroCurveCode;
/// ZeroCurve is a curve of zero rates which implements Parameter (Observer) trait.
/// Input is a vector of dates and a vector of zero rates of Data (observable) type.
/// when the zero rates are updated, the zero curve will be updated.
pub struct ZeroCurve {
    dates: Vec<Date>,
    zero_rates: Data,
    name : ZeroCurveCode,
}