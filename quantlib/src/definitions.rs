#![allow(unused_imports)]
use simba::simd::{WideF32x4, WideF32x8};
use simba::simd::WideF64x4;

// Note! must be a variable that can derive Copy and PartialOrd trait. 
// Of course, it would be either f32 or f64.
pub type Real = f32; 
pub type Realx4 = WideF32x4;


pub type Natural = u32;
pub type Integer = i32;

