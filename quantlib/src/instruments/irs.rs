use crate::assets::currency::Currency;
use crate::definitions::Real;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::parameters::rate_index::RateIndex;
use crate::instruments::schedule::{self, Schedule};
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::time::calendar::Calendar;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IRS {
    fixed_legs: Schedule,
    floating_legs: Schedule,
    //
    currency: Currency,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    fixed_rate: Real,
    rate_index: RateIndex,
    //
    fixed_daycounter: DayCountConvention,
    floating_daycounter: DayCountConvention,
    //
    fixed_busi_convenction: BusinessDayConvention,
    floating_busi_convenction: BusinessDayConvention,
    //
    fixed_frequency: PaymentFrequency,
    floating_frequency: PaymentFrequency,
    //
    calendar: Calendar,
    name: String,
    code: String,
}

impl IRS {
    pub fn new(
        fixed_legs: Schedule,
        floating_legs: Schedule,
        currency: Currency,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        maturity: OffsetDateTime,
        fixed_rate: Real,
        rate_index: RateIndex,
        fixed_daycounter: DayCountConvention,
        floating_daycounter: DayCountConvention,
        fixed_busi_convention: BusinessDayConvention,
        floating_busi_convention: BusinessDayConvention,
        fixed_frequency: PaymentFrequency,
        floating_frequency: PaymentFrequency,
        calendar: Calendar,
        name: String,
        code: String,
    ) -> IRS {
        IRS {
            fixed_legs,
            floating_legs,
            currency,
            unit_notional,
            issue_date,
            maturity,
            fixed_rate,
            rate_index,
            fixed_daycounter,
            floating_daycounter,
            fixed_busi_convention,
            floating_busi_convention,
            fixed_frequency,
            floating_frequency,
            calendar,
            name,
            code,
        }
    }

    /// construct IRS using PaymentFrequency, BusinessDayConvention, DayCountConvention
    /// without fixed/floating - legs given directly
    pub fn new_from_conventions(
        currency: Currency,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        maturity: OffsetDateTime,
        fixed_rate: Real,
        rate_index: RateIndex,
        fixed_daycounter: DayCountConvention,
        floating_daycounter: DayCountConvention,
        fixed_busi_convention: BusinessDayConvention,
        floating_busi_convention: BusinessDayConvention,
        fixed_frequency: PaymentFrequency,
        floating_frequency: PaymentFrequency,
        fixing_days: i64,
        payment_days: i64,
        calendar: Calendar,
        name: String,
        code: String,
    ) -> IRS {
        let fixed_legs = schedule::build_schedule(
            &issue_date,
            None,
            &maturity,
            &calendar,
            &fixed_busi_convention,
            &fixed_frequency,
            fixing_days,
            payment_days,
        ).expect("Failed to build fixed legs");

        let floating_legs = schedule::build_schedule(
            &issue_date,
            None,
            &floating_frequency,
            &calendar,
            &floating_busi_convenction,
            &floating_frequency,
            fixing_days,
            payment_days,
        ).expect("Failed to build floating legs");

        IRS {
            fixed_legs,
            floating_legs,
            currency,
            unit_notional,
            issue_date,
            maturity,
            fixed_rate,
            rate_index,
            fixed_daycounter,
            floating_daycounter,
            fixed_busi_convenction,
            floating_busi_convenction,
            fixed_frequency,
            floating_frequency,
            calendar,
            name,
            code,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    pub fn get_maturity(&self) -> &OffsetDateTime {
        &self.maturity
    }

    pub fn get_rate_index(&self) -> Option<&RateIndex> {
        Some(&self.rate_index)
    }
}