use crate::assets::currency::Currency;
use crate::definitions::{Real, Integer};
use crate::instrument::InstrumentTriat;
use crate::time::jointcalendar::JointCalendar;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::instruments::schedule::{self, Schedule};
use crate::enums::{IssuerType, CreditRating, RankType};
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use anyhow::{Result, Context, anyhow};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedCouponBond {
    issuer_type: IssuerType,
    credit_rating: CreditRating,
    issuer_name: String,
    rank: RankType,
    currency: Currency,
    //
    coupon_rate: Real,
    unit_notional: Real,
    is_coupon_strip: bool,
    //
    schedule: Schedule,
    //
    issue_date: OffsetDateTime,
    effective_date: OffsetDateTime,
    first_coupon_date: Option<OffsetDateTime>,
    maturity: OffsetDateTime,
    //
    daycounter: DayCountConvention,
    busi_convention: BusinessDayConvention,
    frequency: PaymentFrequency, 
    payment_days: i64,
    //
    name: String,
    code: String,
}

impl FixedCouponBond {
    pub fn new(
        issuer_type: IssuerType,
        credit_rating: CreditRating,
        issuer_name: String,
        rank: RankType,
        currency: Currency,
        //
        coupon_rate: Real,
        unit_notional: Real,
        is_coupon_strip: bool,
        //
        schedule: Schedule,
        //
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        first_coupon_date: Option<OffsetDateTime>,
        maturity: OffsetDateTime,
        //
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        frequency: PaymentFrequency, 
        payment_days: i64,
        //
        name: String,
        code: String,
    ) -> FixedCouponBond {
        FixedCouponBond {
            issuer_type,
            credit_rating,
            issuer_name,
            rank,
            currency,
            //
            coupon_rate,
            unit_notional,
            is_coupon_strip,
            //
            schedule,
            //
            issue_date,
            effective_date,
            first_coupon_date,
            maturity,
            //
            daycounter,
            busi_convention,
            frequency,
            payment_days,
            //
            name,
            code,
        }
    }

    /// construct FixedCouponBond using PaymentFrequency, BusinessDayConvention, DayCountConvention
    /// without schdule given directly
    pub fn new_from_conventions(
        currency: Currency,
        issuer_type: IssuerType,
        credit_rating: CreditRating,
        rank: RankType,
        is_coupon_strip: bool,
        coupon_rate: Real,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        maturity: OffsetDateTime,
        first_coupon_date: Option<OffsetDateTime>,
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        frequency: PaymentFrequency,
        issuer_name: String,
        payment_days: i64,
        calendar: JointCalendar,
        name: String,
        code: String,
    ) -> Result<FixedCouponBond> {
        let schedule = schedule::build_schedule(
            &effective_date,
            first_coupon_date.as_ref(),
            &maturity,
            &calendar,
            &busi_convention,
            &frequency,
            0,
            payment_days,
        ).with_context(
            || anyhow!("Failed to build schedule in FixedCouponBond: {}({}))", &name, &code)
        )?;

        Ok(FixedCouponBond {
            issuer_type,
            credit_rating,
            issuer_name,
            rank,
            currency,
            //
            coupon_rate,
            unit_notional,
            is_coupon_strip,
            //
            schedule,
            //
            issue_date,
            effective_date,
            first_coupon_date,
            maturity,
            //
            daycounter,
            busi_convention,
            frequency,
            payment_days,
            //
            name,
            code,
        })
    }

    pub fn get_schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn get_coupon_rate(&self) -> Real {
        self.coupon_rate
    }

    pub fn get_daycounter(&self) -> &DayCountConvention {
        &self.daycounter
    }

    pub fn is_coupon_strip(&self) -> bool {
        self.is_coupon_strip
    }

}

impl InstrumentTriat for FixedCouponBond {
    fn as_fixed_coupon_bond(&self) -> Result<&FixedCouponBond> {
        Ok(self)
    }

    fn get_type_name(&self) -> &'static str {
        "FixedCouponBond"
    }

    fn get_credit_rating(&self) -> Option<&CreditRating> {
        Some(&self.credit_rating)
    }

    fn get_issuer_type(&self) -> Option<&IssuerType> { 
        Some(&self.issuer_type)
    }
    
    fn get_rank_type(&self) -> Option<&RankType> { 
        Some(&self.rank)
    }
    
    fn get_issuer_name(&self) -> Option<&String> { 
        Some(&self.issuer_name)
    }

    fn get_currency(&self) -> &Currency { 
        &self.currency
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }
}