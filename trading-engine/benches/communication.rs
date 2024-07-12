use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use std::sync::mpsc::{channel, Sender, Receiver};
use crossbeam_channel::{unbounded, Receiver, Sender};
//use kanal::{unbounded, Sender, Receiver};
use core_affinity::{self, CoreId};
use once_cell::sync::Lazy;
use std::thread;
use ustr::Ustr;

#[derive(Clone)]
struct A {
    number: u64,
    string: Ustr,
}

pub static CORE_IDS: Lazy<Vec<CoreId>> =
    Lazy::new(|| core_affinity::get_core_ids().expect("Failed to get core IDs"));

fn setup_channel() -> (Sender<A>, Receiver<A>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[0];

    let (tx1, rx1) = unbounded();
    let (tx2, rx2) = unbounded();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn bench_channel_roundtrip(c: &mut Criterion) {
    let main_core_id = CORE_IDS[1];

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

fn bench_one_way_trip(c: &mut Criterion) {
    let (tx, rx) = unbounded();
    let msg = A {
        number: 42,
        string: Ustr::from("Hello, World!"),
    };

    c.bench_function("one_way_trip", |b| {
        b.iter(|| {
            tx.send(msg.clone()).unwrap();
            while let Ok(msg) = rx.recv() {
                black_box(msg);
                break;
            }
        });
    });
}

criterion_group!(benches, bench_one_way_trip, bench_channel_roundtrip);
criterion_main!(benches);
