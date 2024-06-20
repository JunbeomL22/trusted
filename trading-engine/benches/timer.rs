use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chrono;
use time;
//use fastdate;

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

    group.finish();
}

fn bench_anchor_new(c: &mut Criterion) {
    c.bench_function("minstant::Anchor::new()", |b| {
        b.iter(minstant::Anchor::new);
    });
}

fn bench_as_unix_nanos(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    let mut group = c.benchmark_group("Instant::as_unix_nanos()");
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

    group.finish();
}

fn bench_datetime_conversion(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    let instant = minstant::Instant::now();
    let unix_nano = instant.as_unix_nanos(&anchor);
    
    let mut group = c.benchmark_group("from nanos to datetime");
    group.bench_function("chrono::DateTime", |b| {
        b.iter(|| {
            chrono::DateTime::<chrono::Utc>::from_timestamp_nanos(unix_nano as i64)
        });
    });
    group.bench_function("time::OffsetDatetime", |b| {
        b.iter(|| {
            time::OffsetDateTime::from_unix_timestamp_nanos(unix_nano as i128)
        });
    });
}

fn bench_datetime_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime creation");
    group.bench_function("chrono::DateTime", |b| {
        b.iter(chrono::Utc::now);
    });
    group.bench_function("time::OffsetDatetime", |b| {
        b.iter(time::OffsetDateTime::now_utc);
    });
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_nows, 
    //bench_anchor_new, 
    bench_as_unix_nanos,
    bench_datetime_conversion,
    bench_datetime_creation,
);
criterion_main!(benches);