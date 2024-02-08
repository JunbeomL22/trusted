#![allow(dead_code)]
// all takend from https://github.com/avhz/RustQuant/blob/main/src/time/conventions.rs
pub enum BusinessDayConvention {
    /// Actual: paid on the actual day, even if it is a non-business day.
    Unadjusted,

    /// Following business day: the payment date is rolled to the next business day.k
    Following,

    /// Modified following business day: the payment date is rolled to the
    /// next business day, unless doing so
    /// would cause the payment to be in the next calendar month,
    /// in which case the payment date is rolled to the previous business day.
    /// Many institutions have month-end accounting procedures that necessitate this.
    ModifiedFollowing,

    /// Previous business day: the payment date is rolled to the previous business day.
    Preceding,

    /// Modified previous business day: the payment date is rolled to the previous
    /// business day, unless doing so would cause the payment to be in the previous
    /// calendar month, in which case the payment date is rolled to the next
    /// business day. Many institutions have month-end accounting procedures
    /// that necessitate this.
    ModifiedPreceding,

    // Modified Rolling business day: the payment date is rolled to the next
    // business day. The adjusted week date is used for the next coupon date.
    // So adjustments are cumulative (excluding month change).
    //ModifiedRolling,
}

/// Day count conventions.
///
/// From Wikipedia (<https://en.wikipedia.org/wiki/Day_count_convention>):
/// """
/// In finance, a day count convention determines how interest accrues
/// over time for a variety of investments, including bonds, notes,
/// loans, mortgages, medium-term notes, swaps, and forward rate agreements (FRAs).
/// This determines the number of days between two coupon payments,
/// thus calculating the amount transferred on payment dates and also the
/// accrued interest for dates between payments. The day count is also
/// used to quantify periods of time when discounting a cash-flow to its
/// present value. When a security such as a bond is sold between interest
/// payment dates, the seller is eligible to some fraction of the coupon amount.
/// """
pub enum DayCountConvention {
    // TODO: Implement the following day count conventions.
    // There are fiddly techicalities to consider, such as leap years.
    // Also need some sort of calendar to determine which days are holidays, etc.
    // Thirty360_BondBasis,
    // Thirty360_US,
    // ThirtyE360,
    // ThirtyE360_ISDA,
    // ActualActual_ICMA,
    // ActualActual_ISDA,
    // Actual365L,
    // ActualActual_AFB,
    // OneOne,
    //
    /// Actual/365 day count convention.
    Actual365Fixed,
    /// Actual/360 day count convention.
    Actual360,
    /// Actual/364 day count convention.
    Actual364,
    /// Thirty/360 day count convention.
    Thirty360,
    /// Actual/Actual day count convention.
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