use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::atomic::Ordering;
use trading_engine::data_updater::update_status::{UpdateStatus, UpdateStatus2};

// Include the UpdateStatus and UpdateStatus2 structs here
// (I'm assuming they're in the same file or module)

fn bench_update_status(c: &mut Criterion) {
    let mut group = c.benchmark_group("UpdateStatus");

    group.bench_function("UpdateStatus - get/set", |b| {
        let status = UpdateStatus::default();
        b.iter(|| {
            status.set_first_status(black_box(42), Ordering::SeqCst);
            status.set_second_status(black_box(24), Ordering::SeqCst);
            black_box(status.get_first_status(Ordering::SeqCst));
            black_box(status.get_second_status(Ordering::SeqCst));
        });
    });

    group.bench_function("UpdateStatus2 - get/set", |b| {
        let status = UpdateStatus2::default();
        b.iter(|| {
            status.set_first_status(black_box(42), Ordering::SeqCst);
            status.set_second_status(black_box(24), Ordering::SeqCst);
            black_box(status.get_first_status(Ordering::SeqCst));
            black_box(status.get_second_status(Ordering::SeqCst));
        });
    });

    group.finish();
}

fn bench_update_status_contended(c: &mut Criterion) {
    let mut group = c.benchmark_group("UpdateStatus Contended");

    group.bench_function("UpdateStatus - contended", |b| {
        let status = std::sync::Arc::new(UpdateStatus::default());
        b.iter(|| {
            std::thread::scope(|s| {
                let status = &status;
                s.spawn(|| {
                    for _ in 0..100_000 {
                        status.set_first_status(black_box(42), Ordering::SeqCst);
                        black_box(status.get_second_status(Ordering::SeqCst));
                    }
                });
                s.spawn(|| {
                    for _ in 0..100_000 {
                        status.set_second_status(black_box(24), Ordering::SeqCst);
                        black_box(status.get_first_status(Ordering::SeqCst));
                    }
                });
            });
        });
    });

    group.bench_function("UpdateStatus2 - contended", |b| {
        let status = std::sync::Arc::new(UpdateStatus2::default());
        b.iter(|| {
            std::thread::scope(|s| {
                let status = &status;
                s.spawn(|| {
                    for _ in 0..100_000 {
                        status.set_first_status(black_box(42), Ordering::SeqCst);
                        black_box(status.get_second_status(Ordering::SeqCst));
                    }
                });
                s.spawn(|| {
                    for _ in 0..100_000 {
                        status.set_second_status(black_box(24), Ordering::SeqCst);
                        black_box(status.get_first_status(Ordering::SeqCst));
                    }
                });
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches, 
    bench_update_status_contended,
    bench_update_status, 
);
criterion_main!(benches);