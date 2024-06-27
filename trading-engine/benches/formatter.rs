use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_format::{lazy_format, write};
use joinery::JoinableIterator;
use trading_engine::utils::timer::get_unix_nano;
use ustr::Ustr;
use ryu;
use std::fmt::Write;
use itoa;
use trading_engine::types::precision::{
    Precision,
    Prec2,
};

fn bench_base(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("compare");

    for n in &[1, 10, 100, 1000] {
        bgroup.bench_function(format!("format/{n}"), |b| {
            b.iter(|| (0..*n)
                .map(|value| format!("\t'{}'", value))
                .join_with("\n")
                .to_string()
            );
        });
        bgroup.bench_function(format!("lazy_format/{n}"), |b| {
            b.iter(|| (0..*n)
                .map(|value| lazy_format!("\t'{}'", value))
                .join_with("\n")
                .to_string()
            );
        });
    }

    bgroup.finish();
}

fn bench_concat_numbers(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("concat_numbers");

    fn format_numbers(dest: &mut String, n: usize) {
        let values = (0..n).map(|value| format!("'{}'...", value));
        for value in values {
            write!(dest, "{}", value).unwrap();
        }
    }

    fn lazy_format_numbers(dest: &mut String, n: usize) {
        let values = (0..n).map(|value| lazy_format!("'{}'...", value));
        for value in values {
            write!(dest, "{}", value).unwrap();
        }
    }

    for n in &[1, 10, 100, 1000] {
        bgroup.bench_function(format!("format/{n}"), |b| {
            b.iter(|| {
                let mut dest = String::new();
                format_numbers(&mut dest, *n);
                dest
            });
        });

        bgroup.bench_function(format!("lazy_format/{n}"), |b| {
            b.iter(|| {
                let mut dest = String::new();
                lazy_format_numbers(&mut dest, *n);
                dest
            });
        });
    }

    bgroup.finish();
}

fn bench_datetime_formatter(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("datetime_formatter");

    let chrono_now = chrono::Utc::now();
    let time_now = time::OffsetDateTime::now_utc();

    bgroup.bench_function("chrono::DateTime -> to_string", |b| {
        b.iter(|| chrono_now.to_string());
    });

    bgroup.bench_function("chrono::DateTime -> format", |b| {
        b.iter(|| format!("{}", chrono_now));
    });

    bgroup.bench_function("time::OffsetDatetime -> to_string", |b| {
        b.iter(|| time_now.to_string());
    });

    bgroup.bench_function("time::OffsetDatetime -> format", |b| {
        b.iter(|| format!("{}", time_now));
    });

    let unix_nano = minstant::Instant::now().as_unix_nanos(&minstant::Anchor::new());
    bgroup.bench_function("unix_nano -> to_string", |b| {
        b.iter(|| {
            unix_nano.to_string();
        });
    });

    bgroup.bench_function("unix_nano -> format", |b| {
        b.iter(|| {
            format!("{}", unix_nano);
        });
    });

    bgroup.bench_function("unix_nano -> lazy_format", |b| {
        b.iter(|| {
            lazy_format!("{}", unix_nano);
        });
    });

    bgroup.finish();
}

fn bench_make_string_then_byte_string(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    let mut bgroup = c.benchmark_group("make_string_then_byte_string");

    bgroup.bench_function("format -> to_string -> into_bytes", |b| {
        b.iter(|| {
            let unix_nano = minstant::Instant::now().as_unix_nanos(&anchor);
            let s = format!(
                "({} unixnano) {}",
                unix_nano,
                "hello world");
            s.as_bytes().to_vec()
        });
    });

    bgroup.bench_function("unix_nano -> into_bytes", |b| {
        b.iter(|| {
            let unix_nano = minstant::Instant::now().as_unix_nanos(&anchor);
            let s1 = "(".as_bytes();
            let ts_bytes = unix_nano.to_le_bytes();
            let s2 = " unixnano) ".as_bytes();
            let msg_bytes = "hello world".as_bytes();

            //return the concated byte
            let mut bytes = Vec::with_capacity(s1.len() + ts_bytes.len() + s2.len() + msg_bytes.len());
            bytes.extend_from_slice(s1);
            bytes.extend_from_slice(&ts_bytes);
            bytes.extend_from_slice(s2);
            bytes.extend_from_slice(msg_bytes);
            bytes
        });
    });
    bgroup.finish();
}

fn bench_lazy_formatter(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("lazy_formatter");

    let unix_nano = get_unix_nano();
    bgroup.bench_function("lazy_format", |b| {
        b.iter(|| {
            
            lazy_format!("({} unixnano) {}", unix_nano, "hello world")
        });
    });

    bgroup.bench_function("lazy_format -> to_string", |b| {
        b.iter(|| {
            lazy_format!("({} unixnano) {}", unix_nano, "hello world").to_string()
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

fn bench_string_to_i64(
    c: &mut Criterion,
) {
    let mut bgroup = c.benchmark_group("string_to_i64");

    let f64_str = "123456.12";

    let prec = Prec2 {};

    bgroup.bench_function("str_to_i64", |b| {
        b.iter(|| {
            let val_f64 = f64_str.parse::<f64>().unwrap();
            let val_i64 = Prec2::price_f64_to_i64(val_f64).unwrap();
        });
    });

    let val_f64 = f64_str.parse::<f64>().unwrap();
    let val_i64 = Prec2::price_f64_to_i64(val_f64).unwrap();
    let mut buffer = ryu::Buffer::new();
    bgroup.bench_function("i64_to_str", |b| {
        b.iter(|| {
            let val_f64 = Prec2::price_i64_to_f64(val_i64);
            let printed = buffer.format(val_f64);
        });
    });

    bgroup.finish();
}

criterion_group!(
    benches, 
    //bench_base,
    //bench_concat_numbers,
    //bench_datetime_formatter,
    //bench_make_string_then_byte_string,
    //bench_lazy_formatter,
    //bench_integer_to_string,
    //bench_float_to_string,
    //bench_str_to_number,
    bench_string_to_i64,
);

criterion_main!(benches);
