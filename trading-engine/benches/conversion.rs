use chrono::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itoa;
use ryu;
use time::format_description::well_known::Rfc3339;
use trading_engine::data::krx::derivative_trade::{
    IFMSRPD0037,
    IFMSRPD0038,
};
use trading_engine::data::krx::derivative_quote::{
    IFMSRPD0034,
    //IFMSRPD0035,
};
use trading_engine::data::{
    trade_quote::TradeQuoteSnapshot,
    quote::QuoteSnapshot,
};
use trading_engine::utils::numeric_converter::NumReprCfg;
use trading_engine::utils::numeric_converter::{
    parse_under16_with_floating_point, parse_under32_with_floating_point,
    parse_under8_with_floating_point, IntegerConverter,
};
use trading_engine::utils::timer::{convert_unix_nano_to_datetime_format, get_unix_nano};
use trading_engine::{
    TimeStamp,
    types::timestamp::DateUnixNanoGenerator,
    data::krx::krx_converter::get_krx_timestamp_converter,
};

fn bench_intger_and_float(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("integer_and_float");

    let u32_var: u32 = 123456789;
    let u64_var: u64 = 123456789;

    let f32_var: f32 = 123456789.0;
    let f64_var: f64 = 123456789.0;

    bgroup.bench_function("u32 as f32", |b| {
        b.iter(|| black_box(u32_var as f32));
    });

    bgroup.bench_function("u32 as f64", |b| {
        b.iter(|| black_box(u32_var as f64));
    });

    bgroup.bench_function("u64 as f32", |b| {
        b.iter(|| black_box(u64_var as f32));
    });

    bgroup.bench_function("u64 as f64", |b| {
        b.iter(|| black_box(u64_var as f64));
    });

    bgroup.bench_function("f32 as u32", |b| {
        b.iter(|| black_box(f32_var as u32));
    });

    bgroup.bench_function("f32 as u64", |b| {
        b.iter(|| black_box(f32_var as u64));
    });

    bgroup.bench_function("f64 as u32", |b| {
        b.iter(|| black_box(f64_var as u32));
    });

    bgroup.bench_function("f64 as u64", |b| {
        b.iter(|| black_box(f64_var as u64));
    });

    bgroup.finish();
}

fn bench_datetime_formatter(c: &mut Criterion) {
    let unix_nano = get_unix_nano();
    let chrono_dt = DateTime::<Local>::from(Local.timestamp_nanos(unix_nano as i64));
    let offset = time::UtcOffset::from_hms(9, 0, 0).expect("failed to create UtcOffset");

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
            time_now
                .format(&Rfc3339)
                .unwrap_or_else(|_| String::from("failed to format"))
        });
    });

    bgroup.bench_function("time::OffsetDatetime Local -> to_string", |b| {
        b.iter(|| time_dt.to_string());
    });

    bgroup.bench_function("time::OffsetDatetime Local -> Rfc3339", |b| {
        b.iter(|| {
            time_dt
                .format(&Rfc3339)
                .unwrap_or_else(|_| String::from("failed to format"))
        });
    });

    bgroup.bench_function("convert_unix_nano_to_datetime_format", |b| {
        b.iter(|| convert_unix_nano_to_datetime_format(unix_nano, 9));
    });

    let unix_nano = minstant::Instant::now().as_unix_nanos(&minstant::Anchor::new());
    bgroup.bench_function("unix_nano -> to_string", |b| {
        b.iter(|| {
            unix_nano.to_string();
        });
    });

    bgroup.finish();
}

fn bench_integer_to_string(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("u64_to_ustr");

    let var: u32 = 12345678;

    bgroup.bench_function("u32_to_std_string", |b| {
        b.iter(|| black_box(var.to_string()));
    });

    let var: u64 = 1234567890;

    bgroup.bench_function("u64_to_std_string", |b| {
        b.iter(|| black_box(var.to_string()));
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

fn bench_float_to_string(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("f64_to_ustr");

    let var: f64 = 1234567890.0;

    bgroup.bench_function("f64_to_std_string", |b| {
        b.iter(|| black_box(var.to_string()));
    });

    let mut buffer = ryu::Buffer::new();
    bgroup.bench_function("f64_to_str_by_ryu", |b| {
        b.iter(|| {
            let printed = buffer.format(var);
        });
    });

    bgroup.finish();
}

fn bench_str_to_number(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("string_to_number");

    let u64_str = "1234567890";

    let f64_str = "123456.12";
    let f64_bytes = f64_str.as_bytes();

    bgroup.bench_function("str_to_i32", |b| {
        b.iter(|| black_box(u64_str.parse::<i32>().unwrap()));
    });

    bgroup.bench_function("str_to_i64", |b| {
        b.iter(|| black_box(u64_str.parse::<i64>().unwrap()));
    });

    bgroup.bench_function("str_to_i128", |b| {
        b.iter(|| black_box(u64_str.parse::<i128>().unwrap()));
    });

    bgroup.bench_function("str_to_u32", |b| {
        b.iter(|| black_box(u64_str.parse::<u32>().unwrap()));
    });

    bgroup.bench_function("str_to_u64", |b| {
        b.iter(|| black_box(u64_str.parse::<u64>().unwrap()));
    });

    bgroup.bench_function("str_to_u128", |b| {
        b.iter(|| black_box(u64_str.parse::<u128>().unwrap()));
    });

    bgroup.bench_function("str_to_f32", |b| {
        b.iter(|| black_box(u64_str.parse::<f32>().unwrap()));
    });

    bgroup.bench_function("str_to_f64", |b| {
        b.iter(|| black_box(u64_str.parse::<f64>().unwrap()));
    });

    bgroup.finish();
}

fn bench_integer_converter(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("integer_converter");

    let cfg = NumReprCfg {
        digit_length: 5,
        decimal_point_length: 0,
        is_signed: false,
        unused_length: 0,
        total_length: 5,
        float_normalizer: None,
        drop_decimal_point: false,
    };

    let converter = IntegerConverter::new(cfg).expect("failed to create IntegerConverter");

    let s = b"12345";

    bgroup.bench_function("converter.to_i64 (12345)", |b| {
        b.iter(|| converter.to_i64(black_box(s)));
    });

    bgroup.bench_function("converter.to_u64 (12345)", |b| {
        b.iter(|| converter.to_u64(black_box(s)));
    });

    let num_cfg = NumReprCfg {
        digit_length: 5,
        decimal_point_length: 3,
        is_signed: true,
        unused_length: 0,
        total_length: 9,
        float_normalizer: None,
        drop_decimal_point: false,
    };

    let converter = IntegerConverter::new(num_cfg).expect("failed to create IntegerConverter");

    let s = b"-12345.67";
    bgroup.bench_function("converter.to_i64 (-12345.67)", |b| {
        b.iter(|| converter.to_i64(black_box(s)));
    });

    bgroup.bench_function("converter.to_u64 (-12345.67)", |b| {
        b.iter(|| converter.to_u64(black_box(s)));
    });

    let cfg = NumReprCfg {
        digit_length: 9,
        decimal_point_length: 0,
        is_signed: false,
        total_length: 9,
        unused_length: 0,
        float_normalizer: None,
        drop_decimal_point: false,
    };

    let converter = IntegerConverter::new(cfg).expect("failed to create IntegerConverter");

    let s = b"123456789";

    bgroup.bench_function("converter.to_i64 (123456789)", |b| {
        b.iter(|| converter.to_i64(black_box(s)));
    });

    bgroup.bench_function("converter.to_u64 (123456789)", |b| {
        b.iter(|| converter.to_u64(black_box(s)));
    });

    let cfg = NumReprCfg {
        digit_length: 18,
        decimal_point_length: 4,
        is_signed: false,
        unused_length: 0,
        total_length: 22,
        float_normalizer: None,
        drop_decimal_point: true,
    };

    let converter = IntegerConverter::new(cfg).expect("failed to create IntegerConverter");

    let s = b"123456789012345678.000";

    bgroup.bench_function("converter.to_i64 (123456789012345678.000)", |b| {
        b.iter(|| converter.to_i64(black_box(s)));
    });

    bgroup.bench_function("converter.to_u64 (123456789012345678.000)", |b| {
        b.iter(|| converter.to_u64(black_box(s)));
    });

    bgroup.finish();
}

fn bench_parsing(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("parsing");
    let s = b"12345.67";
    bgroup.bench_function("parse_under8_with_floating_point (12345.67)", |b| {
        b.iter(|| parse_under8_with_floating_point(black_box(s), 8, 3));
    });

    let s = b"012345678.901";
    bgroup.bench_function("parse_under16_with_floating_point (012345678.901)", |b| {
        b.iter(|| parse_under16_with_floating_point(black_box(s), 13, 4));
    });

    let s = b"12345";
    bgroup.bench_function("parse_under8_with_floating_point (12345)", |b| {
        b.iter(|| parse_under8_with_floating_point(black_box(s), 5, 0));
    });

    let s = b"123456789";
    bgroup.bench_function("parse_under16_with_floating_point (123456789)", |b| {
        b.iter(|| parse_under16_with_floating_point(black_box(s), 9, 0));
    });

    let s = b"123456.7";
    bgroup.bench_function("parse_under8_with_floating_point (123456.7)", |b| {
        b.iter(|| parse_under8_with_floating_point(black_box(s), 8, 2));
    });

    let s = b"123456.78";
    bgroup.bench_function("parse_under16_with_floating_point (123456.78)", |b| {
        b.iter(|| parse_under16_with_floating_point(black_box(s), 9, 3));
    });

    let s = b"123456.789";
    bgroup.bench_function("parse_under16_with_floating_point (123456.789)", |b| {
        b.iter(|| parse_under16_with_floating_point(black_box(s), 10, 4));
    });

    let s = b"012345678901234.5678901234567890";
    bgroup.bench_function(
        "parse_under32_with_floating_point (1234.56789012345678901)",
        |b| {
            b.iter(|| parse_under32_with_floating_point(black_box(s), 22, 18));
        },
    );

    bgroup.finish();
}
fn bench_parse_stock_derivative_trade_quote(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_stock_derivative_trade_quote");
    let mut test_data_vec = b"G704F        G140KR41CNV10006003661104939829612000066500000000007000000000000000000000070300000070900000066100000066400000000041770000000028415067000.000200006990000006310000006660000006640000000006900000006800010000060000667000000663000000000810000001630001200011000066800000066200000000066000000049000120000700006690000006610000000004400000012900013000200000670000000660000000000300000000970000900016000067100000065900000000030000000036000060000600006720000006580000000009100000002300007000080000673000000657000000000290000000160001000005000067400000065600000000026000000043000060001100006750000006550000000004500000004000011000080000023600000021120046600205".to_vec();
    test_data_vec.push(255);
    let test_data = test_data_vec.as_slice();
    let ifmsrpd0038 = IFMSRPD0038::default().with_quote_level_cut(4).expect("");

    let mut trade_quote_data = TradeQuoteSnapshot::with_quote_level(4);

    let mut date_generator = DateUnixNanoGenerator::from(time::macros::date!(2023-12-28));
    group.bench_function("parse stock derivative (G704F, cut 4)", |b| {
        b.iter(|| {
            ifmsrpd0038
                .to_trade_quote_snapshot(black_box(test_data), &mut date_generator)
                .expect("failed to parse")
        });
    });

    group.bench_function("parse stock derivative with buffer (G704F, cut 4)", |b| {
        b.iter(|| {
            ifmsrpd0038
                .to_trade_quote_snapshot_buffer(black_box(test_data), &mut trade_quote_data, &mut date_generator)
                .expect("failed to parse")
        });
    });

    let ifmrspd0038 = IFMSRPD0038::default().with_quote_level_cut(6).expect("");

    group.bench_function("parse stock derivative (G704F, cut 6)", |b| {
        b.iter(|| {
            ifmrspd0038
                .to_trade_quote_snapshot(black_box(test_data), &mut date_generator)
                .expect("failed to parse")
        });
    });

    let mut trade_quote_data = TradeQuoteSnapshot::with_quote_level(6);
    group.bench_function("parse stock derivative with buffer (G704F, cut 6)", |b| {
        b.iter(|| {
            ifmrspd0038
                .to_trade_quote_snapshot_buffer(black_box(test_data), &mut trade_quote_data, &mut date_generator)
                .expect("failed to parse")
        });
    });
}

fn bench_parse_derivative_trade_quote(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("parse derivative trade quote");

    let mut date_generator = DateUnixNanoGenerator::from(time::macros::date!(2023-12-28));
    bgroup.warm_up_time(std::time::Duration::from_secs(4));
    let mut test_data_vec = b"G703F        G140KR4301V13502001656104939081108000002.12000000005000000.00000000.00000002.83000002.93000002.06000002.11000000021511000000013250790000.0002000006.86000000.01000002.12000002.110000000100000000100000300006000002.13000002.100000000330000000410001100011000002.14000002.090000000290000000430000800010000002.15000002.080000000380000000370000900013000002.16000002.0700000001800000006200007000110000017960000059190049400380".to_vec();
    test_data_vec.push(255);
    let test_data = test_data_vec.as_slice();
    let interface = IFMSRPD0037::default().with_quote_level_cut(4).expect("");
    bgroup.bench_function("parse non-stock (g703f, cut 4)", |b| {
        b.iter(|| interface.to_trade_quote_snapshot(black_box(test_data), &mut date_generator));
    });

    let mut trade_quote_data_buffer = TradeQuoteSnapshot::with_quote_level(4);

    bgroup.bench_function("parse non-stock with buffer (g703f, cut 4)", |b| {
        b.iter(|| {
            interface.to_trade_quote_snapshot_buffer(black_box(test_data), &mut trade_quote_data_buffer, &mut date_generator)
        });
    });

    let interface = IFMSRPD0037::default().with_quote_level_cut(5).expect("");
    bgroup.bench_function("parse non-stock (g703f, cut 5)", |b| {
        b.iter(|| interface.to_trade_quote_snapshot(black_box(test_data), &mut date_generator));
    });

    let mut trade_quote_data_buffer = TradeQuoteSnapshot::with_quote_level(5);
    bgroup.bench_function("parse non-stock with buffer (g703f, cut 5)", |b| {
        b.iter(|| {
            interface.to_trade_quote_snapshot_buffer(black_box(test_data), &mut trade_quote_data_buffer, &mut date_generator)
        });
    });

    bgroup.finish();
}

fn bench_derivative_quote(c: &mut Criterion) {
    let mut group = c.benchmark_group("non-stock derivative_quote");
    let mut test_data_vec = b"B602F        G140KR4106V30004000020104939405656001379.70001379.500000000030000000030000300003001379.80001379.400000000040000000040000400004001379.90001379.300000000070000000050000600005001380.00001379.200000000050000000070000500007001380.10001379.1000000000500000000500005000050000009020000025920031700642000000.00000000000".to_vec();
    test_data_vec.push(255);
    let test_data = test_data_vec.as_slice();
    let mut date_generator = DateUnixNanoGenerator::from(time::macros::date!(2023-12-28));

    let ifmsrpd0034 = IFMSRPD0034::default().with_quote_level_cut(4).expect("");
    let mut quote_data = QuoteSnapshot::with_quote_level(4);
    group.bench_function("parse non-stock derivative quote (b602f) (4 cut)", |b| {
        b.iter(|| {
            ifmsrpd0034
                .to_quote_snapshot(black_box(test_data), &mut date_generator)
        });
    });

    let mut quote_data = QuoteSnapshot::with_quote_level(4);    
    group.bench_function("parse non-stock derivative quote with buffer (b602f) (4 cut)", |b| {
        b.iter(|| {
            ifmsrpd0034
                .to_quote_snapshot_buffer(black_box(test_data), &mut quote_data, &mut date_generator)
                .expect("failed to parse")
        });
    });

    let ifmsrpd0034 = IFMSRPD0034::default().with_quote_level_cut(5).expect("");
    group.bench_function("parse non-stock derivative quote (b602f) (5 cut)", |b| {
        b.iter(|| {
            ifmsrpd0034
                .to_quote_snapshot(black_box(test_data), &mut date_generator)
        });
    });

    let mut quote_data = QuoteSnapshot::with_quote_level(5);
    group.bench_function("parse non-stock derivative quote with buffer (b602f) (5 cut)", |b| {
        b.iter(|| {
            ifmsrpd0034
                .to_quote_snapshot_buffer(black_box(test_data), &mut quote_data, &mut date_generator)
                .expect("failed to parse")
        });
    });

    group.finish();

}

fn bench_timestamp_conversion(c: &mut Criterion) {
    let timestamp_converter = get_krx_timestamp_converter();
    let dt = time::macros::date!(2023-12-28);
    
    let mut date_generator = DateUnixNanoGenerator::from(dt);
    
    let timestamp_bytes = b"163020300111";

    let mut bgroup = c.benchmark_group("timestamp_conversion");

    let system_time = TimeStamp { stamp: crate::get_unix_nano() };
    bgroup.bench_function("convert_timestamp", |b| {
        b.iter(|| 
            {
            let _timestamp = timestamp_converter.parse_hhmmssuuuuuu(
                timestamp_bytes, 
                Some(system_time),
                &mut date_generator
            ).unwrap();
        });
    });

    bgroup.finish();
}

criterion_group!(
    benches,
    bench_timestamp_conversion,
    bench_derivative_quote,
    bench_parse_stock_derivative_trade_quote,
    bench_parse_derivative_trade_quote,
    //bench_custom_numeric_converter,
    //bench_integer_converter,
    //bench_parsing,
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
