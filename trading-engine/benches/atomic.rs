use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_atomics() -> Arc<[AtomicU32; 6]> {
    Arc::new([
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ])
}

fn load_three_atomics(atomics: &[AtomicU32; 6]) {
    let _ = black_box(atomics[0].load(Ordering::Relaxed));
    let _ = black_box(atomics[1].load(Ordering::Relaxed));
    let _ = black_box(atomics[2].load(Ordering::Relaxed));
    let _ = black_box(atomics[3].load(Ordering::Relaxed));
    let _ = black_box(atomics[4].load(Ordering::Relaxed));
    let _ = black_box(atomics[5].load(Ordering::Relaxed));
}

fn multi_threaded_load(atomics: Arc<[AtomicU32; 6]>) {
    let mut handles = vec![];
    for _ in 0..8 {
        let atomics_clone = Arc::clone(&atomics);
        let handle = thread::spawn(move || {
            for _ in 0..1_000_000 {
                load_three_atomics(&atomics_clone);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("8 threads load 6 AtomicU32 load 1_000_000 times", |b| {
        b.iter(|| {
            let atomics = create_atomics();
            multi_threaded_load(atomics);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);