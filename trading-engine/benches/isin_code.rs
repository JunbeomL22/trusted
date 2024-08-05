use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use trading_engine::IsinCode;

// Direct byte array comparison
#[derive(Copy, Clone, PartialEq, Eq)]
struct DirectIsin([u8; 12]);

impl DirectIsin {
    fn new(bytes: &[u8; 12]) -> Self {
        DirectIsin(*bytes)
    }
}

fn benchmark_comparisons(c: &mut Criterion) {
    let isin1 = IsinCode::new(b"US0378331005");
    let isin2 = IsinCode::new(b"US0378331005");
    let isin3 = IsinCode::new(b"GB0002374006");

    let direct1 = DirectIsin::new(b"US0378331005");
    let direct2 = DirectIsin::new(b"US0378331005");
    let direct3 = DirectIsin::new(b"GB0002374006");

    let mut group = c.benchmark_group("ISIN Comparisons");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("IsinCode - Same", |b| {
        b.iter(|| black_box(isin1 == isin2))
    });

    group.bench_function("IsinCode - Different", |b| {
        b.iter(|| black_box(isin1 == isin3))
    });

    group.bench_function("DirectIsin - Same", |b| {
        b.iter(|| black_box(direct1 == direct2))
    });

    group.bench_function("DirectIsin - Different", |b| {
        b.iter(|| black_box(direct1 == direct3))
    });

    group.finish();
}

criterion_group!(benches, benchmark_comparisons);
criterion_main!(benches);