use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const MESSAGE_COUNT: usize = 100_000;
const WARMUP_COUNT: usize = 2;

fn producer(tx: Vec<Sender<u64>>, warmup: bool) {
    let count = if warmup { WARMUP_COUNT } else { MESSAGE_COUNT };
    for i in 0..count as u64 {
        for sender in &tx {
            sender.send(i).unwrap();
        }
    }
    if !warmup {
        println!(
            "Producer: Finished sending {} messages to each consumer",
            count
        );
    }
}

fn consumer(rx: Receiver<u64>, id: usize, warmup: bool) {
    let count = if warmup { WARMUP_COUNT } else { MESSAGE_COUNT };
    for _ in 0..count {
        rx.recv().unwrap();
    }
    if !warmup {
        println!("Consumer {}: Received {} messages", id, count);
    }
}

fn run_test(num_consumers: usize) -> f64 {
    let core_ids = Arc::new(core_affinity::get_core_ids().expect("Failed to get core IDs"));
    let num_cores = core_ids.len();

    let (warmup_tx, warmup_rx): (Vec<_>, Vec<_>) = (0..num_consumers).map(|_| bounded(1)).unzip();

    let (tx, rx): (Vec<_>, Vec<_>) = (0..num_consumers).map(|_| bounded(1)).unzip();

    // Warmup phase
    let warmup_consumer_handles: Vec<_> = (0..num_consumers)
        .map(|i| {
            let rx = warmup_rx[i].clone();
            let core_ids = Arc::clone(&core_ids);
            thread::spawn(move || {
                core_affinity::set_for_current(core_ids[(i + 1) % num_cores]);
                consumer(rx, i, true);
            })
        })
        .collect();

    let warmup_producer_handle = {
        let core_ids = Arc::clone(&core_ids);
        thread::spawn(move || {
            core_affinity::set_for_current(core_ids[0]);
            producer(warmup_tx, true);
        })
    };

    for handle in warmup_consumer_handles {
        handle.join().unwrap();
    }
    warmup_producer_handle.join().unwrap();

    println!("Warmup phase completed");

    // Actual test
    let consumer_handles: Vec<_> = (0..num_consumers)
        .map(|i| {
            let rx = rx[i].clone();
            let core_ids = Arc::clone(&core_ids);
            thread::spawn(move || {
                core_affinity::set_for_current(core_ids[(i + 1) % num_cores]);
                consumer(rx, i, false);
            })
        })
        .collect();

    let start = Instant::now();

    let producer_handle = {
        let core_ids = Arc::clone(&core_ids);
        thread::spawn(move || {
            core_affinity::set_for_current(core_ids[0]);
            producer(tx, false);
        })
    };

    for handle in consumer_handles {
        handle.join().unwrap();
    }
    producer_handle.join().unwrap();

    let duration = start.elapsed();
    duration.as_secs_f64() * 1000.0 // Convert to milliseconds
}

fn main() {
    println!("Testing Crossbeam Channel Communication Time");
    println!("--------------------------------------------");
    println!("Messages per consumer: {}", MESSAGE_COUNT);
    println!("Warmup messages per consumer: {}", WARMUP_COUNT);

    for &num_consumers in &[1, 3, 5, 7, 9] {
        println!("\nRunning test with {} consumers", num_consumers);
        let time = run_test(num_consumers);
        println!("{} consumers: {:.2} ms", num_consumers, time);
    }
}
