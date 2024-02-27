#![allow(dead_code)]
// all takend from https://github.com/avhz/RustQuant/blob/main/src/time/conventions.rs
pub enum BusinessDayConvention {
    Unadjusted,
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
}
pub enum DayCountConvention {
    Actual365Fixed,
    Actual360,
    Actual364,
    Thirty360,
    ActActIsda,
}

/// Interest payment frequency/year enumeration.
#[derive(Debug, Clone, Copy)]
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
    Annually = 1
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
        }
    }
}