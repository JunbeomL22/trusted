use crate::assets::currency::Currency;
use crate::definitions::{Integer, Real, COUPON_PAYMENT_TIME};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::instruments::bonds::bond::Bond;
use crate::instruments::bonds::fixed_coupon_bond::FixedCouponBond;
use crate::time::conventions::{DayCountConvention, PaymentFrequency, BusinessDayConvention};
use crate::instrument::InstrumentTriat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KtbfVirtualBond {
    year: Integer,
    coupon_rate: Real,
    frequency: PaymentFrequency,
}

impl KtbfVirtualBond {
    pub fn new(year: Integer, coupon_rate: Real, frequency: PaymentFrequency) -> KtbfVirtualBond {
        KtbfVirtualBond {
            year,
            coupon_rate,
            frequency,
        }
    }

    /// 파생상품시장 업무규정 시행세칙
    /// https://law.krx.co.kr/las/LawRevJo.jsp?lawid=000114&pubno=0000022080&pubdt=20240205
    pub fn npv(&self, bond_yield: Real) -> Real {
        let coupon_payment_number = self.year * self.frequency as Integer;
        let calc_freq = self.frequency.as_real();
        let effective_yield = bond_yield / calc_freq;
        let effective_coupon = self.coupon_rate / calc_freq;
        let mut res = 0.0;
        for i in 1..=coupon_payment_number {
            res += effective_coupon / (1.0 + effective_yield).powi(i as i32);
        }
        res += 1.0 / (1.0 + effective_yield).powi(coupon_payment_number as i32);
        res
    }
    
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KTBF {
    currency: Currency,
    unit_notional: Real,
    issue_date: OffsetDateTime,
    maturity: OffsetDateTime,
    settlement_date: OffsetDateTime,
    virtual_bond: KtbfVirtualBond,
    underlying_bonds: Vec<FixedCouponBond>,
    name: String,
    code: String,
}

impl KTBF {
    pub fn new(
        currency: Currency,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        maturity: OffsetDateTime,
        settlement_date: OffsetDateTime,
        virtual_bond: KtbfVirtualBond,
        underlying_bonds: Vec<FixedCouponBond>,
        name: String,
        code: String,
    ) -> KTBF {
        KTBF {
            currency,
            unit_notional,
            issue_date,
            maturity,
            settlement_date,
            virtual_bond,
            underlying_bonds,
            name,
            code,
        }
    }
    pub fn get_underlying_bonds(&self) -> &Vec<FixedCouponBond> {
        &self.underlying_bonds
    }
}

impl InstrumentTriat for KTBF {
    fn get_type_name(&self) -> &'static str {
        "KTBF"
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

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }
}