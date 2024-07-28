use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam_utils::CachePadded;
use parking_lot::Mutex as ParkingLotMutex;
use std::sync::{Arc, Mutex};
use std::thread;
use trading_engine::utils::counter::CounterU64;

fn bench_counter_u64_access_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("CounterU64");
    let thread_numbers = vec![1, 3, 5, 7, 9];
    let access_numbers = vec![100_000];

    let counter = Arc::new(CounterU64::new(0, 100_000));

    group.sample_size(20);
    for thread_number in &thread_numbers {
        for access_number in &access_numbers {
            let access_number = *access_number; // Dereference `access_number` here
            group.bench_function(
                format!("{} threads, {} accesses", thread_number, access_number),
                |b| {
                    b.iter(|| {
                        let counter_clone = counter.clone(); // Clone `counter` here
                        let handles: Vec<_> = (0..*thread_number)
                            .map(|_| {
                                let counter_clone = counter_clone.clone();
                                thread::spawn(move || {
                                    for _ in 0..access_number {
                                        // Dereference `access_number` here
                                        black_box(&counter_clone.next());
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
#[derive(Clone)]
pub struct CachePaddedA {
    number: CachePadded<u64>,
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

    group.sample_size(20);
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
                                    for _ in 0..access_number {
                                        // Dereference `access_number` here
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

fn bench_arc_parking_lot_mutex_access_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc<parking_lot::Mutex<A>>");
    let thread_numbers = vec![1, 3, 5, 7, 9];
    let access_numbers = vec![100_000];

    let a = A { number: 42 };

    group.sample_size(20);
    for thread_number in &thread_numbers {
        for access_number in &access_numbers {
            let access_number = *access_number; // Dereference `access_number` here
            group.bench_function(
                format!("{} threads, {} accesses", thread_number, access_number),
                |b| {
                    b.iter(|| {
                        let a_clone = a.clone(); // Clone `a` here
                        let shared_a = Arc::new(ParkingLotMutex::new(a_clone));
                        let handles: Vec<_> = (0..*thread_number)
                            .map(|_| {
                                let a_clone = Arc::clone(&shared_a);
                                thread::spawn(move || {
                                    for _ in 0..access_number {
                                        // Dereference `access_number` here
                                        let locked_a = a_clone.lock();
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
    bench_counter_u64_access_group,
    //bench_arc_parking_lot_mutex_access_group,
    //bench_arc_mutex_access_group
);
criterion_main!(benches);
