use flexstr::{LocalStr, SharedStr, IntoSharedStr};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_string_creation(c: &mut Criterion) {
    let str_data = "KR0123456789";
    let mut group = c.benchmark_group("String Creation 1000 time");
    group.bench_function("std::String::from", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = String::from(str_data);
            }
        })
    });

    group.bench_function("LocalStr::from_str", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = LocalStr::from(str_data);
            }
        })
    });

    group.bench_function("SharedStr::from_str", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = SharedStr::from(str_data);
            }
        })
    });

    group.bench_function("Local to Shared", |b| {
        let s = LocalStr::from(str_data);
        b.iter(|| {
            for _ in 0..1_000 {
                let _: SharedStr = SharedStr::from(s.as_str());
            }
        })
    });

    group.finish();
}

fn bench_comparison(c: &mut Criterion) {
    let str_data = "KR0123456789";
    let mut group = c.benchmark_group("String Comparison 1_000_000 time");
    let s1 = String::from(str_data);
    let s3 = LocalStr::from(str_data);
    let s4 = SharedStr::from(str_data);

    group.bench_function("std::String::eq", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = s1 == str_data;
            }
        })
    });

    group.bench_function("LocalStr::eq", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = s3 == str_data;
            }
        })
    });

    group.bench_function("SharedStr::eq", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = s4 == str_data;
            }
        })
    });

    group.finish();
}
criterion_group!(
    benches, 
    bench_comparison,
    bench_string_creation,
);
criterion_main!(benches);