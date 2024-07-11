use std::collections::HashMap;
use rustc_hash::FxHashMap;  

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_hashmap(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("hashmap");

    let mut map = HashMap::new();
    let mut fx_map = FxHashMap::default();

    bgroup.bench_function("std::HashMap 1000 insertion", |b| {
        b.iter(|| {
            
            for i in 0..1000 {
                map.insert(i, i+1);
            }
        });
    });

    bgroup.bench_function("rustc_hash::FxHashMap 1000 insertion", |b| {
        b.iter(|| {
            for i in 0..1000 {
                fx_map.insert(i, i+1);
            }
        });
    });

    bgroup.finish();

}

fn bench_search_time(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("search_time");

    let mut map = HashMap::new();
    for i in 0..100_000 {
        map.insert(i, i+1);
    }

    bgroup.bench_function("std::HashMap search after 1_000_000 insertion", |b| {
        b.iter(|| {
            for i in 0..1_000_000 {
                map.get(&i);
            }
        });
    });

    let mut map = FxHashMap::default();
    for i in 0..1_000_000 {
        map.insert(i, i+1);
    }

    bgroup.bench_function("rustc_hash::FxHashMap search after 1_000_000 insertion", |b| {
        b.iter(|| {
            for i in 0..1_000_000 {
                map.get(&i);
            }
        });
    });

    bgroup.finish();
}

criterion_group!(
    benches, 
    bench_search_time,
    bench_hashmap, 
);

criterion_main!(benches);