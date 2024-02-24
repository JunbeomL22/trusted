#![allow(unused_imports)]
use time;

// Note! must be a variable that can derive Copy and PartialOrd trait. 
// Of course, it would be highly likely either f32 or f64.
pub type Real = f32; 
pub type Time = Real;

pub type Natural = u32;
pub type Integer = i32;

/// Default time. This actually changes regarding the markets and the instruments.
pub const CLOSING_TIME: time::Time = time::macros::time!(16:00:00); 
pub const EX_DIVIDEND_TIME: time::Time = time::macros::time!(00:00:00);
pub const OPENING_TIME: time::Time = time::macros::time!(09:00:00);
pub const MARKING_DATE: time::Date = time::macros::date!(1970-01-01);

// hours, minutes, seconds
pub const SOUTH_KOREA_OFFSET: (i8, i8, i8) = (9, 0, 0);