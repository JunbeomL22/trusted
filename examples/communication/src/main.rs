use crossbeam_utils::CachePadded;
use std::sync::mpsc::{
    channel as std_channel,
    Sender as StdSender,
    Receiver as StdReceiver,
};
use crossbeam_channel::{
    unbounded as crossbeam_unbounded,
    Sender as CrossbeamSender,
    Receiver as CrossbeamReceiver,
};
use kanal::{
    unbounded as kanal_unbounded,
    Sender as KanalSender,
    Receiver as KanalReceiver,
};
use flume::{
    unbounded as flume_unbounded,
    Sender as FlumeSender,
    Receiver as FlumeReceiver,
};
use std::thread;
use ustr::Ustr;
use core_affinity::{self, CoreId};
use once_cell::sync::Lazy;
use trading_engine::utils::timer::get_unix_nano;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub static CORE_IDS: Lazy<Vec<CoreId>> = Lazy::new(|| {
    core_affinity::get_core_ids().expect("Failed to get core IDs")
});

const DATA_SIZE: usize = 1000;

#[derive(Clone, Debug)]
struct A {
    number: Vec<u64>,
}

fn setup_channel_std(core_id: usize) -> (StdSender<A>, StdReceiver<A>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[core_id];

    //let (tx1, rx1) = unbounded();
    //let (tx2, rx2) = unbounded();
    let (tx1, rx1) = std_channel();
    let (tx2, rx2) = std_channel();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn setup_channel_crossbeam(
    core_id: usize,
) -> (CrossbeamSender<A>, CrossbeamReceiver<A>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[core_id];

    let (tx1, rx1) = crossbeam_unbounded();
    let (tx2, rx2) = crossbeam_unbounded();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn setup_channel_kanal(
    core_id: usize,
) -> (KanalSender<A>, KanalReceiver<A>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[core_id];

    let (tx1, rx1) = kanal_unbounded();
    let (tx2, rx2) = kanal_unbounded();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn round_trip_std(
    channel_core_id: usize,
    core_id: usize,
    trip_number: usize, 
) -> Result<()> {
    let main_core_id = CORE_IDS[core_id];
    core_affinity::set_for_current(main_core_id);

    let (tx, rx, handle) = setup_channel_std( channel_core_id );

    let a = A {
        number: vec![42; DATA_SIZE],
    };

    let a_vec = (0..trip_number).map(|_| a.clone()).collect::<Vec<A>>();

    for _ in 0..10 {
        tx.send(a.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            break;
        }
    }

    let start = get_unix_nano();
    let mut count = 0;
    for e in a_vec.iter() {
        tx.send(e.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            //a = msg;
            count += 1;
            break;
        }
    }
    assert!(count == trip_number);

    println!(
        "(std) cores = ({}, {}) average round for {} trip time: {:.0} ns", 
        channel_core_id,
        core_id,
        trip_number,
        (get_unix_nano() - start) / trip_number as u64,
    );

    drop(tx);
    handle.join().unwrap();
    Ok(())
}

fn round_trip_crossbeam(
    channel_core_id: usize, 
    core_id: usize,
    trip_number: usize, 
) -> Result<()> {
    let main_core_id = CORE_IDS[core_id];
    core_affinity::set_for_current(main_core_id);

    let (tx, rx, handle) = setup_channel_crossbeam( channel_core_id );

    let a = A {
        number: vec![42; DATA_SIZE],
    };

    let a_vec = (0..trip_number).map(|_| a.clone()).collect::<Vec<A>>();

    for _ in 0..10 {
        tx.send(a.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            break;
        }
    }
    
    let mut count = 0;
    let start = get_unix_nano();
    for e in a_vec.iter() {
        tx.send(e.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            //a = msg;
            count += 1;
            break;
        }
    }
    assert!(count == trip_number);

    let end = get_unix_nano();
    let ave = (end - start) / trip_number as u64;
    println!(
        "(crossbeam) cores = ({}, {}) average round for {} trip time: {:.0} ns", 
        channel_core_id,
        core_id,
        trip_number,
        ave,
    );

    drop(tx);
    handle.join().unwrap();
    Ok(())
}


fn setup_channel_crossbeam_cachepadded(
    core_id: usize,
) -> (CrossbeamSender<CachePadded<A>>, CrossbeamReceiver<CachePadded<A>>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[core_id];

    let (tx1, rx1) = crossbeam_unbounded();
    let (tx2, rx2) = crossbeam_unbounded();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn round_trip_with_cache_padded(
    channel_core_id: usize, 
    core_id: usize,
    trip_number: usize, 
) -> Result<()> {
    let main_core_id = CORE_IDS[core_id];
    core_affinity::set_for_current(main_core_id);

    let (tx, rx, handle) = setup_channel_crossbeam_cachepadded( channel_core_id );

    let a = CachePadded::new(A {
        number: vec![42; DATA_SIZE],
    });

    let a_vec = (0..trip_number).map(|_| a.clone()).collect::<Vec<CachePadded<A>>>();

    for _ in 0..10 {
        tx.send(a.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            break;
        }
    }

    let start = get_unix_nano();
    let mut count = 0;
    for e in a_vec.iter() {
        tx.send(e.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            //a = msg;
            count += 1;
            break;
        }
    }
    assert!(count == trip_number);

    let end = get_unix_nano();
    let ave = (end - start) / trip_number as u64;

    println!(
        "(crossbeam_padded) cores = ({}, {}) average round for {} trip time: {:.0} ns", 
        channel_core_id,
        core_id,
        trip_number,
        ave,
    );

    drop(tx);
    handle.join().unwrap();
    Ok(())
}

fn round_trip_kanal(
    channel_core_id: usize, 
    core_id: usize,
    trip_number: usize, 
) -> Result<()> {
    let main_core_id = CORE_IDS[core_id];
    core_affinity::set_for_current(main_core_id);

    let (tx, rx, handle) = setup_channel_kanal( channel_core_id );

    let a = A {
        number: vec![42; DATA_SIZE],
    };

    let a_vec = (0..trip_number).map(|_| a.clone()).collect::<Vec<A>>();

    for _ in 0..10 {
        tx.send(a.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            break;
        }
    }

    let start = get_unix_nano();
    let mut count = 0;
    for e in a_vec.iter() {
        tx.send(e.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            //a = msg;
            count += 1;
            break;
        }
    }
    assert!(count == trip_number);
    let end = get_unix_nano();
    let ave = (end - start) / trip_number as u64;

    println!(
        "(kanal) cores = ({}, {}) average round for {} trip time: {:.0} ns", 
        channel_core_id,
        core_id,
        trip_number,
        ave,
    );

    drop(tx);
    handle.join().unwrap();
    Ok(())
}

fn setup_channel_flume(
    core_id: usize,
) -> (FlumeSender<A>, FlumeReceiver<A>, thread::JoinHandle<()>) {
    let worker_core_id = CORE_IDS[core_id];

    let (tx1, rx1) = flume_unbounded();
    let (tx2, rx2) = flume_unbounded();

    let handle = thread::spawn(move || {
        core_affinity::set_for_current(worker_core_id);
        while let Ok(msg) = rx1.recv() {
            tx2.send(msg).unwrap();
        }
    });

    (tx1, rx2, handle)
}

fn round_trip_flume(
    channel_core_id: usize, 
    core_id: usize,
    trip_number: usize, 
) -> Result<()> {
    let main_core_id = CORE_IDS[core_id];
    core_affinity::set_for_current(main_core_id);

    let (tx, rx, handle) = setup_channel_flume( channel_core_id );

    let a = A {
        number: vec![42; DATA_SIZE],
    };

    let a_vec = (0..trip_number).map(|_| a.clone()).collect::<Vec<A>>();

    for _ in 0..10 {
        tx.send(a.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            break;
        }
    }

    let start = get_unix_nano();
    let mut count = 0;
    for e in a_vec.iter() {
        tx.send(e.clone()).unwrap();
        while let Ok(_) = rx.recv() {
            //a = msg;
            count += 1;
            break;
        }
    }
    assert!(count == trip_number);
    let end = get_unix_nano();
    let ave = (end - start) / trip_number as u64;

    println!(
        "(flume) cores = ({}, {}) average round for {} trip time: {:.0} ns", 
        channel_core_id,
        core_id,
        trip_number,
        ave,
    );

    drop(tx);
    handle.join().unwrap();
    Ok(())
}

fn main() -> Result<()> {
    let trip_numbers = vec![1_000,];
    let core_pairs = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 2),
        (2, 3),
        (3, 3),
    ];

    for trip in trip_numbers.clone() {
        for (channel_core_id, core_id) in core_pairs.iter() {
            round_trip_std(*channel_core_id, *core_id, trip)?;
        }
    }

    for trip in trip_numbers.clone() {
        for (channel_core_id, core_id) in core_pairs.iter() {
            round_trip_crossbeam(*channel_core_id, *core_id, trip)?;
        }
    }

    for trip in trip_numbers.clone() {
        for (channel_core_id, core_id) in core_pairs.iter() {
            round_trip_with_cache_padded(*channel_core_id, *core_id, trip)?;
        }
    }

    for trip in trip_numbers.clone() {
        for (channel_core_id, core_id) in core_pairs.iter() {
            round_trip_kanal(*channel_core_id, *core_id, trip)?;
        }
    }

    for trip in trip_numbers.clone() {
        for (channel_core_id, core_id) in core_pairs.iter() {
            round_trip_flume(*channel_core_id, *core_id, trip)?;
        }
    }

    Ok(())
}
