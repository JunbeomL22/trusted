#![allow(unused_imports)]
use time;
use crate::assets::currency::{self, Currency};
/// Note! must be a variable that can derive Copy and PartialOrd trait. 
/// Of course, it would be highly likely either f32 or f64.
pub type Real = f32; 
pub type Time = Real;

pub type Natural = u32;
pub type Integer = i32;

/// (Currency::USD, Currency::KRW) => USD/KRW = 1,331.4 (as of 2024-02-27)
pub type FX = (Currency, Currency); 

/// Default time. This actually changes regarding the markets and the instruments.
pub const CLOSING_TIME: time::Time = time::macros::time!(16:00:00); 
pub const EX_DIVIDEND_TIME: time::Time = time::macros::time!(00:00:01);
pub const COUPON_PAYMENT_TIME: time::Time = time::macros::time!(16:30:00);
pub const OPENING_TIME: time::Time = time::macros::time!(09:00:00);
pub const MARKING_DATE: time::Date = time::macros::date!(1970-01-01); // to make an offsetdatetime to be an integer

/// hours, minutes, seconds
pub const SEOUL_OFFSET: (i8, i8, i8) = (9, 0, 0);

/// pnl units
pub const DELTA_PNL_UNIT: Real = 0.01;
pub const GAMMA_PNL_UNIT: Real = 0.01;
pub const VEGA_PNL_UNIT: Real = 0.01;
pub const RHO_PNL_UNIT: Real = 0.0001;
pub const DIV_PNL_UNIT: Real = 0.0001;
pub const THETA_PNL_UNIT: Real = 1.0;

