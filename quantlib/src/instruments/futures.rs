use crate::definitions::{Real, Time};
use time::OffsetDateTime;

pub struct Futures {
    strike: Real,
    maturity: OffsetDateTime,
    notional: Real,
    underlying_asset: Vec<String>,
}