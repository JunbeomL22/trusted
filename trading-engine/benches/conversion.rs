use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trading_engine::utils::timer::{
    get_unix_nano,
    convert_unix_nano_to_datetime_format,
};
use ryu;
use itoa;
use chrono::prelude::*;
use time::format_description::well_known::Rfc3339;
use trading_engine::utils::numeric_converter::NumReprCfg;
use trading_engine::utils::numeric_converter::{
    IntegerConverter, 
    parse_8_chars,
    parse_9_chars, 
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
        digit_length: 5,
        decimal_point_length: 2,
        drop_decimal_point: false,
        is_signed: true,
        total_length: 9,
        float_normalizer: None,
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();

    let val_str = b"-12345.67";

    bgroup.bench_function("small-number str_to_i64 (-12_345.67)", |b| {
        b.iter(|| {
            converter.to_i64(black_box(val_str))
        });
    });

    let cfg = NumReprCfg {
        digit_length: 7,
        decimal_point_length: 1,
        drop_decimal_point: false,
        is_signed: true,
        total_length: 10,
        float_normalizer: None,
    };

    let mut converter = IntegerConverter::new(cfg).unwrap();
    let val_str = b"01000012.3";

    bgroup.bench_function("small-number str_to_u64 (010_000_12.3)", |b| {
        b.iter(|| {
            converter.to_u64(black_box(val_str))
        });
    });

    bgroup.bench_function("small-number str_to_i64 negative (-10_000_12.3)", |b| {
        b.iter(|| {
            converter.to_i64(black_box(b"-1000012.3"))
        });
    });

    let cfg = NumReprCfg {
        digit_length: 14,
        decimal_point_length: 2,
        drop_decimal_point: false,
        is_signed: false,
        total_length: 17,
        float_normalizer: None,
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();
    let val_str = b"12345678901234.56";

    bgroup.bench_function("mid-size str_to_u64 (12_345_678_901_234.56)", |b| {
        b.iter(|| {
            converter.to_u64(black_box(val_str))
        });
    });

    bgroup.bench_function("mid-size str_to_i64 (12_345_678_901_234.56)", |b| {
        b.iter(|| {
            converter.to_i64(black_box(val_str))
        });
    });


    let cfg = NumReprCfg {
        digit_length: 8,
        decimal_point_length: 1,
        drop_decimal_point: false,
        is_signed: true,
        total_length: 11,
        float_normalizer: None,
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();

    let val_str = b"012345678.9";
   
    bgroup.bench_function("mid-size str_to_u64 (-1_234_567.89)", |b| {
        b.iter(|| {
            converter.to_u64(black_box(val_str))            
        });
    });
   
    bgroup.bench_function("mid-size str_to_i64 (-1_234_567.89)", |b| {
        b.iter(|| {
            converter.to_i64(black_box(val_str))            
        });
    });

    let cfg = NumReprCfg {
        digit_length: 8,
        decimal_point_length: 3,
        drop_decimal_point: false,
        is_signed: true,
        total_length: 13,
        float_normalizer: None,
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();
    let val_str = b"-12345678.912";

    bgroup.bench_function("mid-size str_to_i64 (-1_234_567.891)", |b| {
        b.iter(|| {
            converter.to_i64(black_box(val_str))
        });
    });

    
    let val_str = b"-111_111_100_001_234.563";
    let cfg = NumReprCfg {
        digit_length: 15,
        decimal_point_length: 3,
        drop_decimal_point: false,
        is_signed: true,
        total_length: 20,
        float_normalizer: Some(3),
    };
    let mut converter = IntegerConverter::new(cfg).unwrap();
    bgroup.bench_function("long-integr str_to_i64 (-111_111_100_001_234.563)", |b| {
        b.iter(|| {
            converter.to_i64(black_box(val_str))
        });
    });

    bgroup.finish();
}

fn bench_parsing(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("parsing");
    let s = "100123456.1";
    bgroup.bench_function("parse_by_std", |b| {
        b.iter(|| {
            black_box(s).parse::<f64>().unwrap()
        });
    });

    let s = b"00001234";
    bgroup.bench_function("parse_8_chars (00001234)", |b| {
        b.iter(|| {
            parse_8_chars(black_box(s))
        });
    });

    let s = b"000012345";
    bgroup.bench_function("parse_9_chars (000012345)", |b| {
        b.iter(|| {
            parse_9_chars(black_box(s))
        });
    });

    let s = b"0000123456789012";
    bgroup.bench_function("parse_16_chars_with_u128 (0000123456789012)", |b| {
        b.iter(|| {
            parse_16_chars_with_u128(black_box(s))
        });
    });


    bgroup.bench_function("parse_16_chars_by_split", |b| {
        b.iter(|| {
            parse_16_chars_by_split(black_box(s))
        });
    });

    let s = b"00000000000000940000123400001234";
    bgroup.bench_function("parse_32_chars_by_split", |b| {
        b.iter(|| {
            parse_32_chars_by_split(black_box(s))
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
