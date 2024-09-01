use crate::definitions::Real;
use crate::enums::{CreditRating, IssuerType, RankType};
use crate::instrument::InstrumentTrait;
use crate::instruments::schedule::{build_schedule, Schedule};
use crate::parameters::zero_curve::ZeroCurve;
use crate::parameters::{past_price::DailyClosePrice, rate_index::RateIndex};
use crate::time::{
    calendar_trait::CalendarTrait,
    conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
    jointcalendar::JointCalendar,
};
use crate::InstInfo;
use static_id::StaticId;
//
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BondInfo {
    pub issuer_type: IssuerType,
    pub credit_rating: CreditRating,
    pub issuer_id: StaticId,
    pub rank: RankType,
}
/// None effective_date means issue date
/// None pricing_date means evaluation date
/// None settlement_date means maturity date
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bond {
    pub inst_info: InstInfo,
    pub bond_info: BondInfo,
    //
    pub is_coupon_strip: bool,
    //
    pub schedule: Schedule,
    pub floating_coupon_spread: Option<Real>,
    pub rate_index: Option<RateIndex>,
    pub floating_compound_tenor: Option<String>,
    pub fixed_coupon_rate: Option<Real>,
    //
    pub effective_date: OffsetDateTime,
    pub pricing_date: Option<OffsetDateTime>,
    pub settlement_date: OffsetDateTime,
    //
    pub calendar: JointCalendar,
    //
    pub daycounter: DayCountConvention,
    pub busi_convention: BusinessDayConvention,
    pub payment_frequency: PaymentFrequency,
    pub payment_gap_days: i64,
    pub fixing_gap_days: i64,
}

impl Default for Bond {
    fn default() -> Bond {
        Bond {
            inst_info: InstInfo::default(),
            bond_info: BondInfo {
                issuer_type: IssuerType::Undefined,
                credit_rating: CreditRating::Undefined,
                issuer_id: StaticId::default(),
                rank: RankType::Undefined,
            },
            //
            is_coupon_strip: false,
            //
            schedule: Schedule::default(),
            floating_coupon_spread: None,
            rate_index: None,
            floating_compound_tenor: None,
            fixed_coupon_rate: None,
            //
            effective_date: OffsetDateTime::now_utc(),
            pricing_date: None,
            settlement_date: OffsetDateTime::now_utc(),
            //
            calendar: JointCalendar::default(),
            //
            daycounter: DayCountConvention::Actual365Fixed,
            busi_convention: BusinessDayConvention::Following,
            payment_frequency: PaymentFrequency::SemiAnnually,
            payment_gap_days: 0,
            fixing_gap_days: 0,
        }
    }
}

impl Bond {
    //#[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        bond_info: BondInfo,
        //  
        is_coupon_strip: bool,
        //
        schedule: Schedule,
        floating_coupon_spread: Option<Real>,
        rate_index: Option<RateIndex>,
        floating_compound_tenor: Option<String>,
        fixed_coupon_rate: Option<Real>,
        //
        effective_date: Option<OffsetDateTime>,
        pricing_date: Option<OffsetDateTime>,
        settlement_date: Option<OffsetDateTime>,
        //
        calendar: JointCalendar,
        //
        daycounter: DayCountConvention,
        busi_convention: BusinessDayConvention,
        payment_frequency: PaymentFrequency,
        payment_gap_days: i64,
        fixing_gap_days: i64,
    ) -> Result<Bond> {
        // fixed_rate_coupon and rate_index can not be both None
        if fixed_coupon_rate.is_none() && rate_index.is_none() {
            let err = || anyhow!(
                "{}:{} id = {:?},\n\
                Both fixed_coupon_rate and rate_index can not be None",
                file!(),
                line!(),
                &inst_info.id
            );
            return Err(err());
        }
        // fixed_rate_coupon and rate_index can not be both Some
        if fixed_coupon_rate.is_some() && rate_index.is_some() {
            return Err(anyhow!(
                "{}:{} id = {:?},\n\
                Both fixed_coupon_rate and rate_index can not be Some",
                file!(),
                line!(),
                &inst_info.id
            ));
        }

        let effective_date = match effective_date {
            Some(date) => date,
            None => {
                if let Some(issue_date) = inst_info.get_issue_date() {
                    *issue_date
                } else {
                    let err = || anyhow!(
                        "{}:{} id = {:?},\n\
                        Failed to get issue date",
                        file!(),
                        line!(),
                        &inst_info.id
                    );

                    return Err(err());
                }
            },
        };
                    
        let settlement_date = match settlement_date {
            Some(date) => date,
            None => {
                if let Some(maturity) = inst_info.get_maturity() {
                    *maturity
                } else {
                    let err = || anyhow!(
                        "{}:{} id = {:?},\n\
                        Failed to get maturity date",
                        file!(),
                        line!(),
                        &inst_info.id
                    );

                    return Err(err());
                }
            },
        };
        
        Ok(Bond {
            inst_info,
            bond_info,
            //
            is_coupon_strip,
            //
            schedule,
            floating_coupon_spread,
            rate_index,
            floating_compound_tenor,
            fixed_coupon_rate,
            //
            effective_date,
            pricing_date,
            settlement_date,
            //
            calendar,
            //
            daycounter,
            busi_convention,
            payment_frequency,
            payment_gap_days,
            fixing_gap_days,
        })
    }

    //#[allow(clippy::too_many_arguments)]
    pub fn new_from_conventions(
        inst_info: InstInfo,
        bond_info: BondInfo,
        //
        is_coupon_strip: bool,
        //
        effective_date: Option<OffsetDateTime>,
        pricing_date: Option<OffsetDateTime>,
        settlement_date: Option<OffsetDateTime>,
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
    ) -> Result<Bond> {
        let effective_date = match effective_date {
            Some(date) => date,
            None => {
                if let Some(issue_date) = inst_info.get_issue_date() {
                    *issue_date
                } else {
                    let err = || anyhow!(
                        "{}:{} id = {:?},\n\
                        Failed to get issue date",
                        file!(),
                        line!(),
                        &inst_info.id
                    );

                    return Err(err());
                }
            },
        };

        let maturity = inst_info.get_maturity().unwrap();
        let schedule = build_schedule(
            forward_generation,
            &effective_date,
            maturity,
            &calendar,
            &busi_convention,
            &payment_frequency,
            fixing_gap_days,
            payment_gap_days,
        )
        .with_context(|| {
            anyhow!(
                "Failed to build schedule in FloatingRateNote: {:?}",
                inst_info.id
            )
        })?;

        let settlement_date = match settlement_date {
            Some(date) => date,
            None => *maturity,
        };

        Ok(Bond {
            inst_info,
            bond_info,
            //
            is_coupon_strip,
            //
            schedule,
            //
            fixed_coupon_rate,
            floating_coupon_spread,
            rate_index,
            floating_compound_tenor,
            //
            effective_date,
            pricing_date,
            settlement_date,
            //
            calendar,
            //
            daycounter,
            busi_convention,
            payment_frequency,
            fixing_gap_days,
            payment_gap_days,
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

    fn get_inst_info(&self) ->  &InstInfo {
        &self.inst_info
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

    fn get_credit_rating(&self) -> Result<CreditRating> {
        Ok(self.bond_info.credit_rating)
    }

    fn get_issuer_type(&self) -> Result<IssuerType> {
        Ok(self.bond_info.issuer_type)
    }

    fn get_issuer_id(&self) -> Result<StaticId> {
        Ok(self.bond_info.issuer_id)
    }

    fn is_coupon_strip(&self) -> Result<bool> {
        Ok(self.is_coupon_strip)
    }

    fn get_cashflows(
        &self,
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
                    res.entry(*payment_date)
                        .and_modify(|e| *e += amount)
                        .or_insert(amount);
                    //res.insert(payment_date.clone(), amount);
                }
                None => {
                    match self.rate_index.as_ref() {
                        Some(rate_index) => {
                            // begin of the case of frn
                            let amount = rate_index.get_coupon_amount(
                                base_schedule,
                                self.floating_coupon_spread,
                                forward_curve.clone().unwrap(),
                                past_data
                                    .clone()
                                    .unwrap_or(Rc::new(DailyClosePrice::default())),
                                pricing_date,
                                self.floating_compound_tenor.as_ref(),
                                &self.calendar,
                                &self.daycounter,
                                self.fixing_gap_days,
                            )?;
                            res.entry(*payment_date)
                                .and_modify(|e| *e += amount)
                                .or_insert(amount);
                            //*res.entry(payment_date.clone()).or_insert(amount) += amount;
                            //res.insert(payment_date.clone(), amount);
                        } // end of the case of frn
                        //
                        None => {
                            // begin of the case of fixed rate bond
                            let frac = self.calendar.year_fraction(
                                base_schedule.get_calc_start_date(),
                                base_schedule.get_calc_end_date(),
                                &self.daycounter,
                            )?;
                            let rate = self.fixed_coupon_rate.unwrap();
                            let amount = frac * rate;
                            res.entry(*payment_date)
                                .and_modify(|e| *e += amount)
                                .or_insert(amount);
                        } // end of the case of fixed rate bond
                    } // end of branch of bond type
                } // where the given amount is None
            } // end of branch of optional given amount
        }

        let maturity = self.inst_info.get_maturity().ok_or_else(|| {
            anyhow!(
                "{}:{} id = {:?},\n\
                Failed to get maturity date",
                file!(),
                line!(),
                &self.inst_info.id
            )
        })?;

        if !self.is_coupon_strip()? && maturity.date() >= pricing_date.date() {
            res.entry(*maturity)
                .and_modify(|e| *e += 1.0)
                .or_insert(1.0);
        }

        Ok(res)
    }
}
