use crate::parameters::enums::ZeroCurveCode;
use crate::parameter::{EvaluationDate, CurveData};
use crate::definitions::Real;
use time::OffsetDateTime;
/// ZeroCurve is a curve of zero rates which implements Parameter (Observer) trait.
/// Input is a vector of dates and a vector of zero rates of Data (observable) type.
/// when the zero rates are updated, the zero curve will be updated.
pub struct ZeroCurve {
    evaluation_date: EvaluationDate,
    dates: Vec<OffsetDateTime>,
    times: Vec<Real>,
    zero_rates: CurveData,
    name : ZeroCurveCode,
}