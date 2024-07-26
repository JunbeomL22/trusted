use rustc_hash::FxHashMap;
use std::collections::HashMap;
use criterion::{criterion_group, criterion_main, Criterion, black_box};

use std::collections::BTreeMap;

fn find_two_smallest_fxhashmap(map: &FxHashMap<u64, u64>) -> (u64, u64) {
    let mut smallest = u64::MAX;
    let mut second_smallest = u64::MAX;
    
    for &key in map.keys() {
        if key < smallest {
            second_smallest = smallest;
            smallest = key;
        } else if key < second_smallest && key != smallest {
            second_smallest = key;
        }
    }
    
    (smallest, second_smallest)
}

fn find_two_smallest_btreemap(map: &BTreeMap<u64, u64>) -> (u64, u64) {
    let mut iter = map.keys();
    let smallest = *iter.next().unwrap_or(&u64::MAX);
    let second_smallest = *iter.next().unwrap_or(&u64::MAX);
    (smallest, second_smallest)
}

fn bench_find_minimal_elements(c: &mut Criterion) {
    let mut fxhash_map = FxHashMap::default();
    let mut btree_map = BTreeMap::new();

    let mut group = c.benchmark_group("find_two_smallest");
    
    let n = 100;
    for i in 0..n {
        fxhash_map.insert(i as u64, i as u64);
        btree_map.insert(i as u64, i as u64);
    }

    group.bench_function(format!("FxHashMap find two smallest where {} elements", n).as_str(), |b| {
        b.iter(|| find_two_smallest_fxhashmap(black_box(&fxhash_map)))
    });

    group.bench_function(format!("BTreeMap find two smallest where {} elements", n).as_str(), |b| {
        b.iter(|| find_two_smallest_btreemap(black_box(&btree_map)))
    });

    fxhash_map.clear();
    btree_map.clear();
    let n = 1_000;
    for i in 0..n {
        fxhash_map.insert(i as u64, i as u64);
        btree_map.insert(i as u64, i as u64);
    }

    group.bench_function(format!("FxHashMap find two smallest where {} elements", n).as_str(), |b| {
        b.iter(|| find_two_smallest_fxhashmap(black_box(&fxhash_map)))
    });

    group.bench_function(format!("BTreeMap find two smallest where {} elements", n).as_str(), |b| {
        b.iter(|| find_two_smallest_btreemap(black_box(&btree_map)))
    });

    fxhash_map.clear();
    btree_map.clear();
    let n = 10_000;
    for i in 0..n {
        fxhash_map.insert(i as u64, i as u64);
        btree_map.insert(i as u64, i as u64);
    }

    group.bench_function(format!("FxHashMap find two smallest where {} elements", n).as_str(), |b| {
        b.iter(|| find_two_smallest_fxhashmap(black_box(&fxhash_map)))
    });

    group.bench_function(format!("BTreeMap find two smallest where {} elements", n).as_str(), |b| {
        b.iter(|| find_two_smallest_btreemap(black_box(&btree_map)))
    });

    group.finish();

}

fn bench_hashmap(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("hashmap");

    let mut map = HashMap::new();
    let mut fx_map = FxHashMap::default();

    bgroup.bench_function("std::HashMap 1000 insertion", |b| {
        b.iter(|| {
            for i in 0..1000 {
                map.insert(i, i + 1);
            }
        });
    });

    bgroup.bench_function("rustc_hash::FxHashMap 1000 insertion", |b| {
        b.iter(|| {
            for i in 0..1000 {
                fx_map.insert(i, i + 1);
            }
        });
    });

    bgroup.finish();
}

fn bench_search_time(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("search_time");

    let mut map = HashMap::new();
    for i in 0..100_000 {
        map.insert(i, i + 1);
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
        map.insert(i, i + 1);
    }

    bgroup.bench_function(
        "rustc_hash::FxHashMap search after 1_000_000 insertion",
        |b| {
            b.iter(|| {
                for i in 0..1_000_000 {
                    map.get(&i);
                }
            });
        },
    );

    bgroup.finish();
}


fn search_fxhashmap(map: &FxHashMap<u64, u64>, key: u64) -> Option<&u64> {
    map.get(&key)
}

fn search_btreemap(map: &BTreeMap<u64, u64>, key: u64) -> Option<&u64> {
    map.get(&key)
}

fn remove_fxhashmap(map: &mut FxHashMap<u64, u64>, key: u64) -> Option<u64> {
    map.remove(&key)
}

fn remove_btreemap(map: &mut BTreeMap<u64, u64>, key: u64) -> Option<u64> {
    map.remove(&key)
}

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("Search");

    for &size in &[100, 1_000] {
        let key_to_search = size / 2;

        let fxhash_map: FxHashMap<_, _> = (0..size).map(|i| (i, i)).collect();
        let btree_map: BTreeMap<_, _> = (0..size).map(|i| (i, i)).collect();

        group.bench_function(&format!("FxHashMap search {} elements", size), |b| {
            b.iter(|| search_fxhashmap(black_box(&fxhash_map), key_to_search))
        });

        group.bench_function(&format!("BTreeMap search {} elements", size), |b| {
            b.iter(|| search_btreemap(black_box(&btree_map), key_to_search))
        });
    }

    group.finish();
}

fn bench_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove");

    for &size in &[100, 1_000] {
        let key_to_remove = size / 2;

        group.bench_function(&format!("FxHashMap remove {} elements", size), |b| {
            b.iter_with_setup(
                || (0..size).map(|i| (i, i)).collect::<FxHashMap<_, _>>(),
                |mut map| remove_fxhashmap(black_box(&mut map), key_to_remove)
            )
        });

        group.bench_function(&format!("BTreeMap remove {} elements", size), |b| {
            b.iter_with_setup(
                || (0..size).map(|i| (i, i)).collect::<BTreeMap<_, _>>(),
                |mut map| remove_btreemap(black_box(&mut map), key_to_remove)
            )
        });
    }

    group.finish();
}

criterion_group!(
    benches, 
    bench_find_minimal_elements,
    bench_search,
    bench_remove,
    //bench_search_time, 
    //bench_hashmap,
);

criterion_main!(benches);
