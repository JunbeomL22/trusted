use crate::types::{
    isin_code::IsinCode,
    venue::Venue,
};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Spread {
    isin_code: IsinCode,
    venue: Venue,
    near_code: IsinCode,
    far_code: IsinCode,
    maturity: OffsetDateTime,
    unit_amount: f64,
}