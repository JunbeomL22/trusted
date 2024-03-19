use crate::assets::currency::Currency;
use crate::definitions::Real;
use crate::instruments::schedule::Schedule;
use crate::parameters::rate_index::RateIndex;
use crate::enums::{IssuerType, CreditRating, RankType};
use crate::instrument::InstrumentTriat;
//
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingRateNote {
    currency: Currency,
    issuer_type: IssuerType,
    credit_rating: CreditRating,
    rank: RankType,
    schedule: Schedule,
    spread: Real,
    rate_index: RateIndex,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    issuer_name: String,
    name: String,
    code: String,
}

impl InstrumentTriat for FloatingRateNote {
    fn get_type_name(&self) -> &'static str {
        "FloatingRateNote"
    }

    fn get_issuer_name(&self) -> Result<&String> {
        Ok(&self.issuer_name)
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) ->  &Currency {
        &self.currency
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }
}