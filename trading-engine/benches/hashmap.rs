use std::collections::HashMap;
use hashbrown::HashMap as HashbrownHashMap;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_hashmap(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("hashmap");

    bgroup.bench_function("std::HashMap", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(i, i+1);
            }
        });
    });

    bgroup.bench_function("hashbrown::HashMap", |b| {
        b.iter(|| {
            let mut map = HashbrownHashMap::new();
            for i in 0..1000 {
                map.insert(i, i+1);
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

    bgroup.bench_function("std::HashMap", |b| {
        b.iter(|| {
            for i in 0..100_000 {
                map.get(&i);
            }
        });
    });

    let mut map = HashbrownHashMap::new();
    for i in 0..100_000 {
        map.insert(i, i+1);
    }

    bgroup.bench_function("hashbrown::HashMap", |b| {
        b.iter(|| {
            for i in 0..100_000 {
                map.get(&i);
            }
        });
    });

    bgroup.finish();
}

criterion_group!(benches, bench_hashmap, bench_search_time);

criterion_main!(benches);