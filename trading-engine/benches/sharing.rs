use std::sync::{Arc, Mutex};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::thread;
use ustr::Ustr;
use crossbeam_utils::CachePadded;

#[derive(Clone)]
pub struct CachePaddedA {
    number: CachePadded<u64>,
}

fn bench_arc_mutex_cache_padded_access_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc<Mutex<CachePadded<A>>");
    let thread_numbers = vec![1, 3, 5, 7, 9];
    let access_numbers = vec![100_000];

    let a = CachePaddedA {
        number: CachePadded::new(42),
    };

    group.sample_size(50);
    for thread_number in &thread_numbers {
        for access_number in &access_numbers {
            let access_number = *access_number; // Dereference `access_number` here
            group.bench_function(
                format!("{} threads, {} accesses", thread_number, access_number),
                |b| {
                    b.iter(|| {
                        let a_clone = a.clone(); // Clone `a` here
                        let shared_a = Arc::new(Mutex::new(a_clone));
                        let handles: Vec<_> = (0..*thread_number)
                            .map(|_| {
                                let a_clone = Arc::clone(&shared_a);
                                thread::spawn(move || {
                                    for _ in 0..access_number { // Dereference `access_number` here
                                        let locked_a = a_clone.lock().unwrap();
                                        black_box(&locked_a.number);
                                    }
                                })
                            })
                            .collect();

                        for handle in handles {
                            handle.join().unwrap();
                        }
                    })
                },
            );
        }
    }
    group.finish();
}


// Define struct A
#[derive(Clone)]
struct A {
    number: u64,
}


fn bench_arc_mutex_access_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc<Mutex<A>>");
    let thread_numbers = vec![1, 3, 5, 7, 9];
    let access_numbers = vec![100_000];

    let a = A {
        number: 42,
        //string: Ustr::from("Hello, World!"),
    };

    group.sample_size(50);
    for thread_number in &thread_numbers {
        for access_number in &access_numbers {
            let access_number = *access_number; // Dereference `access_number` here
            group.bench_function(
                format!("{} threads, {} accesses", thread_number, access_number),
                |b| {
                    b.iter(|| {
                        let a_clone = a.clone(); // Clone `a` here
                        let shared_a = Arc::new(Mutex::new(a_clone));
                        let handles: Vec<_> = (0..*thread_number)
                            .map(|_| {
                                let a_clone = Arc::clone(&shared_a);
                                thread::spawn(move || {
                                    for _ in 0..access_number { // Dereference `access_number` here
                                        let locked_a = a_clone.lock().unwrap();
                                        black_box(&locked_a.number);
                                        //black_box(&locked_a.string);
                                    }
                                })
                            })
                            .collect();

                        for handle in handles {
                            handle.join().unwrap();
                        }
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(
    benches, 
    //bench_arc_mutex_cache_padded_access_group,
    bench_arc_mutex_access_group);
criterion_main!(benches);