#![allow(dead_code)]
use crate::definitions::{Integer, Real};
use serde::{Deserialize, Serialize};
// all takend from https://github.com/avhz/RustQuant/blob/main/src/time/conventions.rs

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum BusinessDayConvention {
    Unadjusted,
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
    Dummy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub enum DayCountConvention {
    Actual365Fixed,
    Actual360,
    Actual364,
    Thirty360,
    ActActIsda,
    StreetConvention, // 30/360 but with EOM
    Dummy,
}

/// Interest payment frequency/year enumeration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PaymentFrequency {
    Daily = 252,
    Weekly = 52,
    BiWeekly = 26,
    SemiMonthly = 24,
    Monthly = 12,
    SemiQuarterly = 6,
    Quarterly = 4,
    TriAnnually = 3,
    SemiAnnually = 2,
    Annually = 1,
    None = 0,
}

impl PaymentFrequency {
    pub fn as_real(&self) -> Real {
        *self as Integer as Real
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            PaymentFrequency::Daily => "1D",
            PaymentFrequency::Weekly => "1W",
            PaymentFrequency::BiWeekly => "2W",
            PaymentFrequency::SemiMonthly => "2W",
            PaymentFrequency::Monthly => "1M",
            PaymentFrequency::SemiQuarterly => "2M",
            PaymentFrequency::Quarterly => "3M",
            PaymentFrequency::TriAnnually => "4M",
            PaymentFrequency::SemiAnnually => "6M",
            PaymentFrequency::Annually => "1Y",
            PaymentFrequency::None => "None",
        }
    }

    pub fn to_string_with_multiple(&self, multiple: Integer) -> String {
        let str_type = self.as_str();
        // take the head before "D|W|M|Y"
        let (head, _) = str_type.split_at(str_type.len() - 1);
        // change head to interger and multiply the multiple
        let head = (head.parse::<Integer>().unwrap() * multiple).to_string();
        // append the tail
        let tail = &str_type[str_type.len() - 1..];
        // concat the head and tail
        let result = head + tail;
        result
    }
}

impl ToString for PaymentFrequency {
    fn to_string(&self) -> String {
        match *self {
            PaymentFrequency::Daily => "1D".to_string(),
            PaymentFrequency::Weekly => "1W".to_string(),
            PaymentFrequency::BiWeekly => "2W".to_string(),
            PaymentFrequency::SemiMonthly => "2W".to_string(),
            PaymentFrequency::Monthly => "1M".to_string(),
            PaymentFrequency::SemiQuarterly => "2M".to_string(),
            PaymentFrequency::Quarterly => "3M".to_string(),
            PaymentFrequency::TriAnnually => "4M".to_string(),
            PaymentFrequency::SemiAnnually => "6M".to_string(),
            PaymentFrequency::Annually => "1Y".to_string(),
            PaymentFrequency::None => "None".to_string(),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_payment_frequency() {
        let freq = PaymentFrequency::Quarterly;
        assert_eq!(freq.to_string_with_multiple(3), "9M");
    }
}