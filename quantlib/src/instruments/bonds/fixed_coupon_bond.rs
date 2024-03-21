use crate::assets::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTriat;
use crate::instruments::schedule::{self, Schedule};
use crate::enums::{IssuerType, CreditRating, RankType};
use crate::time::{
    conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
    jointcalendar::JointCalendar,
    calendar_trait::CalendarTrait,
};
use crate::parameters::zero_curve::ZeroCurve;
use crate::data::history_data::CloseData;
//
use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::{
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};

/// pricing date is not mandatory, if not given, it is assumed to be the same date as evaluation date in Engine
/// Bond is settled on the next or day after the trade date. Therefore, the price must be calculated on the settlement date.
/// However, after the trade date, if we calculate the price considering the settlement gap, we obtain a weird theta value.
/// if pricing date is not None, it discounts the cashflows to the evaluation date and compound the value to the settlement date.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        pricing_date: Option<OffsetDateTime>,
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
            pricing_date,
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
        pricing_date: Option<OffsetDateTime>,
        //
        calendar: JointCalendar,
        //
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        frequency: PaymentFrequency,
        issuer_name: String,
        coupon_payment_days: i64,
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
            coupon_payment_days,
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
            pricing_date,
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
        })
    }

    pub fn get_frequency(&self) -> &PaymentFrequency {
        &self.frequency
    }

    pub fn get_effective_date(&self) -> &OffsetDateTime {
        &self.effective_date
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

    pub fn set_pricing_date(&mut self, pricing_date: Option<OffsetDateTime>) {
        self.pricing_date = pricing_date;
    }

}

impl InstrumentTriat for FixedCouponBond {
    fn get_pricing_date(&self) -> Result<Option<&OffsetDateTime>, anyhow::Error> {
        Ok(self.pricing_date.as_ref())
    }

    fn is_coupon_strip(&self) -> Result<bool> {
        Ok(self.is_coupon_strip)
    }

    fn get_type_name(&self) -> &'static str {
        "FixedCouponBond"
    }

    fn get_credit_rating(&self) -> Result<&CreditRating> {
        Ok(&self.credit_rating)
    }

    fn get_issuer_type(&self) -> Result<&IssuerType> { 
        Ok(&self.issuer_type)
    }
    
    fn get_rank_type(&self) -> Result<&RankType> { 
        Ok(&self.rank)
    }
    
    fn get_issuer_name(&self) -> Result<&String> { 
        Ok(&self.issuer_name)
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

    fn get_issue_date(&self) -> Result<&OffsetDateTime> {
        Ok(&self.issue_date)
    }

    /// generate coupon-cashflow after evaluation date for bonds
    /// if include_evaluation_date is true, it will include the evaluation date
    fn get_coupon_cashflow(
        &self, 
        _pricing_date: Option<&OffsetDateTime>,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        _past_data: Option<&Rc<CloseData>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        let mut coupon_amount: Real;

        for base_schedule in self.schedule.iter() {
            let start_date = base_schedule.get_calc_start_date();
            let end_date = base_schedule.get_calc_end_date();
            let payment_date = base_schedule.get_payment_date();
            let amount = base_schedule.get_amount();

            match amount {
                Some(amount) => {
                    res.insert(payment_date.clone(), amount);
                },
                None => {
                    coupon_amount = self.coupon_rate * self.calendar.year_fraction(
                        start_date, 
                        end_date,
                        &self.daycounter
                    )?;

                    res.insert(payment_date.clone(), coupon_amount);
                }
            }
        }
        Ok(res)
    }

    fn get_calendar(&self) -> Result<&JointCalendar> {
        Ok(&self.calendar)
    }

    fn get_frequency(&self) -> Result<PaymentFrequency> {
        Ok(self.frequency)
    }
}