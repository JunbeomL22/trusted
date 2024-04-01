use crate::currency::Currency;
use crate::definitions::{Integer, Real};
use crate::instruments::bond::Bond;
use crate::time::conventions::PaymentFrequency;
use crate::instrument::InstrumentTrait;
//
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KtbfVirtualBond {
    year: Integer,
    coupon_rate: Real,
    frequency: PaymentFrequency,
    unit_notional: Real,
}

impl KtbfVirtualBond {
    pub fn new(
        year: Integer, 
        coupon_rate: Real, 
        frequency: PaymentFrequency,
        unit_notional: Real,
    ) -> KtbfVirtualBond {
        KtbfVirtualBond {
            year,
            coupon_rate,
            frequency,
            unit_notional,
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
        res * self.unit_notional
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
    underlying_bonds: Vec<Bond>,
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
        underlying_bonds: Vec<Bond>,
        name: String,
        code: String,
    ) -> Result<KTBF> {
        // all underlying_bonds should have the same pricing date as the maturity
        for bond in underlying_bonds.iter() {
            let pricing_date = bond.get_pricing_date()?;
            if pricing_date.is_none() {
                return Err(anyhow!(
                    "({}:{}) The underlying bond {} ({}) in ktbf {} ({}) does not have a pricing date",
                    file!(), line!(),
                    bond.get_name(),
                    bond.get_code(),
                    name,
                    code,
                ));
            }
            match pricing_date.unwrap() == &maturity {
                true => (),
                false => return Err(anyhow!(
                    "({}:{}) The pricing date of the underlying bond {} ({}) in ktbf {} ({}) is not the same as the ktbf maturity",
                    file!(), line!(),
                    bond.get_name(),
                    bond.get_code(),
                    name,
                    code,
                )),
            }
        }    
        
        Ok(KTBF {
            currency,
            unit_notional,
            issue_date,
            maturity,
            settlement_date,
            virtual_bond,
            underlying_bonds,
            name,
            code,
        })
    }
    pub fn get_underlying_bonds(&self) -> &Vec<Bond> {
        &self.underlying_bonds
    }

}

impl InstrumentTrait for KTBF {
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

    fn get_virtual_bond_npv(&self, bond_yield: Real) -> Result<Real> {
        Ok(self.virtual_bond.npv(bond_yield))
    }

    fn get_underlying_bonds(&self) -> Result<&Vec<Bond>> {
        Ok(&self.underlying_bonds)
    }

}