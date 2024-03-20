use crate::assets::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTriat;
use crate::instruments::schedule::{build_schedule, Schedule};
use crate::enums::{IssuerType, CreditRating, RankType};
use crate::parameters::zero_curve::ZeroCurve;
use crate::time::{
    conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
    jointcalendar::JointCalendar,
};
use crate::data::history_data::CloseData;
use crate::parameters::rate_index::RateIndex;
//
use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::{
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingRateNote {
    issuer_type: IssuerType,
    credit_rating: CreditRating,
    issuer_name: String,
    rank: RankType,
    currency: Currency,
    //
    unit_notional: Real,
    is_coupon_strip: bool,
    //
    schedule: Schedule,
    spread: Real,
    rate_index: RateIndex,
    //
    issue_date: OffsetDateTime,
    effective_date: OffsetDateTime,
    pricing_date: Option<OffsetDateTime>,
    first_coupon_date: Option<OffsetDateTime>,
    maturity: OffsetDateTime,
    //
    calendar: JointCalendar,
    //
    daycounter: DayCountConvention,
    busi_convention: BusinessDayConvention,
    frequency: PaymentFrequency, 
    coupon_payment_days: i64,
    //
    name: String,
    code: String,
}

impl FloatingRateNote {
    pub fn new(
        issuer_type: IssuerType,
        credit_rating: CreditRating,
        issuer_name: String,
        rank: RankType,
        currency: Currency,
        //
        unit_notional: Real,
        is_coupon_strip: bool,
        //
        schedule: Schedule,
        spread: Real,
        rate_index: RateIndex,
        //
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        pricing_date: Option<OffsetDateTime>,
        first_coupon_date: Option<OffsetDateTime>,
        maturity: OffsetDateTime,
        //
        calendar: JointCalendar,
        //
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        frequency: PaymentFrequency, 
        coupon_payment_days: i64,
        //
        name: String,
        code: String,
    ) -> Self {
        FloatingRateNote {
            issuer_type,
            credit_rating,
            issuer_name,
            rank,
            currency,
            //
            unit_notional,
            is_coupon_strip,
            //
            schedule,
            spread,
            rate_index,
            //
            issue_date,
            effective_date,
            pricing_date,
            first_coupon_date,
            maturity,
            //
            calendar,
            //
            daycounter,
            busi_convention,
            frequency, 
            coupon_payment_days,
            //
            name,
            code,
        }
    }

    pub fn new_from_conventions(
        issuer_type: IssuerType,
        credit_rating: CreditRating,
        issuer_name: String,
        rank: RankType,
        currency: Currency,
        //
        unit_notional: Real,
        is_coupon_strip: bool,
        //
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        first_coupon_date: Option<OffsetDateTime>,
        maturity: OffsetDateTime,
        //
        spread: Real,
        rate_index: RateIndex,
        coupon_payment_days: i64,
        calendar: JointCalendar,
        payment_days: i64,
        //
        name: String,
        code: String,
    ) -> Result<FloatingRateNote> {
        let daycounter = rate_index.get_daycounter().clone();
        let calc_day_convention = rate_index.get_calc_day_convention().clone();
        let frequency = rate_index.get_frequency().clone();
        let fixing_days = rate_index.get_fixing_days();

        let schedule = build_schedule(
            &effective_date,
            first_coupon_date.as_ref(),
            &maturity,
            &calendar,
            &calc_day_convention,
            &frequency,
            fixing_days,
            payment_days,
        ).with_context(
            || anyhow!("Failed to build schedule in FloatingRateNote: {}({})", &name, &code)
        )?;

        Ok(FloatingRateNote {
            issuer_type,
            credit_rating,
            issuer_name,
            rank,
            currency,
            //
            unit_notional,
            is_coupon_strip,
            //
            schedule,
            spread,
            rate_index,
            //
            issue_date,
            effective_date,
            pricing_date: None,
            first_coupon_date,
            maturity,
            //
            calendar,
            //
            daycounter,
            busi_convention: calc_day_convention,
            frequency, 
            coupon_payment_days,
            //
            name,
            code,
        })
    }

    pub fn get_schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn get_rate_index(&self) -> &RateIndex {
        &self.rate_index
    }

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

    fn get_coupon_cashflow(&self,
        pricing_date: Option<&OffsetDateTime>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_data: Option<&CloseData>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        // similar to FixedCouponBond::get_coupon_cashflow
        // but use rate_index to calculate the coupon amount
        // and use schedule to get the coupon payment date
        // moreover, it should consider the spread and the cashflow is given only when 
        // the payment date is after the pricing date
        let mut res = HashMap::new();
        let mut coupon_amount: Real;
        let eval_dt = pricing_date.date();
    }
}