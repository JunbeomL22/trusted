// Include the anyhow crate
use anyhow::{Result, Context};
use crate::definitions::{Real, Time};
use thiserror::Error;
use std::{fs, vec};
use std::fmt::Debug;
use time::OffsetDateTime;

//
#[derive(Debug)]
pub enum VectorDisplay {
    BOOL(Vec<bool>),
    I8(Vec<i8>),
    U8(Vec<u8>),
    I16(Vec<i16>),
    U16(Vec<u16>),
    I32(Vec<i32>),
    U32(Vec<u32>),
    F32(Vec<f32>),
    I64(Vec<i64>),
    U64(Vec<u64>),
    F64(Vec<f64>),
    I128(Vec<i128>),
    U128(Vec<u128>),
    REAL(Vec<Real>),
    DATETIME(Vec<OffsetDateTime>),
}

#[derive(Error, Debug)]
pub enum MyError {
    // base error
    #[error("by {file}:{line}. contents: {contents}")]
    BaseError {
        file: String,
        line: u32,
        contents: String,
    },
    
    #[error(
        "by {file}:{line}. Failed to read file ({path})\n\
        others: {other_info}")]
    ReadError {
        file: String,
        line: u32,
        path: String,
        other_info: String,
    },

    #[error(
        "by {file}:{line}.\n\
        two vectors have different lengths:\n\
        left = {left:?}\n\
        right = {right:?}\n\
        others: {other_info}"
    )]
    MismatchedLengthError{
        file: String,
        line: u32,
        left: VectorDisplay,
        right: VectorDisplay,
        other_info: String,
    },

    #[error(
        "by {file}:{line}.\n\
        the vector is empty:\n\
        vector = {vector:?}\n\
        others: {other_info}"
    )]
    EmptyVectorError{
        file: String,
        line: u32,
        vector: VectorDisplay,
        other_info: String,
    },

    #[error(
        "by {file}:{line},\n\
        the time t1 ({t1}) is greater than t2 ({t2})\n\
        others: {other_info}"
    )]
    MisorderedTimeError {
        file: String, 
        line: u32,
        t1: Time,
        t2: Time,
        other_info: String,
    }

    // misordered offsetdatetime error
    #[error(
        "by {file}:{line},\n\
        the offsetdatetime d1 ({d1}) is greater than d2 ({d2})\n\
        others: {other_info}"
    )]
    MisorderedOffsetDateTimeError {
        file: String, 
        line: u32,
        d1: OffsetDateTime,
        d2: OffsetDateTime,
        other_info: String,
    }
       
}