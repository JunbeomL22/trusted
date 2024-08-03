use chrono::prelude::*;
use chrono::{DateTime, Local};
use core_affinity;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::thread;
use time;
use trading_engine::timer::{
    convert_unix_nano_to_date_and_time, get_thread_local_unix_nano, get_unix_nano,
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
    let _ = get_thread_local_unix_nano();

    group.bench_function("minstant", |b| {
        b.iter(|| {
            let instant = minstant::Instant::now();
            instant.as_unix_nanos(&anchor)
        });
    });

    group.bench_function("systemtime", |b| {
        b.iter(|| {
            let now = std::time::SystemTime::now();
            now.duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        });
    });

    group.bench_function("quanta", |b| {
        b.iter(|| get_unix_nano());
    });

    group.bench_function("quanta thread local", |b| {
        b.iter(|| get_thread_local_unix_nano());
    });

    group.finish();
}

fn bench_datetime_conversion_from_unix_nano(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    let instant = minstant::Instant::now();
    let unix_nano = instant.as_unix_nanos(&anchor);
    let local_offset = time::UtcOffset::from_hms(9, 0, 0).expect("failed to create UtcOffset");

    let mut group = c.benchmark_group("from nanos to datetime");
    group.bench_function("chrono::DateTime Local", |b| {
        b.iter(|| DateTime::<Local>::from(Local.timestamp_nanos(unix_nano as i64)));
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

fn bench_datetime_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime creation now");
    let offset = time::UtcOffset::from_hms(9, 0, 0).expect("failed to create UtcOffset");

    group.bench_function("chrono::DateTime utc", |b| {
        b.iter(chrono::Utc::now);
    });

    group.bench_function("chrono::DateTime Local", |b| {
        b.iter(|| chrono::Local::now());
    });

    group.bench_function("time::OffsetDatetime utc", |b| {
        b.iter(time::OffsetDateTime::now_utc);
    });
    group.bench_function("time::OffsetDatetime Local", |b| {
        b.iter(|| time::OffsetDateTime::now_utc().to_offset(offset));
    });

    group.finish();
}

fn bench_multi_thread_unix_nano(c: &mut Criterion) {
    let thread_number = 3;
    let mut group = c.benchmark_group(format!("{} threads quanta unix nano", thread_number));
    let _ = get_unix_nano();
    let _ = get_thread_local_unix_nano();
    let core_ids = core_affinity::get_core_ids().unwrap();

    group.sample_size(10);
    // generate 1000 get_unix_nano() for 6 threads
    group.bench_function("with thread local", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..thread_number)
                .map(|i| {
                    let core_id = core_ids[i].clone();
                    core_affinity::set_for_current(core_id);
                    thread::spawn(|| {
                        for _ in 0..1_000_000 {
                            get_thread_local_unix_nano();
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.bench_function("without thread local", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..thread_number)
                .map(|i| {
                    let core_id = core_ids[i].clone();
                    core_affinity::set_for_current(core_id);
                    thread::spawn(|| {
                        for _ in 0..1_000_000 {
                            get_unix_nano();
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_datetime_conversion_from_unix_nano,
    bench_datetime_creation,
    bench_nows,
    bench_unix_nanos,
    bench_multi_thread_unix_nano,
);
criterion_main!(benches);
