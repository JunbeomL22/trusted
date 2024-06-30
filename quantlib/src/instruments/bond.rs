use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::instruments::schedule::{build_schedule, Schedule};
use crate::enums::{IssuerType, CreditRating, RankType};
use crate::parameters::zero_curve::ZeroCurve;
use crate::time::{
    conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
    jointcalendar::JointCalendar,
    calendar_trait::CalendarTrait,
};
use crate::parameters::{
    rate_index::RateIndex,
    past_price::DailyClosePrice,
};
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
pub struct Bond {
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
    floating_coupon_spread: Option<Real>,
    rate_index: Option<RateIndex>,
    floating_compound_tenor: Option<String>,
    fixed_coupon_rate: Option<Real>,
    //
    issue_date: OffsetDateTime,
    effective_date: OffsetDateTime,
    pricing_date: Option<OffsetDateTime>,
    maturity: OffsetDateTime,
    //
    calendar: JointCalendar,
    //
    daycounter: DayCountConvention,
    busi_convention: BusinessDayConvention,
    payment_frequency: PaymentFrequency, 
    payment_gap_days: i64,
    fixing_gap_days: i64,
    //
    name: String,
    code: String,
}

impl Bond {
    #[allow(clippy::too_many_arguments)]
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
        //
        fixed_coupon_rate: Option<Real>,
        floating_coupon_spread: Option<Real>,
        rate_index: Option<RateIndex>,
        floating_compound_tenor: Option<String>,
        //
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        pricing_date: Option<OffsetDateTime>,
        maturity: OffsetDateTime,
        //
        calendar: JointCalendar,
        //
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        payment_frequency: PaymentFrequency, 
        payment_gap_days: i64,
        fixing_gap_days: i64,
        //
        name: String,
        code: String,
    ) -> Result<Bond> {
        // fixed_rate_coupon and rate_index can not be both None
        if fixed_coupon_rate.is_none() && rate_index.is_none() {
            return Err(anyhow!(
                "{}:{} name = {}, code = {}\n\
                Both fixed_coupon_rate and rate_index can not be None",
                file!(), line!(), &name, &code
            ));
        }
        // fixed_rate_coupon and rate_index can not be both Some
        if fixed_coupon_rate.is_some() && rate_index.is_some() {
            return Err(anyhow!(
                "{}:{} name = {}, code = {}\n\
                Both fixed_coupon_rate and rate_index can not be Some",
                file!(), line!(), &name, &code
            ));
        }
        Ok(Bond {
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
            //
            fixed_coupon_rate,
            floating_coupon_spread,
            rate_index,
            floating_compound_tenor,
            //
            issue_date,
            effective_date,
            pricing_date,
            maturity,
            //
            calendar,
            //
            daycounter,
            busi_convention,
            payment_frequency, 
            payment_gap_days,
            fixing_gap_days,
            //
            name,
            code,
        })
    }

    #[allow(clippy::too_many_arguments)]
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
        pricing_date: Option<OffsetDateTime>,
        maturity: OffsetDateTime,
        //
        fixed_coupon_rate: Option<Real>,
        floating_coupon_spread: Option<Real>,
        rate_index: Option<RateIndex>,
        floating_compound_tenor: Option<String>,
        //
        calendar: JointCalendar,
        //
        forward_generation: bool,
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        payment_frequency: PaymentFrequency, 
        fixing_gap_days: i64,
        payment_gap_days: i64,
        //
        name: String,
        code: String,
    ) -> Result<Bond> {
        let schedule = build_schedule(
            forward_generation,
            &effective_date,
            &maturity,
            &calendar,
            &busi_convention,
            &payment_frequency,
            fixing_gap_days,
            payment_gap_days,
        ).with_context(
            || anyhow!("Failed to build schedule in FloatingRateNote: {} ({})", &name, &code)
        )?;

        Ok(Bond {
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
            //
            fixed_coupon_rate,
            floating_coupon_spread,
            rate_index,
            floating_compound_tenor,
            //
            issue_date,
            effective_date,
            pricing_date,
            maturity,
            //
            calendar,
            //
            daycounter,
            busi_convention,
            payment_frequency, 
            fixing_gap_days,
            payment_gap_days,
            
            //
            name,
            code,
        })
    }

    pub fn set_pricing_date(&mut self, pricing_date: OffsetDateTime) {
        self.pricing_date = Some(pricing_date);
    }
}

impl InstrumentTrait for Bond {
    fn get_pricing_date(&self) -> Result<Option<&OffsetDateTime>> {
        Ok(self.pricing_date.as_ref())
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    fn get_schedule(&self) -> Result<&Schedule> {
        Ok(&self.schedule)
    }

    fn get_calendar(&self) -> Result<&JointCalendar> {
        Ok(&self.calendar)
    }

    fn get_coupon_frequency(&self) -> Result<PaymentFrequency> {
        Ok(self.payment_frequency)
    }

    fn get_type_name(&self) -> &'static str {
        "Bond"
    }

    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        Ok(self.rate_index.as_ref())
    }

    fn get_credit_rating(&self) -> Result<&CreditRating> {
        Ok(&self.credit_rating)
    }
    
    fn get_issuer_type(&self) -> Result<&IssuerType> {
        Ok(&self.issuer_type)
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

    fn get_issue_date(&self) -> Result<&OffsetDateTime> {
        Ok(&self.issue_date)
    }

    fn is_coupon_strip(&self) -> Result<bool> {
        Ok(self.is_coupon_strip)
    }

    fn get_cashflows(&self,
        pricing_date: &OffsetDateTime,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_data: Option<Rc<DailyClosePrice>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        for base_schedule in self.schedule.iter() {
            let payment_date = base_schedule.get_payment_date();
            if payment_date.date() < pricing_date.date() {
                continue;
            }

            let given_amount = base_schedule.get_amount();
            match given_amount {
                Some(amount) => {
                    res.entry(*payment_date).and_modify(|e| *e += amount).or_insert(amount);
                    //res.insert(payment_date.clone(), amount);
                },
                None => {
                    match self.rate_index.as_ref() {
                        Some(rate_index) => {// begin of the case of frn
                            let amount = rate_index.get_coupon_amount(
                                base_schedule,
                                self.floating_coupon_spread,
                                forward_curve.clone().unwrap(),
                                past_data.clone().unwrap_or(Rc::new(DailyClosePrice::default())),
                                pricing_date,
                                self.floating_compound_tenor.as_ref(),
                                &self.calendar,
                                &self.daycounter,
                                self.fixing_gap_days,
                            )?;
                            res.entry(*payment_date).and_modify(|e| *e += amount).or_insert(amount);
                            //*res.entry(payment_date.clone()).or_insert(amount) += amount;
                            //res.insert(payment_date.clone(), amount);
                        }, // end of the case of frn
                        //
                        None => {// begin of the case of fixed rate bond
                            let frac = self.calendar.year_fraction(
                                base_schedule.get_calc_start_date(),
                                base_schedule.get_calc_end_date(),
                                &self.daycounter,
                            )?;
                            let rate = self.fixed_coupon_rate.unwrap();
                            let amount = frac * rate;
                            res.entry(*payment_date).and_modify(|e| *e += amount).or_insert(amount);

                        }, // end of the case of fixed rate bond
                    } // end of branch of bond type
                }, // where the given amount is None
            } // end of branch of optional given amount
        }

        if !self.is_coupon_strip()? && self.maturity.date() >= pricing_date.date() {
            res.entry(self.maturity).and_modify(|e| *e += 1.0).or_insert(1.0);
        }

        Ok(res)
    }
}