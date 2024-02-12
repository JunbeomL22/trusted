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