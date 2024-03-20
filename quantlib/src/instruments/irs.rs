use crate::assets::currency::Currency;
use crate::definitions::{Integer, Real};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::parameters::rate_index::RateIndex;
use crate::instruments::schedule::{self, Schedule};
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::time::jointcalendar::JointCalendar;
use crate::instrument::InstrumentTriat;
use anyhow::{Result, Context, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRS {
    fixed_legs: Schedule,
    floating_legs: Schedule,
    fixed_rate: Real,
    rate_index: RateIndex,
    currency: Currency,
    unit_notional: Real,
    //
    issue_date: OffsetDateTime,
    effective_date: OffsetDateTime,
    maturity: OffsetDateTime,
    //
    fixed_daycounter: DayCountConvention,
    floating_daycounter: DayCountConvention,
    //
    fixed_busi_convention: BusinessDayConvention,
    floating_busi_convention: BusinessDayConvention,
    //
    fixed_frequency: PaymentFrequency,
    floating_frequency: PaymentFrequency,
    //
    fixing_days: Integer,
    payment_days: Integer,
    //
    name: String,
    code: String,
}

impl IRS {
    pub fn new(
        fixed_legs: Schedule,
        floating_legs: Schedule,
        fixed_rate: Real,
        rate_index: RateIndex,
        currency: Currency,
        unit_notional: Real,
        //
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        maturity: OffsetDateTime,
        //
        fixed_daycounter: DayCountConvention,
        floating_daycounter: DayCountConvention,
        //
        fixed_busi_convention: BusinessDayConvention,
        floating_busi_convention: BusinessDayConvention,
        //
        fixed_frequency: PaymentFrequency,
        floating_frequency: PaymentFrequency,
        //
        fixing_days: Integer,
        payment_days: Integer,
        //
        name: String,
        code: String,
    ) -> IRS {
        IRS {
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            currency,
            unit_notional,
            issue_date,
            effective_date,
            maturity,
            fixed_daycounter,
            floating_daycounter,
            fixed_busi_convention,
            floating_busi_convention,
            fixed_frequency,
            floating_frequency,
            fixing_days,
            payment_days,
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
        effective_date: OffsetDateTime,
        maturity: OffsetDateTime,
        //
        fixed_first_coupon_date: Option<OffsetDateTime>,
        fixed_rate: Real,
        rate_index: RateIndex,
        fixed_daycounter: DayCountConvention,
        fixed_busi_convention: BusinessDayConvention,
        fixed_frequency: PaymentFrequency,
        fixing_days: Integer,
        payment_days: Integer,
        calendar: JointCalendar,
        name: String,
        code: String,
    ) -> Result<IRS> {
        let floating_daycounter = rate_index.get_daycounter().clone();
        let floating_busi_convention = rate_index.get_business_day_convention().clone();
        let floating_frequency = rate_index.get_frequency().clone();

        let fixed_legs = schedule::build_schedule(
            &effective_date,
            None,
            &maturity,
            &calendar,
            &fixed_busi_convention,
            &fixed_frequency,
            fixing_days as i64,
            payment_days as i64,
        ).with_context(
            || anyhow!("Failed to build fixed legs in IRS: {}({})", &name, &code)
        )?;

        let floating_legs = schedule::build_schedule(
            &effective_date,
            fixed_first_coupon_date.as_ref(),
            &maturity,      
            &calendar,
            &floating_busi_convention,
            &floating_frequency,
            fixing_days as i64,
            payment_days as i64,
        ).with_context(
            || anyhow!("Failed to build floating legs in IRS: {}({})", &name, &code)
        )?;

        Ok(IRS {
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            currency,
            unit_notional,
            issue_date,
            effective_date,
            maturity,
            fixed_daycounter,
            floating_daycounter,
            fixed_busi_convention,
            floating_busi_convention,
            fixed_frequency,
            floating_frequency,
            fixing_days,
            payment_days,
            name,
            code,
        })
    }
}

impl InstrumentTriat for IRS {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_rate_index(&self) -> Result<&RateIndex> {
        Ok(&self.rate_index)
    }

    fn get_type_name(&self) -> &'static str {
        "IRS"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assets::currency::Currency,
        time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
        time::calendars::southkorea::{SouthKorea, SouthKoreaType},
        time::calendar::Calendar,
        time::jointcalendar::JointCalendar,
        parameters::rate_index::RateIndex,
        enums::RateIndexCode,
    };
    use time::{Duration, macros::datetime};
    use anyhow::Result;

    #[test]
    fn test_new_from_convention() -> Result<()> {
        let currency = Currency::KRW;
        let unit_notional = 100.0;
        let issue_date = datetime!(2021-01-01 00:00:00 +09:00);
        let maturity = datetime!(2021-12-31 00:00:00 +09:00);
        let fixed_rate = 0.01;
        let sk = JointCalendar::new(
            vec![Calendar::SouthKorea(
                SouthKorea::new(SouthKoreaType::Settlement)
                )]
            )?;

        let rate_index = RateIndex::new(
            PaymentFrequency::Quarterly,
            BusinessDayConvention::ModifiedFollowing,
            DayCountConvention::Actual365Fixed,
            String::from("91D"),
            None,
            None,
            sk,
            Currency::KRW,
            RateIndexCode::CD,
            "CD91".to_string(),
        )?;

        let fixing_days = 2;
        let payment_days = 0;

        let sk = JointCalendar::new(
            vec![Calendar::SouthKorea(
                SouthKorea::new(SouthKoreaType::Settlement)
                )]
            )?;
        
        let irs = IRS::new_from_conventions(
            currency,
            unit_notional,
            issue_date.clone(),
            issue_date.clone(),
            maturity,
            None,
            fixed_rate,
            rate_index.clone(),
            DayCountConvention::Actual365Fixed,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            fixing_days,
            payment_days,
            sk,
            "IRS".to_string(),
            "IRS".to_string(),
        )?;

        println!("{:?}", irs);
        assert_eq!(currency, *irs.get_currency());
        assert_eq!(unit_notional, irs.get_unit_notional());
        assert_eq!(maturity, irs.get_maturity().unwrap().clone());

        Ok(())
    }

}