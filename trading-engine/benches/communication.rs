use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use ustr::Ustr;

#[derive(Clone)]
struct A {
    number: u64,
    string: Ustr,
}

const TOTAL_MESSAGES: usize = 1_000_000;

fn create_message(i: u64) -> A {
    A {
        number: i,
        string: Ustr::from("hello"),
    }
}

fn bench_channel_grouping(c: &mut Criterion) {
    let mut group = c.benchmark_group("Channel Grouping");
    group.sample_size(40);

    for &group_size in &[1, 100_000, 1_000_000, 10_000_000] {
        group.bench_function(format!("group_size_{}", group_size), |b| {
            b.iter(|| {
                let (tx, rx) = mpsc::channel();

                let sender = thread::spawn(move || {
                    for i in 0..(TOTAL_MESSAGES / group_size) {
                        let messages: Vec<A> = (0..group_size)
                            .map(|j| create_message((i * group_size + j) as u64))
                            .collect();
                        tx.send(messages).unwrap();
                    }
                });

                let receiver = thread::spawn(move || {
                    let mut count = 0;
                    while count < TOTAL_MESSAGES {
                        let messages = rx.recv().unwrap();
                        count += messages.len();
                        // Perform some operation on the received data to ensure it's not optimized away
                        for msg in messages {
                            black_box(&msg.number);
                            black_box(&msg.string);
                        }
                    }
                    assert_eq!(count, TOTAL_MESSAGES);
                });

                sender.join().unwrap();
                receiver.join().unwrap();
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_channel_grouping);
criterion_main!(benches);