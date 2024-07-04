use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_format::{lazy_format, write};
use joinery::JoinableIterator;
use trading_engine::utils::timer::{
    get_unix_nano,
    convert_unix_nano_to_datetime_format,
};
use ustr::Ustr;
use ryu;
use std::fmt::Write;
use itoa;
use chrono::prelude::*;
use time::format_description::well_known::Rfc3339;
use trading_engine::utils::numeric_converter::NumReprCfg;
use trading_engine::utils::numeric_converter::{
    IntegerConverter, 
    parse_8_chars, 
    parse_16_chars_by_split,
    parse_16_chars_with_u128,
    parse_32_chars_by_split,
};

fn bench_intger_and_float(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("integer_and_float");

    let u32_var: u32 = 123456789;
    let u64_var: u64 = 123456789;

    let f32_var: f32 = 123456789.0;
    let f64_var: f64 = 123456789.0;

    bgroup.bench_function("u32 as f32", |b| {
        b.iter(|| {
            black_box(u32_var as f32)
        });
    });

    bgroup.bench_function("u32 as f64", |b| {
        b.iter(|| {
            black_box(u32_var as f64)
        });
    });

    bgroup.bench_function("u64 as f32", |b| {
        b.iter(|| {
            black_box(u64_var as f32)
        });
    });

    bgroup.bench_function("u64 as f64", |b| {
        b.iter(|| {
            black_box(u64_var as f64)
        });
    });

    bgroup.bench_function("f32 as u32", |b| {
        b.iter(|| {
            black_box(f32_var as u32)
        });
    });

    bgroup.bench_function("f32 as u64", |b| {
        b.iter(|| {
            black_box(f32_var as u64)
        });
    });

    bgroup.bench_function("f64 as u32", |b| {
        b.iter(|| {
            black_box(f64_var as u32)
        });
    });

    bgroup.bench_function("f64 as u64", |b| {
        b.iter(|| {
            black_box(f64_var as u64)
        });
    });

    bgroup.finish();
}

fn bench_datetime_formatter(c: &mut Criterion) {
    let unix_nano = get_unix_nano();
    let chrono_dt = DateTime::<Local>::from(Local.timestamp_nanos(unix_nano as i64));
    let offset = time::UtcOffset::from_hms(9,0,0)
        .expect("failed to create UtcOffset");

    let time_dt = time::OffsetDateTime::from_unix_timestamp_nanos(unix_nano as i128)
        .expect("failed to convert to OffsetDateTime")
        .to_offset(offset);

    let mut bgroup = c.benchmark_group("datetime_formatter");

    let chrono_now = chrono::Utc::now();
    let time_now = time::OffsetDateTime::now_utc();

    bgroup.bench_function("chrono::DateTime utc -> to_string", |b| {
        b.iter(|| chrono_now.to_string());
    });

    bgroup.bench_function("chrono::DateTime Local -> to_string", |b| {
        b.iter(|| chrono_dt.to_string());
    }); 

    bgroup.bench_function("time::OffsetDatetime utc -> to_string", |b| {
        b.iter(|| time_now.to_string());
    });

    bgroup.bench_function("time::OffsetDatetime utc -> Rfc3339", |b| {
        b.iter(|| {
            time_now.format(&Rfc3339).unwrap_or_else(|_| String::from("failed to format"))
        });
    });

    bgroup.bench_function("time::OffsetDatetime Local -> to_string", |b| {
        b.iter(|| { time_dt.to_string() });
    });

    bgroup.bench_function("time::OffsetDatetime Local -> Rfc3339", |b| {
        b.iter(|| {
            time_dt.format(&Rfc3339).unwrap_or_else(|_| String::from("failed to format"))
        });
    });

    bgroup.bench_function("convert_unix_nano_to_datetime_format", |b| {
        b.iter(|| {
            convert_unix_nano_to_datetime_format(unix_nano, 9)
        });
    });

    let unix_nano = minstant::Instant::now().as_unix_nanos(&minstant::Anchor::new());
    bgroup.bench_function("unix_nano -> to_string", |b| {
        b.iter(|| {
            unix_nano.to_string();
        });
    });

    bgroup.finish();
}

fn bench_integer_to_string(
    c: &mut Criterion,
) {
    let mut bgroup = c.benchmark_group("u64_to_ustr");

    let var: u32 = 12345678;

    bgroup.bench_function("u32_to_std_string", |b| {
        b.iter(|| {
            black_box(var.to_string())
        });
    });

    let var: u64 = 1234567890;

    bgroup.bench_function("u64_to_std_string", |b| {
        b.iter(|| {
            black_box(var.to_string())
        });
    });

    let mut itoa_buffer = itoa::Buffer::new();
    bgroup.bench_function("itoa_u32_to_str", |b| {
        b.iter(|| {
            let printed = itoa_buffer.format(var);
        });
    });

    bgroup.bench_function("itoa_u64_to_str", |b| {
        b.iter(|| {
            let printed = itoa_buffer.format(var);
        });
    });

    bgroup.finish();
}

fn bench_float_to_string(
    c: &mut Criterion,
) {
    let mut bgroup = c.benchmark_group("f64_to_ustr");

    let var: f64 = 1234567890.0;
    
    bgroup.bench_function("f64_to_std_string", |b| {
        b.iter(|| {
            black_box(var.to_string())
        });
    });

    let mut buffer = ryu::Buffer::new();
    bgroup.bench_function("f64_to_str_by_ryu", |b| {
        b.iter(|| {
            let printed = buffer.format(var);
        });
    });

    bgroup.finish();
}

fn bench_str_to_number(
    c: &mut Criterion,
) {
    let mut bgroup = c.benchmark_group("string_to_number");

    let u64_str = "1234567890";

    let f64_str = "123456.12";
    let f64_bytes = f64_str.as_bytes();
    
    

    bgroup.bench_function("str_to_i32", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<i32>().unwrap())
        });
    });

    bgroup.bench_function("str_to_i64", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<i64>().unwrap())
        });
    });

    bgroup.bench_function("str_to_i128", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<i128>().unwrap())
        });
    });

    bgroup.bench_function("str_to_u32", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<u32>().unwrap())
        });
    });

    bgroup.bench_function("str_to_u64", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<u64>().unwrap())
        });
    });

    bgroup.bench_function("str_to_u128", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<u128>().unwrap())
        });
    });

    bgroup.bench_function("str_to_f32", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<f32>().unwrap())
        });
    });

    bgroup.bench_function("str_to_f64", |b| {
        b.iter(|| {
            black_box(u64_str.parse::<f64>().unwrap())
        });
    });

    bgroup.finish();
}

fn bench_custom_numeric_converter(
    c: &mut Criterion,
) {
    let mut bgroup = c.benchmark_group("custom_numeric_converter");

    let cfg = NumReprCfg {
        digit_length: 11,
        decimal_point_length: 0,
        is_signed: false,
        total_length: 11,
        float_normalizer: None,
    };

    let mut converter = IntegerConverter::new(cfg).unwrap();
    let val_str = "00000123456";
    bgroup.bench_function("str_to_u64", |b| {
        b.iter(|| {
            let _ = converter.to_u64(val_str);
        });
    });

    let cfg = NumReprCfg {
        digit_length: 8,
        decimal_point_length: 3,
        is_signed: true,
        total_length: 13,
        float_normalizer: None,
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();

    let val_str = "-00001234.563";
    bgroup.bench_function("str_to_i64", |b| {
        b.iter(|| {
            let _ = converter.to_i64(val_str);
        });
    });
    
    let val_str = "-111111100001234.563";
    let cfg = NumReprCfg {
        digit_length: 15,
        decimal_point_length: 3,
        is_signed: true,
        total_length: 20,
        float_normalizer: Some(3),
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();
    bgroup.bench_function("str_to_i64_long_integer", |b| {
        b.iter(|| {
            let _ = converter.to_i64(val_str);
        });
    });

    let val_i64 = -1234567890;
    bgroup.bench_function("normalized i64_to_f64", |b| {
        b.iter(|| {
            let _ = converter.normalized_f64_from_i64(val_i64);
        });
    });

    bgroup.finish();
}

fn bench_parsing(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("parsing");

    let s = "00001234";
    bgroup.bench_function("parse_8_chars", |b| {
        b.iter(|| {
            let _ = parse_8_chars(s);
        });
    });
    
    let s = "0000123456789012";
    bgroup.bench_function("parse_16_chars_with_u128", |b| {
        b.iter(|| {
            let _ = parse_16_chars_with_u128(s);
        });
    });

    bgroup.bench_function("parse_16_chars_by_split", |b| {
        b.iter(|| {
            let _ = parse_16_chars_by_split(s);
        });
    });

    let s = "00000000000000940000123400001234";
    bgroup.bench_function("parse_32_chars_by_split", |b| {
        b.iter(|| {
            let _ = parse_32_chars_by_split(s);
        });
    });

    bgroup.finish();
}

criterion_group!(
    benches, 
    bench_custom_numeric_converter,
    bench_parsing,
    /*
    bench_str_to_number,
    bench_intger_and_float,
    bench_datetime_formatter,
    bench_integer_to_string,
    bench_float_to_string,
    bench_str_to_number,
    bench_string_to_i64,
     */
);

criterion_main!(benches);
