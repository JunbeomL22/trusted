use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use ustr::Ustr;
use core_affinity::{self, CoreId};
use once_cell::sync::Lazy;

#[derive(Clone)]
struct A {
    number: u64,
    string: Ustr,
}

pub static CORE_IDS: Lazy<Vec<CoreId>> = Lazy::new(|| {
    core_affinity::get_core_ids().expect("Failed to get core IDs")
});

fn setup_channel() -> (Sender<A>, Receiver<A>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[5];

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn bench_channel_roundtrip(c: &mut Criterion) {
    let main_core_id = CORE_IDS[4];

    let trip_numbers = vec![1, 10, 100, 1000, 10000];
    let mut group = c.benchmark_group("channel_roundtrip");

    for trip in trip_numbers {
        group.bench_function(format!("channel {} roundtrip", trip), |b| {
            let (tx, rx, handle) = setup_channel();
    
            let msg = A {
                number: 42,
                string: Ustr::from("Hello, World!"),
            };
            core_affinity::set_for_current(main_core_id);
            b.iter(|| {
                for _ in 0..trip {
                    tx.send(msg.clone()).unwrap();
                    
                    while let Ok(msg) = rx.recv() {
                        black_box(msg);
                        break;
                    }
                }
            });
    
            drop(tx);
            handle.join().unwrap();
        });   
    }
}

criterion_group!(benches, bench_channel_roundtrip);
criterion_main!(benches);