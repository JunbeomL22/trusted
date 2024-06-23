use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chrono::{DateTime, Local};
use chrono::prelude::*;
use time;
use time::format_description::well_known::Rfc3339;
use trading_engine::timer::{
    get_unix_nano,
    convert_unix_nano_to_datetime_format,
};

fn bench_nows(c: &mut Criterion) {
    // The first call will take some time for calibartion
    quanta::Instant::now();

    let mut group = c.benchmark_group("Instant::now()");
    group.bench_function("minstant", |b| {
        b.iter(minstant::Instant::now);
    });
    
    group.bench_function("systemtime", |b| {
        b.iter(std::time::Instant::now);
    });

    group.bench_function("time crate", |b| {
        b.iter(time::Instant::now);
    });

    group.bench_function("quanta now", |b| {
        b.iter(quanta::Instant::now);
    });

    let clock = quanta::Clock::new();
    group.bench_function("quanta clock raw", |b| {
        b.iter(|| clock.raw());
    });

    group.finish();
}

fn bench_unix_nanos(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    let mut group = c.benchmark_group("Instant::as_unix_nanos()");
    let _ = get_unix_nano();

    group.bench_function("minstant", |b| {
        b.iter(|| {
            let instant = minstant::Instant::now();
            instant.as_unix_nanos(&anchor)
        });
    });

    group.bench_function("systemtime", |b| {
        b.iter(|| {
            let now = std::time::SystemTime::now();
            now.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        });
    });

    group.bench_function("quanta", |b| {
        b.iter(|| {
            get_unix_nano()
        });
    });

    group.finish();
}

fn bench_datetime_conversion_from_unix_nano(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    let instant = minstant::Instant::now();
    let unix_nano = instant.as_unix_nanos(&anchor);
    let local_offset = time::UtcOffset::from_hms(9,0,0)
        .expect("failed to create UtcOffset");

    let mut group = c.benchmark_group("from nanos to datetime");
    group.bench_function("chrono::DateTime Local", |b| {
        b.iter(|| {
            DateTime::<Local>::from(Local.timestamp_nanos(unix_nano as i64))
        });
    });
    
    group.bench_function("time::OffsetDatetime Local", |b| {
        b.iter(|| {
            //time::OffsetDateTime::from_unix_timestamp_nanos(unix_nano as i128)
            time::OffsetDateTime::from_unix_timestamp_nanos(unix_nano as i128)
                .expect("failed to convert to OffsetDateTime")
                .to_offset(local_offset)
        });
    });

    group.finish();

}

fn bench_direct_datetime_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime creation now");
    group.bench_function("chrono::DateTime", |b| {
        b.iter(chrono::Utc::now);
    });
    group.bench_function("time::OffsetDatetime", |b| {
        b.iter(time::OffsetDateTime::now_utc);
    });
    
    group.finish();
}

fn bench_conversion_to_string(c: &mut Criterion) {
    let unix_nano = get_unix_nano();
    let chrono_dt = DateTime::<Local>::from(Local.timestamp_nanos(unix_nano as i64));
    let time_dt = time::OffsetDateTime::from_unix_timestamp_nanos(unix_nano as i128)
        .expect("failed to convert to OffsetDateTime");

    let mut group = c.benchmark_group("conversion to string");
    group.bench_function("chrono::DateTime", |b| {
        b.iter(|| {
            format!("{}", chrono_dt.format("%Y-%m-%dT%H:%M:%S%.3f%:z"))
        });
    });
    
    group.bench_function("time::OffsetDatetime Local", |b| {
        b.iter(|| {
            time_dt.format(&Rfc3339).unwrap_or_else(|_| String::from("failed to format"))
        });
    });

    group.bench_function("convert_unix_nano_to_datetime_format", |b| {
        b.iter(|| {
            convert_unix_nano_to_datetime_format(unix_nano, 9)
        });
    });

    group.finish();
}



criterion_group!(
    benches, 
    bench_datetime_conversion_from_unix_nano,
    bench_direct_datetime_creation,
    bench_conversion_to_string,
    bench_nows,
    bench_unix_nanos,
);
criterion_main!(benches);