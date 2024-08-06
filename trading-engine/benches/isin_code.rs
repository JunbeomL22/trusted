use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use trading_engine::IsinCode;
use anyhow::Result;
use rustc_hash::FxHashMap;

// Direct byte array comparison
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct DirectIsin([u8; 12]);

impl DirectIsin {
    fn new(bytes: &[u8; 12]) -> Self {
        DirectIsin(*bytes)
    }
}

fn benchmark_comparisons(c: &mut Criterion) {
    let isin1 = IsinCode::new(b"US0378331005").expect("failed to create IsinCode");
    let isin2 = IsinCode::new(b"US0378331005").expect("failed to create IsinCode");
    let isin3 = IsinCode::new(b"GB0002374006").expect("failed to create IsinCode");

    let direct1 = DirectIsin::new(b"US0378331005");
    let direct2 = DirectIsin::new(b"US0378331005");
    let direct3 = DirectIsin::new(b"GB0002374006");

    let mut group = c.benchmark_group("ISIN Comparisons");
    //group.measurement_time(Duration::from_secs(10));

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

fn big_hashmap(c: &mut Criterion) {
    let mut isin_map = FxHashMap::default();
    let mut direct_isin_map = FxHashMap::default();
    let mut isin_vec = vec![];
    let mut direct_isin_vec = vec![];
    for i in 0..10_000 {
        let number_string = format!("{:04}", i);
        let number_length = number_string.len();
        let isin_string = format!("US{}{}", "0".repeat(10 - number_length), number_string);
        let isin = IsinCode::new(isin_string.as_bytes()).expect("ss");
        isin_vec.push(isin.clone());
        isin_map.insert(isin.clone(), i);
        let direct_isin_str = isin_string.clone();
        let direct_isin = DirectIsin::new(direct_isin_str.as_bytes().try_into().unwrap());
        direct_isin_vec.push(direct_isin.clone());
        direct_isin_map.insert(direct_isin.clone(), i);

    }

    let mut group = c.benchmark_group(format!("Big HashMap: {} entries", isin_map.len()));
    group.bench_function("IsinCode - Big HashMap", |b| {
        b.iter(|| {
            for isin in &isin_vec {
                black_box(isin_map.get(isin));
            }
        })
    });

    group.bench_function("DirectIsin - Big HashMap", |b| {
        b.iter(|| {
            for direct_isin in &direct_isin_vec {
                black_box(direct_isin_map.get(direct_isin));
            }
        })
    });

    group.finish();

}

criterion_group!(
    benches, 
    big_hashmap,
    benchmark_comparisons,
);
criterion_main!(benches);