use crate::assets::currency::Currency;
use crate::definitions::Real;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::instruments::schedule::Schedule;
use crate::parameters::rate_index::RateIndex;
use crate::enums::{IssuerType, CreditRating, RankType};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRateNote<'a> {
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
    issuer_name: &'a str,
    name: &'a str,
    code: &'a str,
}

impl<'a> FloatingRateNote<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_code(&self) -> &str {
        self.code
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    pub fn get_credit_rating(&self) -> Option<&CreditRating> {
        Some(&self.credit_rating)
    }

    pub fn get_issuer_type(&self) -> Option<&IssuerType> {
        Some(&self.issuer_type)
    }

    pub fn get_rank(&self) -> Option<&RankType> {
        Some(&self.rank)
    }

    pub fn get_issuer_name(&self) -> Option<&str> {
        Some(&self.issuer_name)
    }

    pub fn get_spread(&self) -> Real {
        self.spread
    }

}