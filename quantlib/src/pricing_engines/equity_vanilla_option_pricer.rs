use crate::evaluation_date::EvaluationDate;
use crate::assets::{
    stock::Stock,
    currency::Currency,
};
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::pricing_engines::pricer::PricerTrait;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::InstrumentTrait;
//
use time::OffsetDateTime;
use anyhow::{anyhow, Context, Result};
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

pub struct StockVanillaOptionPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    stock: Rc<RefCell<Stock>>,
    collateral_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    borrowing_curve: Rc<RefCell<ZeroCurve>>, // or repo
}